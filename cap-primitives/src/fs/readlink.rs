//! This defines `readlink`, the primary entrypoint to sandboxed symbolic link
//! dereferencing.

use crate::fs::readlink_impl;
#[cfg(debug_assertions)]
use crate::fs::{readlink_unchecked, stat, FollowSymlinks};
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

    #[cfg(debug_assertions)]
    let unchecked = readlink_unchecked(start, path);

    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    check_readlink(start, path, &result, &unchecked);

    result
}

#[allow(clippy::enum_glob_use)]
#[cfg(debug_assertions)]
fn check_readlink(
    start: &fs::File,
    path: &Path,
    result: &io::Result<PathBuf>,
    unchecked: &io::Result<PathBuf>,
) {
    use super::map_result;
    use io::ErrorKind::*;

    match (map_result(result), map_result(unchecked)) {
        (Ok(target), Ok(unchecked_target)) => {
            assert_eq!(target, unchecked_target);
        }

        (Err((PermissionDenied, message)), _) => {
            match map_result(&stat(start, path, FollowSymlinks::No)) {
                Err((PermissionDenied, canon_message)) => {
                    assert_eq!(message, canon_message);
                }
                _ => panic!("readlink failed where canonicalize succeeded"),
            }
        }

        (Err((_kind, _message)), Err((_unchecked_kind, _unchecked_message))) => {
            /* TODO: Check error messages.
            assert_eq!(kind, unchecked_kind);
            assert_eq!(message, unchecked_message);
            */
        }

        other => panic!(
            "unexpected result from readlink start='{:?}', path='{}':\n{:#?}",
            start,
            path.display(),
            other,
        ),
    }
}
