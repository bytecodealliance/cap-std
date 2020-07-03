//! This defines `rename`, the primary entrypoint to sandboxed renaming.

use crate::fs::rename_impl;
#[cfg(debug_assertions)]
use crate::fs::{get_path, stat_unchecked, FollowSymlinks};
use std::{fs, io, path::Path};

/// Perform a `renameat`-like operation, ensuring that the resolution of both
/// the old and new paths never escape the directory tree rooted at their
/// respective starts.
#[cfg_attr(not(debug_assertions), allow(clippy::let_and_return))]
#[inline]
pub fn rename(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    #[cfg(debug_assertions)]
    let orig_metadata = stat_unchecked(old_start, old_path, FollowSymlinks::No);

    // Call the underlying implementation.
    let result = rename_impl(old_start, old_path, new_start, new_path);

    // Do an unsandboxed lookup and check that we found the same result.
    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    match stat_unchecked(new_start, new_path, FollowSymlinks::No) {
        Ok(new_metadata) => match &result {
            Ok(()) => assert!(orig_metadata.unwrap().is_same_file(&new_metadata)),
            Err(_) => (),
        },
        Err(unchecked_error) => match &result {
            Ok(()) => panic!(
                "unexpected success opening start='{:?}', path='{}'; expected {:?}",
                get_path(new_start),
                new_path.display(),
                unchecked_error
            ),
            Err(_) => (),
        },
    }

    result
}
