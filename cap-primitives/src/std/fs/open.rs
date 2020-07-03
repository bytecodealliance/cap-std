//! This defines `open`, the primary entrypoint to sandboxed `open`.

#[cfg(debug_assertions)]
use super::get_path;
#[cfg(debug_assertions)]
use crate::fs::{is_same_file, open_unchecked};
use crate::fs::{open_impl, OpenOptions};
use std::{fs, io, path::Path};

/// Perform an `openat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
pub fn open(start: &fs::File, path: &Path, options: &OpenOptions) -> io::Result<fs::File> {
    // Call `open`.
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
                    get_path(start), path.display(), get_path(&result.unwrap()), get_path(&unchecked_file));
            }
            Err(e) => match e.kind() {
                io::ErrorKind::PermissionDenied => (),
                io::ErrorKind::AlreadyExists if options.create_new => (),
                _ => panic!(
                    "unexpected error opening start='{:?}', path='{}': {:?}",
                    get_path(start),
                    path.display(),
                    e
                ),
            },
        },
        Err(unchecked_error) => match &result {
            Ok(result_file) => panic!(
                "unexpected success opening start='{:?}', path='{}'; expected {:?}; got {:?}",
                get_path(start),
                path.display(),
                unchecked_error,
                result_file
            ),
            Err(result_error) => match result_error.kind() {
                io::ErrorKind::PermissionDenied => (),
                _ => {
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
                for (start_part, result_part) in
                    start_path.components().zip(result_path.components())
                {
                    assert_eq!(
                        start_part,
                        result_part,
                        "sandbox escape: start='{}' result='{}'",
                        start_path.display(),
                        result_path.display()
                    );
                }
            }
        }
    }

    result
}
