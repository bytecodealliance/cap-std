//! This defines `symlink`, the primary entrypoint to sandboxed symlink creation.

#[cfg(debug_assertions)]
use crate::fs::{stat_unchecked, FollowSymlinks};
use std::{fs, io, path::Path};

/// Perform a `symlinkat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(not(debug_assertions), allow(clippy::let_and_return))]
#[cfg(any(
    unix,
    target_os = "fuchsia",
    target_os = "redox",
    target_os = "vxworks"
))]
#[inline]
pub fn symlink(old_path: &Path, new_start: &fs::File, new_path: &Path) -> io::Result<()> {
    use crate::fs::symlink_impl;
    // Call the underlying implementation.
    let result = symlink_impl(old_path, new_start, new_path);

    // Do an unsandboxed lookup and check that we found the same result.
    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    match stat_unchecked(new_start, new_path, FollowSymlinks::No) {
        Ok(metadata) => match &result {
            Ok(()) => debug_assert!(metadata.file_type().is_symlink()),
            Err(e) => match e.kind() {
                io::ErrorKind::AlreadyExists | io::ErrorKind::PermissionDenied => (),
                _ => panic!(
                    "unexpected error opening start='{:?}', path='{}': {:?}",
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

/// Perform a `symlink_file`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(not(debug_assertions), allow(clippy::let_and_return))]
#[cfg(windows)]
#[inline]
pub fn symlink_file(old_path: &Path, new_start: &fs::File, new_path: &Path) -> io::Result<()> {
    use crate::fs::symlink_file_impl;
    todo!("symlink_file")
}

/// Perform a `symlink_dir`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(not(debug_assertions), allow(clippy::let_and_return))]
#[cfg(windows)]
#[inline]
pub fn symlink_dir(old_path: &Path, new_start: &fs::File, new_path: &Path) -> io::Result<()> {
    use crate::fs::symlink_dir_impl;
    todo!("symlink_dir")
}
