//! This defines `open`, the primary entrypoint to sandboxed file and directory opening.

use crate::fs::{open_impl, OpenOptions};
use std::{fs, io, path::Path};
#[cfg(debug_assertions)]
use {
    super::get_path,
    crate::fs::{is_same_file, open_unchecked, OpenUncheckedError},
};

/// Perform an `openat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(not(debug_assertions), allow(clippy::let_and_return))]
#[inline]
pub fn open(start: &fs::File, path: &Path, options: &OpenOptions) -> io::Result<fs::File> {
    // Call the underlying implementation.
    let result = open_impl(start, path, options);

    // Do an unsandboxed lookup and check that we found the same result.
    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    match open_unchecked(
        start,
        path,
        options
            .clone()
            .create(false)
            .create_new(false)
            .truncate(false),
    ) {
        Ok(unchecked_file) => match &result {
            Ok(result_file) => {
                assert!(is_same_file(result_file, &unchecked_file)?,
                    "path resolution inconsistency: start='{:?}', path='{}' got='{:?}' expected='{:?}'",
                    start, path.display(), result_file, &unchecked_file);
            }
            Err(e) => match e.kind() {
                io::ErrorKind::PermissionDenied | io::ErrorKind::InvalidInput => (),
                io::ErrorKind::AlreadyExists if options.create_new => (),
                _ => panic!(
                    "unexpected error opening start='{:?}', path='{}': {:?}",
                    start,
                    path.display(),
                    e
                ),
            },
        },
        Err(unchecked_error) => match &result {
            Ok(result_file) => panic!(
                "unexpected success opening start='{:?}', path='{}'; expected {:?}; got {:?}",
                start,
                path.display(),
                unchecked_error,
                result_file
            ),
            Err(result_error) => match result_error.kind() {
                io::ErrorKind::PermissionDenied | io::ErrorKind::InvalidInput => (),
                _ => {
                    let unchecked_error = match unchecked_error {
                        OpenUncheckedError::Other(err) | OpenUncheckedError::Symlink(err) => err,
                    };
                    assert_eq!(result_error.to_string(), unchecked_error.to_string());
                    assert_eq!(result_error.kind(), unchecked_error.kind());
                }
            },
        },
    }

    // On operating systems which can tell us the path of a file descriptor,
    // assert that the start path is a parent of the result path.
    #[cfg(debug_assertions)]
    if let Ok(result_file) = &result {
        if let Some(result_path) = get_path(result_file) {
            if let Some(start_path) = get_path(start) {
                assert!(
                    result_path.starts_with(start_path),
                    "sandbox escape: start='{:?}' result='{}'",
                    start,
                    result_path.display()
                );
            }
        }
    }

    result
}
