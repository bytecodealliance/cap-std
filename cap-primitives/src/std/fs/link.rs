//! This defines `link`, the primary entrypoint to sandboxed hard-link creation.

use crate::fs::link_impl;
use std::{fs, io, path::Path};
#[cfg(debug_assertions)]
use {crate::fs::stat_unchecked, crate::fs::FollowSymlinks};

/// Perform a `linkat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(not(debug_assertions), allow(clippy::let_and_return))]
#[inline]
pub fn link(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    // Call the underlying implementation.
    let result = link_impl(old_start, old_path, new_start, new_path);

    // Do an unsandboxed lookup and check that we found the same result.
    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    match stat_unchecked(new_start, new_path, FollowSymlinks::No) {
        Ok(metadata) => match &result {
            Ok(()) => match stat_unchecked(old_start, old_path, FollowSymlinks::No) {
                Ok(old_metadata) => assert!(metadata.is_same_file(&old_metadata)),
                Err(e) => panic!(
                    "couldn't stat old path after link: start='{:?}' path='{}': {:?}",
                    old_start,
                    old_path.display(),
                    e,
                ),
            },
            Err(e) => match e.kind() {
                io::ErrorKind::AlreadyExists | io::ErrorKind::PermissionDenied => (),
                _ => panic!(
                    "unexpected error opening start='{:?}' path='{}': {:?}",
                    new_start,
                    new_path.display(),
                    e
                ),
            },
        },
        Err(unchecked_error) => match &result {
            Ok(()) => panic!(
                "unexpected success opening start='{:?}', path='{}'; expected {:?}",
                new_start,
                new_path.display(),
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
