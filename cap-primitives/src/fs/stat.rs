//! This defines `stat`, the primary entrypoint to sandboxed metadata querying.

#[cfg(debug_assertions)]
use crate::fs::{canonicalize, stat_unchecked};
use crate::fs::{stat_impl, FollowSymlinks, Metadata};
use std::{fs, io, path::Path};

/// Perform an `fstatat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(not(debug_assertions), allow(clippy::let_and_return))]
#[inline]
pub fn stat(start: &fs::File, path: &Path, follow: FollowSymlinks) -> io::Result<Metadata> {
    // Call the underlying implementation.
    let result = stat_impl(start, path, follow);

    #[cfg(debug_assertions)]
    let stat = stat_unchecked(start, path, follow);

    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    check_stat(start, path, follow, &result, &stat);

    result
}

#[allow(clippy::enum_glob_use)]
#[cfg(debug_assertions)]
fn check_stat(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
    result: &io::Result<Metadata>,
    stat: &io::Result<Metadata>,
) {
    use super::map_result;
    use io::ErrorKind::*;

    match (map_result(result), map_result(stat)) {
        (Ok(metadata), Ok(unchecked_metadata)) => {
            assert!(
                metadata.is_same_file(&unchecked_metadata),
                "path resolution inconsistency: start='{:?}', path='{}' got='{:?}' expected='{:?}'",
                start,
                path.display(),
                metadata,
                unchecked_metadata
            );
        }

        (Err((PermissionDenied, message)), _) => {
            if let FollowSymlinks::Yes = follow {
                match map_result(&canonicalize(start, path)) {
                    Err((PermissionDenied, canon_message)) => {
                        assert_eq!(message, canon_message);
                    }
                    _ => panic!("stat failed where canonicalize succeeded"),
                }
            } else {
                // TODO: Check that stat in the no-follow case got the right error.
            }
        }

        (Err((kind, message)), Err((unchecked_kind, unchecked_message))) => {
            assert_eq!(kind, unchecked_kind);
            assert_eq!(message, unchecked_message);
        }

        other => panic!(
            "unexpected result from stat start='{:?}', path='{}':\n{:#?}",
            start,
            path.display(),
            other,
        ),
    }
}
