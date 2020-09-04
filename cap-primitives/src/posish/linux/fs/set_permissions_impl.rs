use super::procfs::set_permissions_through_proc_self_fd;
use crate::fs::{errors, open, OpenOptions, Permissions};
use posish::fs::{fchmod, Mode};
use std::{
    fs, io,
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    path::Path,
    sync::atomic::{AtomicBool, Ordering::Relaxed},
};

pub(crate) fn set_permissions_impl(
    start: &fs::File,
    path: &Path,
    perm: Permissions,
) -> io::Result<()> {
    // Record whether we've seen an `EBADF` from an `fchmod` on an `O_PATH`
    // file descriptor, meaning we're on a Linux that doesn't support it.
    static FCHMOD_PATH_BADF: AtomicBool = AtomicBool::new(false);

    let std_perm = perm.into_std(start)?;

    if !FCHMOD_PATH_BADF.load(Relaxed) {
        // First try to open the path with `O_PATH`; if that succeeds, it'll give
        // us a few options. Use `read(true)` even though we don't need `read`
        // permissions, because Rust's libstd requires an access mode, and Linux
        // ignores `O_RDONLY` with `O_PATH`.
        // TODO: Add tests with no-read no-permissions.
        let opath_result = open(
            start,
            path,
            OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_PATH | libc::O_NOCTTY),
        );

        // If `O_PATH` worked, try to use `fchmod` on it.
        if let Ok(file) = opath_result {
            match set_file_permissions(&file, std_perm.clone()) {
                Ok(()) => return Ok(()),
                Err(err) => match err.raw_os_error() {
                    // If it fails with `EBADF`, `fchmod` didn't like `O_PATH`,
                    // so proceed to the fallback strategies below.
                    Some(libc::EBADF) => FCHMOD_PATH_BADF.store(true, Relaxed),
                    _ => return Err(err),
                },
            }
        }
    }

    // Then try `fchmod` with a normal handle. Normal handles need some kind of
    // access, so first try read.
    match open(start, path, OpenOptions::new().read(true)) {
        Ok(file) => return set_file_permissions(&file, std_perm),
        Err(err) => match err.raw_os_error() {
            Some(libc::EACCES) => (),
            _ => return Err(err),
        },
    }

    // Next try write.
    match open(start, path, OpenOptions::new().write(true)) {
        Ok(file) => return set_file_permissions(&file, std_perm),
        Err(err) => match err.raw_os_error() {
            Some(libc::EACCES) | Some(libc::EISDIR) => (),
            _ => return Err(err),
        },
    }

    // If neither of those worked, turn to `/proc`.
    set_permissions_through_proc_self_fd(start, path, std_perm)
}

/// Like `file.set_permissions(perm)`, but without dependeing on libc's
/// `fchmod`, since some libc implementations such as musl emulate `O_PATH`
/// support by emulating it with /proc.
fn set_file_permissions(file: &fs::File, perm: fs::Permissions) -> io::Result<()> {
    let mode = Mode::from_bits(perm.mode()).ok_or_else(errors::invalid_flags)?;
    fchmod(file, mode)
}
