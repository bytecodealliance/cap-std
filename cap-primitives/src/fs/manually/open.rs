//! Manual path resolution, one component at a time, with manual symlink
//! resolution, in order to enforce sandboxing.

use super::{readlink_one, CanonicalPath, CowComponent};
use crate::fs::{
    dir_options, errors, open_unchecked, path_has_trailing_dot, path_requires_dir, stat_unchecked,
    FollowSymlinks, MaybeOwnedFile, Metadata, OpenOptions, OpenUncheckedError,
};
use std::{
    ffi::OsStr,
    fs, io, mem,
    path::{Component, Path, PathBuf},
};
#[cfg(windows)]
use {crate::fs::SymlinkKind, winapi::um::winnt::FILE_ATTRIBUTE_DIRECTORY};

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

    /// Rust's `Path` implicitly strips off trailing `.` components, however
    /// we need to treat the last component specially, so check for and remember
    /// if there was a trailing `.` component.
    trailing_dot: bool,

    /// A `PathBuf` that we reuse for calling `readlink_one` to minimize
    /// allocations.
    reuse: PathBuf,

    #[cfg(racy_asserts)]
    start_clone: MaybeOwnedFile<'start>,
}

impl<'start> Context<'start> {
    /// Construct a new instance of `Self`.
    fn new(
        start: MaybeOwnedFile<'start>,
        path: &'start Path,
        _options: &OpenOptions,
        canonical_path: Option<&'start mut PathBuf>,
    ) -> Self {
        let components = path
            .components()
            .rev()
            .map(CowComponent::borrowed)
            .collect::<Vec<_>>();

        #[cfg(racy_asserts)]
        let start_clone = MaybeOwnedFile::owned(start.try_clone().unwrap());

        Self {
            base: start,
            dirs: Vec::with_capacity(components.len()),
            components,
            canonical_path: CanonicalPath::new(canonical_path),
            dir_required: path_requires_dir(path),

            #[cfg(not(windows))]
            dir_precluded: _options.write || _options.append,

            #[cfg(windows)]
            dir_precluded: false,

            trailing_dot: path_has_trailing_dot(path),

            reuse: PathBuf::new(),

            #[cfg(racy_asserts)]
            start_clone,
        }
    }

    /// The last component of a path is special -- unlike all other components,
    /// it is not required to be a directory.
    fn at_last_component(&self) -> bool {
        // If we had a trailing dot that `std::path::Path` implicitly stripped,
        // the last entry in `self.components` is not the actual last component.
        self.components.is_empty() && !self.trailing_dot
    }

    fn check_access(&self, component: Component) -> io::Result<()> {
        // Manually check that we have permissions to search `self.base` to
        // search for `..` in it, since we don't actually open `..`.
        #[cfg(not(windows))]
        {
            // Use `faccess` with `AT_EACCESS`. `AT_EACCCES` is not often the
            // right tool for the job; in POSIX, it's better to ask for errno
            // than to ask for permission. But we use `check_access` to check
            // access for opening `.` and `..` in situations where we already
            // have open handles to them, and now we're accessing them through
            // different paths, and we need to check whether these paths allow
            // us access.
            //
            // Android and Emscripten lack `AT_EACCESS`.
            // https://android.googlesource.com/platform/bionic/+/master/libc/bionic/faccessat.cpp
            #[cfg(any(target_os = "emscripten", target_os = "android"))]
            let at_flags = posish::fs::AtFlags::empty();
            #[cfg(not(any(target_os = "emscripten", target_os = "android")))]
            let at_flags = posish::fs::AtFlags::EACCESS;

            posish::fs::accessat(
                &*self.base,
                component.as_os_str(),
                posish::fs::Access::EXEC_OK,
                at_flags,
            )
        }
        #[cfg(windows)]
        crate::fs::open_dir_unchecked(&self.base, component.as_os_str().as_ref()).map(|_| ())
    }

    /// Handle a "." path component.
    fn cur_dir(&mut self) -> io::Result<()> {
        // If the path ends in `.` and we can't open a directory, fail.
        if self.at_last_component() {
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

        // Check that we have permission to look up `.`.
        self.check_access(Component::CurDir)?;

        // Otherwise do nothing.
        Ok(())
    }

    /// Handle a ".." path component.
    fn parent_dir(&mut self) -> io::Result<()> {
        #[cfg(racy_asserts)]
        if !self.dirs.is_empty() {
            assert_different_file!(&self.start_clone, &self.base);
        }

        if self.at_last_component() && self.dir_precluded {
            return Err(errors::is_directory());
        }

        // We hold onto all the parent directory descriptors so that we
        // don't have to re-open anything when we encounter a `..`. This
        // way, even if the directory is concurrently moved, we don't have
        // to worry about `..` leaving the sandbox.
        match self.dirs.pop() {
            Some(dir) => {
                // Check that we have permission to look up `..`.
                self.check_access(Component::ParentDir)?;

                // Looks good.
                self.base = dir;
            }
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
        // Don't use `at_last_component` here because trailing `.`s still require
        // directories.
        if self.components.is_empty() && self.dir_required && self.dir_precluded {
            return Err(errors::is_directory());
        }

        // Otherwise we're doing an open.
        let use_options = if self.at_last_component() {
            options.clone()
        } else {
            dir_options()
        };
        let dir_required = self.dir_required || use_options.dir_required;
        #[allow(clippy::redundant_clone)]
        match open_unchecked(
            &self.base,
            one.as_ref(),
            use_options
                .clone()
                .follow(FollowSymlinks::No)
                .dir_required(dir_required),
        ) {
            Ok(file) => {
                // Emulate `O_PATH` + `FollowSymlinks::Yes` on Linux. If `file` is a
                // symlink, follow it.
                #[cfg(target_os = "linux")]
                if should_emulate_o_path(&use_options) {
                    match readlink_one(
                        &file,
                        Default::default(),
                        symlink_count,
                        mem::take(&mut self.reuse),
                    ) {
                        Ok(destination) => {
                            self.dir_required |=
                                self.components.is_empty() && path_requires_dir(&destination);
                            self.trailing_dot |= path_has_trailing_dot(&destination);
                            self.components
                                .extend(destination.components().rev().map(CowComponent::owned));
                            self.reuse = destination;
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

                // If Rust's `Path` stripped a trailing `.`, check that we have
                // access to `.`.
                if self.trailing_dot {
                    self.check_access(Component::CurDir)?;
                }

                Ok(())
            }
            #[cfg(not(windows))]
            Err(OpenUncheckedError::Symlink(err, ())) => {
                self.maybe_last_component_symlink(one, symlink_count, options.follow, err)
            }
            #[cfg(windows)]
            Err(OpenUncheckedError::Symlink(err, SymlinkKind::Dir)) => {
                // If this is a Windows directory symlink, require a directory.
                self.dir_required |= self.components.is_empty();
                self.maybe_last_component_symlink(one, symlink_count, options.follow, err)
            }
            #[cfg(windows)]
            Err(OpenUncheckedError::Symlink(err, SymlinkKind::File)) => {
                // If this is a Windows file symlink, preclude a directory.
                self.dir_precluded = true;
                self.maybe_last_component_symlink(one, symlink_count, options.follow, err)
            }
            Err(OpenUncheckedError::NotFound(err)) => Err(err),
            Err(OpenUncheckedError::Other(err)) => {
                // An error occurred. If this was the last component, and the error wasn't
                // due to invalid inputs (eg. the path has an embedded NUL), record it as
                // the last component of the canonical path, even if we couldn't open it.
                if self.at_last_component() && err.kind() != io::ErrorKind::InvalidInput {
                    self.canonical_path.push(one);
                    self.canonical_path.complete();
                }
                Err(err)
            }
        }
    }

    /// Dereference one symlink level.
    fn symlink(&mut self, one: &OsStr, symlink_count: &mut u8) -> io::Result<()> {
        let destination = readlink_one(&self.base, one, symlink_count, mem::take(&mut self.reuse))?;
        self.dir_required |= self.components.is_empty() && path_requires_dir(&destination);
        self.trailing_dot |= path_has_trailing_dot(&destination);
        self.components
            .extend(destination.components().rev().map(CowComponent::owned));
        self.reuse = destination;
        Ok(())
    }

    /// Check whether this is the last component and we don't need
    /// to dereference; otherwise call `Self::symlink`.
    fn maybe_last_component_symlink(
        &mut self,
        one: &OsStr,
        symlink_count: &mut u8,
        follow: FollowSymlinks,
        err: io::Error,
    ) -> io::Result<()> {
        if follow == FollowSymlinks::No && self.at_last_component() {
            self.canonical_path.push(one);
            self.canonical_path.complete();
            return Err(err);
        }

        self.symlink(one, symlink_count)
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

    #[cfg(racy_asserts)]
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
                if ctx.at_last_component() {
                    // If this is the last component, do a non-following `stat_unchecked` on it.
                    let stat = stat_unchecked(&ctx.base, one.as_ref(), FollowSymlinks::No)?;

                    // If we weren't asked to follow symlinks, or it wasn't a symlink, we're done.
                    if options.follow == FollowSymlinks::No || !stat.file_type().is_symlink() {
                        if stat.is_dir() {
                            if ctx.dir_precluded {
                                return Err(errors::is_directory());
                            }
                        } else if ctx.dir_required {
                            return Err(errors::is_not_directory());
                        }
                        return Ok(stat);
                    }

                    // On Windows, symlinks know whether they are a file or directory.
                    #[cfg(windows)]
                    if stat.file_attributes() & FILE_ATTRIBUTE_DIRECTORY != 0 {
                        ctx.dir_required = true;
                    } else {
                        ctx.dir_precluded = true;
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

#[cfg(racy_asserts)]
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
            assert_same_file!(
                &ctx.base,
                &unchecked_file,
                "path resolution inconsistency: start='{:?}', path='{}'; canonical_path='{}'",
                ctx.start_clone,
                path.display(),
                ctx.canonical_path.debug.display(),
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
