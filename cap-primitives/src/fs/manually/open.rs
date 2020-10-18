//! Manual path resolution, one component at a time, with manual symlink
//! resolution, in order to enforce sandboxing.

use super::{readlink_one, CanonicalPath, CowComponent};
#[cfg(not(feature = "no_racy_asserts"))]
use crate::fs::is_same_file;
use crate::fs::{
    dir_options, errors, open_unchecked, path_requires_dir, stat_unchecked, FollowSymlinks,
    MaybeOwnedFile, Metadata, OpenOptions, OpenUncheckedError,
};
use std::{
    ffi::OsStr,
    fs, io,
    path::{Component, Path, PathBuf},
};

/// Implement `open` by breaking up the path into components, resolving each
/// component individually, and resolving symbolic links manually.
pub(crate) fn open(start: &fs::File, path: &Path, options: &OpenOptions) -> io::Result<fs::File> {
    let mut symlink_count = 0;
    let start = MaybeOwnedFile::borrowed(start);
    let maybe_owned = internal_open(start, path, options, &mut symlink_count, None)?;
    maybe_owned.into_file(options)
}

/// Context for performing manual component-at-a-time path resolution.
struct Context<'start> {
    /// The current base directory handle for path lookups.
    base: MaybeOwnedFile<'start>,

    /// The stack of directory handles below the base.
    dirs: Vec<MaybeOwnedFile<'start>>,

    /// The current worklist stack of path components to process.
    components: Vec<CowComponent<'start>>,

    /// If requested, the canonical path is constructed here.
    canonical_path: CanonicalPath<'start>,

    /// Does the path end in `/` or similar, so it requires a directory?
    dir_required: bool,

    /// Are we requesting write permissions, so we can't open a directory?
    dir_precluded: bool,

    #[cfg(not(feature = "no_racy_asserts"))]
    start_clone: MaybeOwnedFile<'start>,
}

impl<'start> Context<'start> {
    /// Construct a new instance of `Self`.
    fn new(
        start: MaybeOwnedFile<'start>,
        path: &'start Path,
        options: &OpenOptions,
        canonical_path: Option<&'start mut PathBuf>,
    ) -> Self {
        let components = path
            .components()
            .rev()
            .map(CowComponent::borrowed)
            .collect::<Vec<_>>();

        #[cfg(not(feature = "no_racy_asserts"))]
        let start_clone = MaybeOwnedFile::owned(start.try_clone().unwrap());

        Self {
            base: start,
            dirs: Vec::with_capacity(components.len()),
            components,
            canonical_path: CanonicalPath::new(canonical_path),
            dir_required: path_requires_dir(path),
            dir_precluded: options.write || options.append,

            #[cfg(not(feature = "no_racy_asserts"))]
            start_clone,
        }
    }

    /// Handle a "." path component.
    fn cur_dir(&mut self) -> io::Result<()> {
        // If the path ends in `.` and we can't open a directory, fail.
        if self.components.is_empty() {
            if self.dir_precluded {
                return Err(errors::is_directory());
            }
            // TODO: This metdata call is unneeded in a common case of calling
            // `read_dir` on `.`.
            if !self.base.metadata()?.is_dir() {
                return Err(errors::is_not_directory());
            }
            self.canonical_path.push(Component::CurDir.as_os_str());
        }

        // Otherwise do nothing.
        Ok(())
    }

    /// Handle a ".." path component.
    fn parent_dir(&mut self) -> io::Result<()> {
        #[cfg(not(feature = "no_racy_asserts"))]
        assert!(self.dirs.is_empty() || !is_same_file(&self.start_clone, &self.base)?);

        if self.components.is_empty() && self.dir_precluded {
            return Err(errors::is_directory());
        }

        // We hold onto all the parent directory descriptors so that we
        // don't have to re-open anything when we encounter a `..`. This
        // way, even if the directory is concurrently moved, we don't have
        // to worry about `..` leaving the sandbox.
        match self.dirs.pop() {
            Some(dir) => self.base = dir,
            None => return Err(errors::escape_attempt()),
        }
        assert!(self.canonical_path.pop());

        Ok(())
    }

    /// Handle a "normal" path component.
    fn normal(
        &mut self,
        one: &OsStr,
        options: &OpenOptions,
        symlink_count: &mut u8,
    ) -> io::Result<()> {
        // If the path requires a directory and we can't open a directory, fail.
        if self.components.is_empty() && self.dir_required && self.dir_precluded {
            return Err(errors::is_directory());
        }

        // Otherwise we're doing an open.
        let mut use_options = if self.components.is_empty() {
            options.clone()
        } else {
            dir_options()
        };
        let dir_required = self.dir_required || use_options.dir_required;
        match open_unchecked(
            &self.base,
            one.as_ref(),
            use_options
                .follow(FollowSymlinks::No)
                .dir_required(dir_required),
        ) {
            Ok(file) => {
                // Emulate `O_PATH` + `FollowSymlinks::Yes` on Linux. If `file` is a
                // symlink, follow it.
                #[cfg(target_os = "linux")]
                if should_emulate_o_path(&use_options) {
                    match readlink_one(&file, Default::default(), symlink_count) {
                        Ok(destination) => {
                            self.components
                                .extend(destination.components().rev().map(CowComponent::owned));
                            self.dir_required |= path_requires_dir(&destination);
                            return Ok(());
                        }
                        // If it isn't a symlink, handle it as normal. `readlinkat` returns
                        // `ENOENT` if the file isn't a symlink in this situation.
                        Err(err) if err.kind() == io::ErrorKind::NotFound => (),
                        // If `readlinkat` fails any other way, pass it on.
                        Err(err) => return Err(err),
                    }
                }

                // Normal case
                let prev_base = self.base.descend_to(MaybeOwnedFile::owned(file));
                self.dirs.push(prev_base);
                self.canonical_path.push(one);
                Ok(())
            }
            Err(OpenUncheckedError::Symlink(err)) => {
                if options.follow == FollowSymlinks::No && self.components.is_empty() {
                    self.canonical_path.push(one);
                    self.canonical_path.complete();
                    return Err(err);
                }
                self.symlink(one, symlink_count)
            }
            Err(OpenUncheckedError::NotFound(err)) => Err(err),
            Err(OpenUncheckedError::Other(err)) => {
                // An error occurred. If this was the last component, and the error wasn't
                // due to invalid inputs (eg. the path has an embedded NUL), record it as
                // the last component of the canonical path, even if we couldn't open it.
                if self.components.is_empty() && err.kind() != io::ErrorKind::InvalidInput {
                    self.canonical_path.push(one);
                    self.canonical_path.complete();
                }
                Err(err)
            }
        }
    }

    /// Dereference one symlink level.
    fn symlink(&mut self, one: &OsStr, symlink_count: &mut u8) -> io::Result<()> {
        let destination = readlink_one(&self.base, one, symlink_count)?;
        self.components
            .extend(destination.components().rev().map(CowComponent::owned));
        self.dir_required |= path_requires_dir(&destination);
        Ok(())
    }
}

/// Internal implementation of manual `open`, exposing some additional
/// parameters.
///
/// Callers can request the canonical path by passing `Some` to
/// `canonical_path`. If the complete canonical path is processed, it will be
/// stored in the provided `&mut PathBuf`, even if the actual open fails. If
/// a failure occurs before the complete canonical path is processed, the
/// provided `&mut PathBuf` is cleared to empty.
///
/// A note on lifetimes: `path` and `canonical_path` here don't strictly
/// need `'start`, but using them makes it easier to store them in the
/// `Context` struct.
pub(super) fn internal_open<'start>(
    start: MaybeOwnedFile<'start>,
    path: &'start Path,
    options: &OpenOptions,
    symlink_count: &mut u8,
    canonical_path: Option<&'start mut PathBuf>,
) -> io::Result<MaybeOwnedFile<'start>> {
    // POSIX returns `ENOENT` on an empty path. TODO: On Windows, we should
    // be compatible with what Windows does instead.
    if path.as_os_str().is_empty() {
        return Err(errors::no_such_file_or_directory());
    }

    let mut ctx = Context::new(start, path, options, canonical_path);

    while let Some(c) = ctx.components.pop() {
        match c {
            CowComponent::PrefixOrRootDir => return Err(errors::escape_attempt()),
            CowComponent::CurDir => ctx.cur_dir()?,
            CowComponent::ParentDir => ctx.parent_dir()?,
            CowComponent::Normal(one) => ctx.normal(&one, options, symlink_count)?,
        }
    }

    #[cfg(not(feature = "no_racy_asserts"))]
    check_internal_open(&ctx, path, options);

    ctx.canonical_path.complete();
    Ok(ctx.base)
}

/// Implement manual `stat` in a similar manner as manual `open`.
pub(crate) fn stat<'start>(
    start: &fs::File,
    path: &'start Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    // POSIX returns `ENOENT` on an empty path. TODO: On Windows, we should
    // be compatible with what Windows does instead.
    if path.as_os_str().is_empty() {
        return Err(errors::no_such_file_or_directory());
    }

    let mut options = OpenOptions::new();
    options.follow(follow);
    let mut symlink_count = 0;
    let mut ctx = Context::new(MaybeOwnedFile::borrowed(start), path, &options, None);
    assert!(!ctx.dir_precluded);

    while let Some(c) = ctx.components.pop() {
        match c {
            CowComponent::PrefixOrRootDir => return Err(errors::escape_attempt()),
            CowComponent::CurDir => ctx.cur_dir()?,
            CowComponent::ParentDir => ctx.parent_dir()?,
            CowComponent::Normal(one) => {
                if ctx.components.is_empty() {
                    // If this is the last component, do a non-following `stat_unchecked` on it.
                    let stat = stat_unchecked(&ctx.base, one.as_ref(), FollowSymlinks::No)?;

                    // If we weren't asked to follow symlinks, or it wasn't a symlink, we're done.
                    if options.follow == FollowSymlinks::No || !stat.file_type().is_symlink() {
                        if ctx.dir_required && !stat.is_dir() {
                            return Err(errors::is_not_directory());
                        }
                        return Ok(stat);
                    }

                    // If it was a symlink and we're asked to follow symlinks, dereference it.
                    ctx.symlink(&one, &mut symlink_count)?
                } else {
                    // Otherwise open the path component normally.
                    ctx.normal(&one, &options, &mut symlink_count)?
                }
            }
        }
    }

    // If the path ended in `.` or `..`, we already have it open, so just do
    // `.metadata()` on it.
    ctx.base.metadata().map(Metadata::from_std)
}

/// Test whether the given options imply that we should treat an open file as
/// potentially being a symlink we need to follow, due to use of `O_PATH`.
#[cfg(target_os = "linux")]
fn should_emulate_o_path(use_options: &OpenOptions) -> bool {
    (use_options.ext.custom_flags & libc::O_PATH) == libc::O_PATH
        && use_options.follow == FollowSymlinks::Yes
}

#[cfg(not(feature = "no_racy_asserts"))]
fn check_internal_open(ctx: &Context, path: &Path, options: &OpenOptions) {
    match open_unchecked(
        &ctx.start_clone,
        ctx.canonical_path.debug.as_ref(),
        options
            .clone()
            .create(false)
            .create_new(false)
            .truncate(false),
    ) {
        Ok(unchecked_file) => {
            assert!(
                is_same_file(&ctx.base, &unchecked_file).unwrap(),
                "path resolution inconsistency: start='{:?}', path='{}'; canonical_path='{}'; \
                 got='{:?}' expected='{:?}'",
                ctx.start_clone,
                path.display(),
                ctx.canonical_path.debug.display(),
                ctx.base,
                &unchecked_file,
            );
        }
        Err(_unchecked_error) => {
            /* TODO: Check error messages.
            panic!(
                "unexpected success opening result={:?} start='{:?}', path='{}'; canonical_path='{}'; \
                 expected {:?}",
                ctx.base,
                ctx.start_clone,
                path.display(),
                ctx.canonical_path.debug.display(),
                unchecked_error,
            */
        }
    }
}
