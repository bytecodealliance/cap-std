//! This defines `mkdir`, the primary entrypoint to sandboxed directory creation.

use crate::fs::mkdir_impl;
#[cfg(debug_assertions)]
use crate::fs::{stat_unchecked, FollowSymlinks};
use std::{fs, io, path::Path};

/// Perform a `mkdirat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(not(debug_assertions), allow(clippy::let_and_return))]
#[inline]
pub fn mkdir(start: &fs::File, path: &Path) -> io::Result<()> {
    // Call the underlying implementation.
    let result = mkdir_impl(start, path);

    // Do an unsandboxed lookup and check that we found the same result.
    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    match stat_unchecked(start, path, FollowSymlinks::No) {
        Ok(metadata) => match &result {
            Ok(()) => debug_assert!(metadata.is_dir()),
            Err(e) => match e.kind() {
                io::ErrorKind::AlreadyExists | io::ErrorKind::PermissionDenied => (),
                _ => panic!(
                    "unexpected error opening start='{:?}', path='{}': {:?}",
                    start,
                    path.display(),
                    e
                ),
            },
        },
        Err(unchecked_error) => match &result {
            Ok(()) => panic!(
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
