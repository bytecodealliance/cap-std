//! This defines `stat`, the primary entrypoint to sandboxed metadata querying.

#[cfg(debug_assertions)]
use super::get_path;
#[cfg(debug_assertions)]
use crate::fs::stat_unchecked;
use crate::fs::{stat_impl, FollowSymlinks, Metadata};
use std::{fs, io, path::Path};

/// Perform an `fstatat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(not(debug_assertions), allow(clippy::let_and_return))]
#[inline]
pub fn stat(start: &fs::File, path: &Path, follow: FollowSymlinks) -> io::Result<Metadata> {
    // Call the underlying implementation.
    let result = stat_impl(start, path, follow);

    // Do an unsandboxed lookup and check that we found the same result.
    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    match stat_unchecked(start, path, follow) {
        Ok(unchecked_metadata) => match &result {
            Ok(result_metadata) => {
                assert!(result_metadata.is_same_file(&unchecked_metadata),
                    "path resolution inconsistency: start='{:?}', path='{}' got='{:?}' expected='{:?}'",
                    get_path(start), path.display(), result_metadata, unchecked_metadata);
            }
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
            Ok(result_metadata) => panic!(
                "unexpected success opening start='{:?}', path='{}'; expected {:?}; got {:?}",
                get_path(start),
                path.display(),
                unchecked_error,
                result_metadata
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
