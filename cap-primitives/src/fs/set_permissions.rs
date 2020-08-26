//! This defines `set_permissions`, the primary entrypoint to sandboxed
//! filesystem permissions modification.

#[cfg(not(feature = "no_racy_asserts"))]
use crate::fs::{map_result, stat, stat_unchecked, FollowSymlinks, Metadata};
use crate::fs::{set_permissions_impl, Permissions};
use std::{fs, io, path::Path};

/// Perform a `chmodat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(feature = "no_racy_asserts", allow(clippy::let_and_return))]
#[inline]
pub fn set_permissions(start: &fs::File, path: &Path, perm: Permissions) -> io::Result<()> {
    #[cfg(not(feature = "no_racy_asserts"))]
    let perm_clone = perm.clone();

    #[cfg(not(feature = "no_racy_asserts"))]
    let stat_before = stat(start, path, FollowSymlinks::Yes);

    // Call the underlying implementation.
    let result = set_permissions_impl(start, path, perm);

    #[cfg(not(feature = "no_racy_asserts"))]
    let stat_after = stat_unchecked(start, path, FollowSymlinks::Yes);

    #[cfg(not(feature = "no_racy_asserts"))]
    check_set_permissions(start, path, perm_clone, &stat_before, &result, &stat_after);

    result
}

#[cfg(not(feature = "no_racy_asserts"))]
fn check_set_permissions(
    start: &fs::File,
    path: &Path,
    perm: Permissions,
    stat_before: &io::Result<Metadata>,
    result: &io::Result<()>,
    stat_after: &io::Result<Metadata>,
) {
    match (
        map_result(stat_before),
        map_result(result),
        map_result(stat_after),
    ) {
        (Ok(_), Ok(()), Ok(metadata)) => {
            assert_eq!(perm, metadata.permissions());
        }

        (Ok(metadata_before), Err(_), Ok(metadata_after)) => {
            assert_eq!(metadata_before.permissions(), metadata_after.permissions());
        }

        // TODO: Check error messages
        (Err(_), Err(_), Err(_)) => (),

        other => panic!(
            "inconsistent set_permissions checks: start='{:?}' path='{}':\n{:#?}",
            start,
            path.display(),
            other,
        ),
    }
}
