//! This defines `readlink`, the primary entrypoint to sandboxed symbolic link
//! dereferencing.

use crate::fs::{errors, readlink_impl};
#[cfg(racy_asserts)]
use crate::fs::{map_result, readlink_unchecked, stat, FollowSymlinks};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Perform a `readlinkat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(not(racy_asserts), allow(clippy::let_and_return))]
#[inline]
pub fn readlink(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    // Call the underlying implementation.
    let result = readlink_impl(start, path);

    #[cfg(racy_asserts)]
    let unchecked = readlink_unchecked(start, path, PathBuf::new());

    #[cfg(racy_asserts)]
    check_readlink(start, path, &result, &unchecked);

    // Don't allow reading symlinks to absolute paths. This isn't strictly
    // necessary to preserve the sandbox, since `open` will refuse to follow
    // absolute paths in any case. However, it is useful to enforce this
    // restriction to avoid leaking information about the host filesystem
    // outside the sandbox.
    if let Ok(path) = &result {
        if path.has_root() {
            return Err(errors::escape_attempt());
        }
    }

    result
}

#[cfg(racy_asserts)]
#[allow(clippy::enum_glob_use)]
fn check_readlink(
    start: &fs::File,
    path: &Path,
    result: &io::Result<PathBuf>,
    unchecked: &io::Result<PathBuf>,
) {
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
