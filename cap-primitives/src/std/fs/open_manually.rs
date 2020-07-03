//! Manual path resolution, one component at a time, with manual symlink
//! resolution, in order to enforce sandboxing.

#[cfg(debug_assertions)]
use crate::fs::get_path;
use crate::fs::{is_same_file, open_unchecked, resolve_symlink_at, MaybeOwnedFile, OpenOptions};
use std::{
    borrow::ToOwned,
    ffi::OsString,
    fs, io, mem,
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
        Component::Normal(os_str) => OwnedComponent::Normal((*os_str).to_owned()),
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
            path,
            #[cfg(debug_assertions)]
            debug: PathBuf::new(),
        }
    }

    fn push(&mut self, one: OsString) {
        #[cfg(debug_assertions)]
        self.debug.push(one.clone());
        if let Some(path) = &mut self.path {
            path.push(one)
        }
    }

    fn pop(&mut self) {
        #[cfg(debug_assertions)]
        self.debug.pop();
        if let Some(path) = &mut self.path {
            path.pop();
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
pub(crate) fn open_manually(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
    symlink_count: &mut u8,
    canonical_path: Option<&mut PathBuf>,
) -> io::Result<fs::File> {
    let mut components = path
        .components()
        .map(to_owned_component)
        .rev()
        .collect::<Vec<_>>();

    let mut base = MaybeOwnedFile::Borrowed(start);
    let mut dirs = Vec::new();
    let mut canonical_path = CanonicalPath::new(canonical_path);

    while let Some(c) = components.pop() {
        match c {
            OwnedComponent::PrefixOrRootDir => return escape_attempt(),
            OwnedComponent::CurDir => {
                // If the "." is the entire string, open it. Otherwise just skip it.
                if components.is_empty() {
                    components.push(OwnedComponent::Normal(OsString::from(".")))
                } else {
                    canonical_path.push(OsString::from("."));
                }
                continue;
            }
            OwnedComponent::ParentDir => {
                // TODO: This is a racy check, though it is useful for testing and fuzzing.
                debug_assert!(dirs.is_empty() || !is_same_file(start, base.as_file())?);

                match dirs.pop() {
                    Some(dir) => base = dir,
                    None => return escape_attempt(),
                }
                canonical_path.pop();
            }
            OwnedComponent::Normal(one) => {
                let dir_options = OpenOptions::new().read(true).clone();
                let use_options = if components.is_empty() {
                    options
                } else {
                    &dir_options
                };
                match open_unchecked(
                    base.as_file(),
                    one.as_ref(),
                    use_options.clone().nofollow(true),
                ) {
                    Ok(file) => {
                        let prev_base = mem::replace(&mut base, MaybeOwnedFile::Owned(file));
                        dirs.push(prev_base);
                        canonical_path.push(one);
                    }
                    Err(e) => match e.raw_os_error() {
                        Some(libc::ELOOP) | Some(libc::EMLINK) if use_options.nofollow => {
                            return Err(io::Error::from_raw_os_error(libc::ELOOP))
                        }
                        Some(libc::ELOOP) | Some(libc::EMLINK) => {
                            let destination =
                                resolve_symlink_at(base.as_file(), &one, symlink_count)?;
                            components
                                .extend(destination.components().map(to_owned_component).rev());
                        }
                        _ => return Err(e),
                    },
                }
            }
        }
    }

    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    match open_unchecked(
              start,
              canonical_path.debug.as_ref(),
              options
                  .clone()
                  .create(false)
                  .create_new(false)
                  .truncate(false),
          )
    {
        Ok(unchecked_file) => {
            assert!(
                is_same_file(base.as_file(), &unchecked_file)?,
                "path resolution inconsistency: start='{:?}', path='{}'; canonical_path='{}'; got='{:?}' expected='{:?}'",
                get_path(start),
                path.display(),
                canonical_path.debug.display(),
                get_path(base.as_file()),
                get_path(&unchecked_file),
            );
        }
        Err(unchecked_error) => panic!(
            "unexpected success opening result={:?} start='{:?}', path='{}'; canonical_path='{}'; expected {:?}",
            base.as_file(),
            get_path(start),
            path.display(),
            canonical_path.debug.display(),
            unchecked_error,
        ),
    }

    base.into_file()
}

#[cold]
fn escape_attempt() -> io::Result<fs::File> {
    Err(io::Error::new(
        io::ErrorKind::PermissionDenied,
        "a path led outside of the filesystem",
    ))
}
