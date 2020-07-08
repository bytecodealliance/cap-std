//! This defines `readlink`, the primary entrypoint to sandboxed symbolic link
//! dereferencing.

use crate::fs::readlink_impl;
#[cfg(debug_assertions)]
use crate::fs::readlink_unchecked;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Perform a `readlinkat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(not(debug_assertions), allow(clippy::let_and_return))]
#[inline]
pub fn readlink(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    // Call the underlying implementation.
    let result = readlink_impl(start, path);

    // Do an unsandboxed lookup and check that we found the same result.
    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    match readlink_unchecked(start, path) {
        Ok(unchecked_target) => match &result {
            Ok(result_target) => debug_assert_eq!(result_target, &unchecked_target),
            Err(e) => match e.kind() {
                io::ErrorKind::PermissionDenied => (),
                _ => panic!(
                    "unexpected error opening start='{:?}', path='{}': {:?}",
                    start,
                    path.display(),
                    e
                ),
            },
        },
        Err(unchecked_error) => match &result {
            Ok(_) => panic!(
                "unexpected success opening start='{:?}', path='{}'; expected {:?}",
                start,
                path.display(),
                unchecked_error
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

    result
}
