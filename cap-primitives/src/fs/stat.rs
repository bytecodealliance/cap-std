//! This defines `stat`, the primary entrypoint to sandboxed metadata querying.

#[cfg(not(feature = "no_racy_asserts"))]
use crate::fs::{canonicalize, map_result, stat_unchecked};
use crate::fs::{stat_impl, FollowSymlinks, Metadata};
use std::{fs, io, path::Path};

/// Perform an `fstatat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(feature = "no_racy_asserts", allow(clippy::let_and_return))]
#[inline]
pub fn stat(start: &fs::File, path: &Path, follow: FollowSymlinks) -> io::Result<Metadata> {
    // Call the underlying implementation.
    let result = stat_impl(start, path, follow);

    #[cfg(not(feature = "no_racy_asserts"))]
    let stat = stat_unchecked(start, path, follow);

    #[cfg(not(feature = "no_racy_asserts"))]
    check_stat(start, path, follow, &result, &stat);

    result
}

#[cfg(not(feature = "no_racy_asserts"))]
#[allow(clippy::enum_glob_use)]
fn check_stat(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
    result: &io::Result<Metadata>,
    stat: &io::Result<Metadata>,
) {
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
