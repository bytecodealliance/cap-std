//! This defines `symlink`, the primary entrypoint to sandboxed symlink creation.

#[cfg(not(feature = "no_racy_asserts"))]
#[cfg(any(
    unix,
    target_os = "fuchsia",
    target_os = "redox",
    target_os = "vxworks"
))]
use crate::fs::symlink_unchecked;
#[cfg(not(feature = "no_racy_asserts"))]
use crate::fs::{canonicalize, canonicalize_manually, stat_unchecked, FollowSymlinks, Metadata};
use std::{fs, io, path::Path};

/// Perform a `symlinkat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(feature = "no_racy_asserts", allow(clippy::let_and_return))]
#[cfg(any(
    unix,
    target_os = "fuchsia",
    target_os = "redox",
    target_os = "vxworks"
))]
#[inline]
pub fn symlink(old_path: &Path, new_start: &fs::File, new_path: &Path) -> io::Result<()> {
    use crate::fs::symlink_impl;

    #[cfg(not(feature = "no_racy_asserts"))]
    let stat_before = stat_unchecked(new_start, new_path, FollowSymlinks::No);

    // Call the underlying implementation.
    let result = symlink_impl(old_path, new_start, new_path);

    #[cfg(not(feature = "no_racy_asserts"))]
    let stat_after = stat_unchecked(new_start, new_path, FollowSymlinks::No);

    #[cfg(not(feature = "no_racy_asserts"))]
    check_symlink(
        old_path,
        new_start,
        new_path,
        &stat_before,
        &result,
        &stat_after,
    );

    result
}

#[cfg(not(feature = "no_racy_asserts"))]
#[cfg(any(
    unix,
    target_os = "fuchsia",
    target_os = "redox",
    target_os = "vxworks"
))]
#[allow(clippy::enum_glob_use)]
fn check_symlink(
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
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
        (Err((NotFound, _)), Ok(()), Ok(metadata)) => {
            assert!(metadata.file_type().is_symlink());
            let canon = canonicalize_manually(new_start, new_path, FollowSymlinks::No).unwrap();
            assert!(stat_unchecked(new_start, &canon, FollowSymlinks::No)
                .unwrap()
                .is_same_file(&metadata));
        }

        (Ok(metadata_before), Err((AlreadyExists, _)), Ok(metadata_after)) => {
            assert!(metadata_before.is_same_file(&metadata_after));
        }

        (_, Err((_kind, _message)), _) => match map_result(&canonicalize(new_start, new_path)) {
            Ok(canon) => match map_result(&symlink_unchecked(old_path, new_start, &canon)) {
                Err((_unchecked_kind, _unchecked_message)) => {
                    /* TODO: Check error messages.
                    assert_eq!(
                        kind,
                        unchecked_kind,
                        "unexpected error kind from symlink new_start='{:?}', \
                         new_path='{}':\nstat_before={:#?}\nresult={:#?}\nstat_after={:#?}",
                        new_start,
                        new_path.display(),
                        stat_before,
                        result,
                        stat_after
                    );
                    assert_eq!(message, unchecked_message);
                    */
                }
                _ => panic!("unsandboxed symlink success"),
            },
            Err((_canon_kind, _canon_message)) => {
                /* TODO: Check error messages.
                assert_eq!(kind, canon_kind);
                assert_eq!(message, canon_message);
                */
            }
        },

        _other => {
            /* TODO: Check error messages.
            panic!(
                "inconsistent symlink checks: new_start='{:?}' new_path='{}':\n{:#?}",
                new_start,
                new_path.display(),
                other,
            )
            */
        }
    }
}

/// Perform a `symlink_file`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(feature = "no_racy_asserts", allow(clippy::let_and_return))]
#[cfg(windows)]
#[inline]
pub fn symlink_file(old_path: &Path, new_start: &fs::File, new_path: &Path) -> io::Result<()> {
    use crate::fs::symlink_file_impl;
    todo!("symlink_file")
}

/// Perform a `symlink_dir`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(feature = "no_racy_asserts", allow(clippy::let_and_return))]
#[cfg(windows)]
#[inline]
pub fn symlink_dir(old_path: &Path, new_start: &fs::File, new_path: &Path) -> io::Result<()> {
    use crate::fs::symlink_dir_impl;
    todo!("symlink_dir")
}
