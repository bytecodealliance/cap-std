//! Path canonicalization using `/proc/self/fd`.

use super::procfs::get_path_from_proc_self_fd;
use crate::fs::{canonicalize_manually_and_follow, open_beneath, FollowSymlinks, OpenOptions};
use std::{
    fs, io,
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
};

/// Implement `canonicalize` by using readlink on `/proc/self/fd/*`.
pub(crate) fn canonicalize_impl(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    // Open the path with `O_PATH`. Use `read(true)` even though we don't need
    // `read` permissions, because Rust's libstd requires an access mode, and
    // Linux ignores `O_RDONLY` with `O_PATH`.
    // TODO: Add O_NOCTTY once yanix has it.
    let result = open_beneath(
        start,
        path,
        OpenOptions::new()
            .read(true)
            .follow(FollowSymlinks::Yes)
            .custom_flags(libc::O_PATH),
    );

    // If that worked, call `readlink`.
    match result {
        Ok(file) => {
            if let Ok(start_path) = get_path_from_proc_self_fd(start) {
                if let Ok(file_path) = get_path_from_proc_self_fd(&file) {
                    if let Ok(canonical_path) = file_path.strip_prefix(start_path) {
                        #[cfg(not(feature = "no_racy_asserts"))]
                        assert_eq!(
                            canonical_path,
                            canonicalize_manually_and_follow(start, path).unwrap()
                        );

                        return Ok(canonical_path.to_path_buf());
                    }
                }
            }
        }
        Err(err) => match err.raw_os_error() {
            // `ENOSYS` from `open_beneath` means `openat2` is unavailable
            // and we should use a fallback.
            Some(libc::ENOSYS) => (),
            _ => return Err(err),
        },
    }

    // Use a fallback.
    canonicalize_manually_and_follow(start, path)
}
