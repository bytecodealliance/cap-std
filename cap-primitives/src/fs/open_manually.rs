//! Manual path resolution, one component at a time, with manual symlink
//! resolution, in order to enforce sandboxing.

#[cfg(not(feature = "no_racy_asserts"))]
use crate::fs::is_same_file;
use crate::fs::{
    dir_options, errors, open_unchecked, path_requires_dir, readlink_one, FollowSymlinks,
    MaybeOwnedFile, OpenOptions,
};
use std::{
    borrow::Cow,
    ffi::OsStr,
    fs, io,
    path::{Component, Path, PathBuf},
};

/// Like `std::path::Component` except we combine `Prefix` and `RootDir` since
/// we don't support absolute paths, and `Normal` has a `Cow` instead of a plain
/// `OsStr` reference, so it can optionally own its own string.
#[derive(Debug)]
enum CowComponent<'borrow> {
    PrefixOrRootDir,
    CurDir,
    ParentDir,
    Normal(Cow<'borrow, OsStr>),
}

/// Convert a `Component` into a `CowComponent` which borrows strings.
fn to_borrowed_component(component: Component) -> CowComponent {
    match component {
        Component::Prefix(_) | Component::RootDir => CowComponent::PrefixOrRootDir,
        Component::CurDir => CowComponent::CurDir,
        Component::ParentDir => CowComponent::ParentDir,
        Component::Normal(os_str) => CowComponent::Normal(os_str.into()),
    }
}

/// Convert a `Component` into a `CowComponent` which owns strings.
fn to_owned_component<'borrow>(component: Component) -> CowComponent<'borrow> {
    match component {
        Component::Prefix(_) | Component::RootDir => CowComponent::PrefixOrRootDir,
        Component::CurDir => CowComponent::CurDir,
        Component::ParentDir => CowComponent::ParentDir,
        Component::Normal(os_str) => CowComponent::Normal(os_str.to_os_string().into()),
    }
}

/// Utility for collecting the canonical path components.
struct CanonicalPath<'path_buf> {
    /// If the user requested a canonical path, a reference to the `PathBuf` to
    /// write it to.
    path: Option<&'path_buf mut PathBuf>,

    /// Our own private copy of the canonical path, for assertion checking.
    #[cfg(not(feature = "no_racy_asserts"))]
    debug: PathBuf,
}

impl<'path_buf> CanonicalPath<'path_buf> {
    fn new(path: Option<&'path_buf mut PathBuf>) -> Self {
        Self {
            #[cfg(not(feature = "no_racy_asserts"))]
            debug: PathBuf::new(),

            path,
        }
    }

    fn push(&mut self, one: &OsStr) {
        #[cfg(not(feature = "no_racy_asserts"))]
        self.debug.push(one);

        if let Some(path) = &mut self.path {
            path.push(one)
        }
    }

    fn pop(&mut self) -> bool {
        #[cfg(not(feature = "no_racy_asserts"))]
        self.debug.pop();

        if let Some(path) = &mut self.path {
            path.pop()
        } else {
            true
        }
    }

    /// The complete canonical path has been scanned. Set `path` to `None`
    /// so that it isn't cleared when `self` is dropped.
    fn complete(&mut self) {
        // Replace "" with ".", since "" as a relative path is interpreted as an error.
        if let Some(path) = &mut self.path {
            if path.as_os_str().is_empty() {
                path.push(Component::CurDir);
            }
            self.path = None;
        }
    }
}

impl<'path_buf> Drop for CanonicalPath<'path_buf> {
    fn drop(&mut self) {
        // If `self.path` is still `Some` here, it means that we haven't called
        // `complete()` yet, meaning the `CanonicalPath` is being dropped before
        // the complete path has been processed. In that case, clear `path` to
        // indicate that we weren't able to obtain a complete path.
        if let Some(path) = &mut self.path {
            path.clear();
            self.path = None;
        }
    }
}

/// A wrapper around `open_manually` which starts with a `symlink_count` of 0
/// and does not return the canonical path, so it has the signature needed to be
/// used as `open_impl`.
pub(crate) fn open_manually_wrapper(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    let mut symlink_count = 0;
    let start = MaybeOwnedFile::borrowed(start);
    open_manually(start, path, options, &mut symlink_count, None)
        .and_then(|maybe_owned| maybe_owned.into_file(options))
}

/// Implement `open` by breaking up the path into components, resolving each
/// component individually, and resolving symbolic links manually. If requested,
/// also produce the canonical path along the way.
///
/// Callers can request the canonical path by passing `Some` to
/// `canonical_path`. If the complete canonical path is processed, even if
/// `open_manually` returns an `Err`, it will be stored in the provided
/// `&mut PathBuf`. If an error occurs before the complete canonical path is
/// processed, the provided `&mut PathBuf` is cleared to empty.
pub(crate) fn open_manually<'start>(
    start: MaybeOwnedFile<'start>,
    path: &Path,
    options: &OpenOptions,
    symlink_count: &mut u8,
    canonical_path: Option<&mut PathBuf>,
) -> io::Result<MaybeOwnedFile<'start>> {
    #[cfg(not(feature = "no_racy_asserts"))]
    let start_clone = MaybeOwnedFile::owned(start.try_clone().unwrap());

    // POSIX returns `ENOENT` on an empty path. TODO: On Windows, we should
    // be compatible with what Windows does instead.
    if path.as_os_str().is_empty() {
        return Err(errors::no_such_file_or_directory());
    }

    let mut components = path
        .components()
        .map(to_borrowed_component)
        .rev()
        .collect::<Vec<_>>();
    let mut base = start;
    let mut dirs = Vec::with_capacity(components.len());
    let mut canonical_path = CanonicalPath::new(canonical_path);
    let dir_options = dir_options();

    // Does the path end in `/` or similar, so it requires a directory?
    let mut dir_required = path_requires_dir(path);

    // Are we requesting write permissions, so we can't open a directory?
    let dir_precluded = options.write || options.append;

    while let Some(c) = components.pop() {
        match c {
            CowComponent::PrefixOrRootDir => return Err(errors::escape_attempt()),
            CowComponent::CurDir => {
                // If the path ends in `.` and we want write access, fail.
                if components.is_empty() {
                    if dir_precluded {
                        return Err(errors::is_directory());
                    }
                    if !base.metadata()?.is_dir() {
                        return Err(errors::is_not_directory());
                    }
                    canonical_path.push(Component::CurDir.as_os_str());
                }

                // Otherwise just skip `.`.
                continue;
            }
            CowComponent::ParentDir => {
                #[cfg(not(feature = "no_racy_asserts"))]
                assert!(dirs.is_empty() || !is_same_file(&start_clone, &base)?);

                if components.is_empty() && dir_precluded {
                    return Err(errors::is_directory());
                }

                // We hold onto all the parent directory descriptors so that we
                // don't have to re-open anything when we encounter a `..`. This
                // way, even if the directory is concurrently moved, we don't have
                // to worry about `..` leaving the sandbox.
                match dirs.pop() {
                    Some(dir) => base = dir,
                    None => return Err(errors::escape_attempt()),
                }
                assert!(canonical_path.pop());
            }
            CowComponent::Normal(one) => {
                // If the path requires a directory and we'd open it for writing, fail.
                if components.is_empty() && dir_required && dir_precluded {
                    return Err(errors::is_directory());
                }

                let use_options = if components.is_empty() && !dir_required {
                    options
                } else {
                    &dir_options
                };
                match open_unchecked(
                    &base,
                    one.as_ref(),
                    use_options.clone().follow(FollowSymlinks::No),
                ) {
                    Ok(file) => {
                        // Emulate `O_PATH` + `FollowSymlinks::Yes` on Linux. If `file` is a
                        // symlink, follow it.
                        #[cfg(target_os = "linux")]
                        if should_emulate_o_path(use_options) {
                            match readlink_one(&file, Default::default(), symlink_count) {
                                Ok(destination) => {
                                    components.extend(
                                        destination.components().map(to_owned_component).rev(),
                                    );
                                    dir_required |= path_requires_dir(&destination);
                                    continue;
                                }
                                // If it isn't a symlink, handle it as normal. `readlinkat` returns
                                // `ENOENT` if the file isn't a symlink in this situation.
                                Err(err) if err.kind() == io::ErrorKind::NotFound => (),
                                // If `readlinkat` fails any other way, pass it on.
                                Err(err) => return Err(err),
                            }
                        }

                        // Normal case
                        let prev_base = base.descend_to(MaybeOwnedFile::owned(file));
                        dirs.push(prev_base);
                        canonical_path.push(&one);
                    }
                    Err(OpenUncheckedError::Symlink(err)) => {
                        if use_options.follow == FollowSymlinks::No && components.is_empty() {
                            canonical_path.push(&one);
                            canonical_path.complete();
                            return Err(err);
                        }
                        let destination = readlink_one(&base, &one, symlink_count)?;
                        components.extend(destination.components().map(to_owned_component).rev());
                        dir_required |= path_requires_dir(&destination);
                    }
                    Err(OpenUncheckedError::NotFound(err)) => return Err(err),
                    Err(OpenUncheckedError::Other(err)) => {
                        // An error occurred. If this was the last component, and the error wasn't
                        // due to invalid inputs (eg. the path has an embedded NUL), record it as
                        // the last component of the canonical path, even if we couldn't open it.
                        if components.is_empty() && err.kind() != io::ErrorKind::InvalidInput {
                            canonical_path.push(&one);
                            canonical_path.complete();
                        }
                        return Err(err);
                    }
                }
            }
        }
    }

    #[cfg(not(feature = "no_racy_asserts"))]
    check_open(&start_clone, path, options, &canonical_path, &base);

    canonical_path.complete();
    Ok(base)
}

/// Test whether the given options imply that we should treat an open file as
/// potentially being a symlink we need to follow, due to use of `O_PATH`.
#[cfg(target_os = "linux")]
fn should_emulate_o_path(use_options: &OpenOptions) -> bool {
    (use_options.ext.custom_flags & libc::O_PATH) == libc::O_PATH
        && use_options.follow == FollowSymlinks::Yes
}

#[cfg(not(feature = "no_racy_asserts"))]
fn check_open(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
    canonical_path: &CanonicalPath,
    base: &MaybeOwnedFile,
) {
    match open_unchecked(
        start,
        canonical_path.debug.as_ref(),
        options
            .clone()
            .create(false)
            .create_new(false)
            .truncate(false),
    ) {
        Ok(unchecked_file) => {
            assert!(
                is_same_file(base, &unchecked_file).unwrap(),
                "path resolution inconsistency: start='{:?}', path='{}'; canonical_path='{}'; \
                 got='{:?}' expected='{:?}'",
                start,
                path.display(),
                canonical_path.debug.display(),
                base,
                &unchecked_file,
            );
        }
        Err(_unchecked_error) => {
            /* TODO: Check error messages.
            panic!(
                "unexpected success opening result={:?} start='{:?}', path='{}'; canonical_path='{}'; \
                 expected {:?}",
                base,
                start,
                path.display(),
                canonical_path.debug.display(),
                unchecked_error,
            */
        }
    }
}

#[derive(Debug)]
pub(crate) enum OpenUncheckedError {
    Other(io::Error),
    Symlink(io::Error),
    NotFound(io::Error),
}

impl From<OpenUncheckedError> for io::Error {
    fn from(error: OpenUncheckedError) -> io::Error {
        match error {
            OpenUncheckedError::Other(err)
            | OpenUncheckedError::Symlink(err)
            | OpenUncheckedError::NotFound(err) => err,
        }
    }
}
