//! This defines `unlink`, the primary entrypoint to sandboxed `unlink`.

use crate::fs::unlink_impl;
#[cfg(debug_assertions)]
use crate::fs::{get_path, stat_unchecked, FollowSymlinks};
use std::{fs, io, path::Path};

/// Perform a `unlinkat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
pub fn unlink(start: &fs::File, path: &Path) -> io::Result<()> {
    // Call `unlink`.
    let result = unlink_impl(start, path);

    // Do an unsandboxed lookup and check that we found the same result.
    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    match stat_unchecked(start, path, FollowSymlinks::No) {
        Ok(_) => match &result {
            Ok(()) => panic!(
                "file still exists after unlink start='{:?}', path='{}'",
                get_path(start),
                path.display()
            ),
            Err(e) => match e.kind() {
                io::ErrorKind::PermissionDenied => (),
                _ => panic!(
                    "unexpected error opening start='{:?}', path='{}': {:?}",
                    get_path(start),
                    path.display(),
                    e
                ),
            },
        },
        Err(unchecked_error) => match &result {
            Ok(()) => (),
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
