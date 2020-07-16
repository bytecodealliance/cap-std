//! This defines `unlink`, the primary entrypoint to sandboxed file removal.

use crate::fs::unlink_impl;
#[cfg(debug_assertions)]
use crate::fs::{
    canonicalize_manually, stat_unchecked, unlink_unchecked, FollowSymlinks, Metadata,
};
use std::{fs, io, path::Path};

/// Perform a `unlinkat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(not(debug_assertions), allow(clippy::let_and_return))]
#[inline]
pub fn unlink(start: &fs::File, path: &Path) -> io::Result<()> {
    #[cfg(debug_assertions)]
    let stat_before = stat_unchecked(start, path, FollowSymlinks::No);

    // Call the underlying implementation.
    let result = unlink_impl(start, path);

    #[cfg(debug_assertions)]
    let stat_after = stat_unchecked(start, path, FollowSymlinks::No);

    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    check_unlink(start, path, &stat_before, &result, &stat_after);

    result
}

#[allow(clippy::enum_glob_use)]
#[cfg(debug_assertions)]
fn check_unlink(
    start: &fs::File,
    path: &Path,
    stat_before: &io::Result<Metadata>,
    result: &io::Result<()>,
    stat_after: &io::Result<Metadata>,
) {
    use super::map_result;
    use io::ErrorKind::*;

    match (
        map_result(stat_before),
        map_result(result),
        map_result(stat_after),
    ) {
        (Ok(metadata), Ok(()), Err((NotFound, _))) => {
            // TODO: Check that the path was inside the sandbox.
            assert!(!metadata.is_dir());
        }

        (Err((Other, _)), Ok(()), Err((NotFound, _))) => {
            // TODO: Check that the path was inside the sandbox.
        }

        (_, Err((_kind, _message)), _) => {
            match map_result(&canonicalize_manually(start, path, FollowSymlinks::No)) {
                Ok(canon) => match map_result(&unlink_unchecked(start, &canon)) {
                    Err((_unchecked_kind, _unchecked_message)) => {
                        /* TODO: Check error messages.
                        assert_eq!(
                            kind,
                            unchecked_kind,
                            "unexpected error kind from unlink start='{:?}', \
                             path='{}':\nstat_before={:#?}\nresult={:#?}\nstat_after={:#?}",
                            start,
                            path.display(),
                            stat_before,
                            result,
                            stat_after
                        );
                        assert_eq!(message, unchecked_message);
                        */
                    }
                    _ => panic!("unsandboxed unlink success"),
                },
                Err((_canon_kind, _canon_message)) => {
                    /* TODO: Check error messages.
                    assert_eq!(kind, canon_kind, "'{}' vs '{}'", message, canon_message);
                    assert_eq!(message, canon_message);
                    */
                }
            }
        }

        other => panic!(
            "inconsistent unlink checks: start='{:?}' path='{}':\n{:#?}",
            start,
            path.display(),
            other,
        ),
    }

    match stat_unchecked(start, path, FollowSymlinks::No) {
        Ok(unchecked_metadata) => match &result {
            Ok(()) => panic!(
                "file still exists after unlink start='{:?}', path='{}'",
                start,
                path.display()
            ),
            Err(e) => match e.kind() {
                io::ErrorKind::PermissionDenied => (),
                io::ErrorKind::Other if unchecked_metadata.is_dir() => (),
                _ => panic!(
                    "unexpected error unlinking start='{:?}', path='{}': {:?}",
                    start,
                    path.display(),
                    e
                ),
            },
        },
        Err(_unchecked_error) => match &result {
            Ok(()) => (),
            Err(result_error) => match result_error.kind() {
                io::ErrorKind::PermissionDenied => (),
                _ => {
                    /* TODO: Check error messages.
                    assert_eq!(result_error.to_string(), unchecked_error.to_string());
                    assert_eq!(result_error.kind(), unchecked_error.kind());
                    */
                }
            },
        },
    }
}
