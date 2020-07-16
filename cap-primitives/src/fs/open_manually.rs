//! Manual path resolution, one component at a time, with manual symlink
//! resolution, in order to enforce sandboxing.

use crate::fs::{
    dir_options, errors, is_same_file, open_unchecked, path_requires_dir, readlink_one,
    FollowSymlinks, MaybeOwnedFile, OpenOptions,
};
use std::{
    ffi::OsString,
    fs, io,
    path::{Component, Path, PathBuf},
};

/// Like `std::path::Component` except we combine `Prefix` and `RootDir` since
/// we don't support absolute paths, and `Normal` has an owned `OsString` instead
/// of an `OsStr` reference, so it doesn't need a lifetime parameter.
#[derive(Debug)]
enum OwnedComponent {
    PrefixOrRootDir,
    CurDir,
    ParentDir,
    Normal(OsString),
}

/// Convert a `Component` into an `OwnedComponent`.
fn to_owned_component(component: Component) -> OwnedComponent {
    match component {
        Component::Prefix(_) | Component::RootDir => OwnedComponent::PrefixOrRootDir,
        Component::CurDir => OwnedComponent::CurDir,
        Component::ParentDir => OwnedComponent::ParentDir,
        Component::Normal(os_str) => OwnedComponent::Normal(os_str.to_os_string()),
    }
}

/// Utility for collecting the canonical path components.
struct CanonicalPath<'path_buf> {
    /// If the user requested a canonical path, a reference to the `PathBuf` to
    /// write it to.
    path: Option<&'path_buf mut PathBuf>,

    /// Our own private copy of the canonical path, for assertion checking.
    #[cfg(debug_assertions)]
    debug: PathBuf,
}

impl<'path_buf> CanonicalPath<'path_buf> {
    fn new(path: Option<&'path_buf mut PathBuf>) -> Self {
        Self {
            #[cfg(debug_assertions)]
            debug: PathBuf::new(),

            path,
        }
    }

    fn push(&mut self, one: OsString) {
        #[cfg(debug_assertions)]
        self.debug.push(one.clone());

        if let Some(path) = &mut self.path {
            path.push(one)
        }
    }

    fn pop(&mut self) -> bool {
        #[cfg(debug_assertions)]
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
/// and does not return the canonical path, so it has the signature needed
/// to be used as `open_impl`.
pub(crate) fn open_manually_wrapper(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    let mut symlink_count = 0;
    open_manually(start, path, options, &mut symlink_count, None)
}

/// Implement `open` by breaking up the path into components and resolving
/// each component individually, and resolving symbolic links manually. This
/// implementation can also optionally produce the canonical path computed along
/// the way.
///
/// Callers can request the canonical path by passing `Some` to
/// `canonical_path`.  If the complete canonical path is processed, even if
/// `open_manually` returns an `Err`, it will be stored in the provided
/// `&mut PathBuf`. If an error occurs before the complete canonical path is
/// processed, the provided `&mut PathBuf` is cleared to empty.
pub(crate) fn open_manually(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
    symlink_count: &mut u8,
    canonical_path: Option<&mut PathBuf>,
) -> io::Result<fs::File> {
    open_manually_maybe(start, path, options, symlink_count, canonical_path)
        .and_then(MaybeOwnedFile::into_file)
}

/// The main body of `open_manually`, which returns a `MaybeOwnedFile` instead
/// of a `std::fs::File` so that users within this crate can avoid calling
/// `ManuallyOwnedFile::into_file`, which allocates a new file descriptor in
/// some cases.
pub(crate) fn open_manually_maybe<'start>(
    start: &'start fs::File,
    path: &Path,
    options: &OpenOptions,
    symlink_count: &mut u8,
    canonical_path: Option<&mut PathBuf>,
) -> io::Result<MaybeOwnedFile<'start>> {
    // POSIX returns `ENOENT` on an empty path. TODO: On Windows, we should
    // be compatible with what Windows does instead.
    if path.as_os_str().is_empty() {
        return Err(errors::no_such_file_or_directory());
    }

    let mut components = path
        .components()
        .map(to_owned_component)
        .rev()
        .collect::<Vec<_>>();
    let mut base = MaybeOwnedFile::borrowed(start);
    let mut dirs = Vec::new();
    let mut canonical_path = CanonicalPath::new(canonical_path);
    let dir_options = dir_options();

    // Does the path end in `/` or similar, so it requires a directory?
    let mut dir_required = path_requires_dir(path);

    // Are we requesting write permissions, so we can't open a directory?
    let dir_precluded = options.write || options.append;

    while let Some(c) = components.pop() {
        match c {
            OwnedComponent::PrefixOrRootDir => return Err(errors::escape_attempt()),
            OwnedComponent::CurDir => {
                // If the path ends in `.` and we want write access, fail.
                if components.is_empty() {
                    if dir_precluded {
                        return Err(errors::is_directory());
                    }
                    if !base.as_ref().metadata()?.is_dir() {
                        return Err(errors::is_not_directory());
                    }
                    canonical_path.push(Component::CurDir.as_os_str().to_os_string());
                }

                // Otherwise just skip `.`.
                continue;
            }
            OwnedComponent::ParentDir => {
                // TODO: This is a racy check, though it is useful for testing and fuzzing.
                debug_assert!(dirs.is_empty() || !is_same_file(start, base.as_ref())?);

                if components.is_empty() && dir_precluded {
                    return Err(errors::is_directory());
                }

                // We hold onto all the parent directory descriptors so that we
                // don't have to re-open anything when we encounter a `..`.
                match dirs.pop() {
                    Some(dir) => base = dir,
                    None => return Err(errors::escape_attempt()),
                }
                assert!(canonical_path.pop());
            }
            OwnedComponent::Normal(one) => {
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
                    base.as_ref(),
                    one.as_ref(),
                    use_options.clone().follow(FollowSymlinks::No),
                ) {
                    Ok(file) => {
                        let prev_base = base.descend_to(file);
                        dirs.push(prev_base);
                        if one != Component::CurDir.as_os_str() {
                            canonical_path.push(one);
                        }
                    }
                    Err(OpenUncheckedError::Symlink(err))
                        if use_options.follow == FollowSymlinks::No && components.is_empty() =>
                    {
                        canonical_path.push(one);
                        canonical_path.complete();
                        return Err(err);
                    }
                    Err(OpenUncheckedError::Symlink(_)) => {
                        let destination = readlink_one(base.as_ref(), &one, symlink_count)?;
                        components.extend(destination.components().map(to_owned_component).rev());
                        dir_required |= path_requires_dir(&destination);
                    }
                    Err(OpenUncheckedError::NotFound(err)) => {
                        return Err(err);
                    }
                    Err(OpenUncheckedError::Other(err)) => {
                        // An error occurred. If this was the last component, record it as the
                        // last component of the canonical path, even if we couldn't open it.
                        if components.is_empty() {
                            canonical_path.push(one);
                            canonical_path.complete();
                        }
                        return Err(err);
                    }
                }
            }
        }
    }

    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    check_open(start, path, options, &canonical_path, &base);

    canonical_path.complete();
    Ok(base)
}

#[cfg(debug_assertions)]
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
                is_same_file(base.as_ref(), &unchecked_file).unwrap(),
                "path resolution inconsistency: start='{:?}', path='{}'; canonical_path='{}'; \
                 got='{:?}' expected='{:?}'",
                start,
                path.display(),
                canonical_path.debug.display(),
                base.as_ref(),
                &unchecked_file,
            );
        }
        Err(_unchecked_error) => {
            /* TODO: Check error messages.
            panic!(
                "unexpected success opening result={:?} start='{:?}', path='{}'; canonical_path='{}'; \
                 expected {:?}",
                base.as_ref(),
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
