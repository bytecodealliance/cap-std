use super::procfs::set_permissions_through_proc_self_fd;
use crate::fs::{open, FollowSymlinks, OpenOptions, Permissions};
use std::{
    fs, io,
    os::unix::{
        fs::{OpenOptionsExt, PermissionsExt},
        io::AsRawFd,
    },
    path::Path,
    sync::atomic::{AtomicBool, Ordering::Relaxed},
};

/// Note that we can't use `fchmodat` because Linux's `fchmodat` system call
/// doesn't support the `AT_SYMLINK_NOFOLLOW` flag. GLIBC and musl have
/// support for emulating that flag, however they do so by relying on `/proc`
/// in a way that trusts that `/proc` is reliable, so we don't use them here.
///
/// In the future, Linux may add an [`fchmodat4`] system call, which would
/// give us a single reliable way to do this.
///
/// [`fchmodat4`]: https://lwn.net/Articles/792628/
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
        // TODO: Here and below, use O_NOCTTY once yanix has it.
        let opath_result = open(
            start,
            path,
            OpenOptions::new()
                .read(true)
                .follow(FollowSymlinks::Yes)
                .custom_flags(libc::O_PATH),
        );

        // If `O_PATH` worked, try to use `fchmod` on it.
        if let Ok(file) = opath_result {
            match file_set_permissions(&file, std_perm.clone()) {
                Ok(()) => return Ok(()),
                Err(e) => match e.raw_os_error() {
                    // If it fails with `EBADF`, `fchmod` didn't like `O_PATH`,
                    // so proceed to the fallback strategies below.
                    Some(libc::EBADF) => FCHMOD_PATH_BADF.store(true, Relaxed),
                    _ => return Err(e),
                },
            }
        }
    }

    // Then try `fchmod` with a normal handle. Normal handles need some kind of
    // access, so first try read.
    match open(
        start,
        path,
        OpenOptions::new().read(true).follow(FollowSymlinks::Yes),
    ) {
        Ok(file) => return file_set_permissions(&file, std_perm),
        Err(e) => match e.raw_os_error() {
            Some(libc::EACCES) => (),
            _ => return Err(e),
        },
    }

    // Next try write.
    match open(
        start,
        path,
        OpenOptions::new().write(true).follow(FollowSymlinks::Yes),
    ) {
        Ok(file) => return file_set_permissions(&file, std_perm),
        Err(e) => match e.raw_os_error() {
            Some(libc::EACCES) | Some(libc::EISDIR) => (),
            _ => return Err(e),
        },
    }

    // If neither of those worked, turn to `/proc`.
    let opath_result = open(
        start,
        path,
        OpenOptions::new()
            .read(true)
            .follow(FollowSymlinks::Yes)
            .custom_flags(libc::O_PATH),
    );
    set_permissions_through_proc_self_fd(&opath_result?, std_perm)
}

/// Like `file.set_permissions(perm)`, but without dependeing on libc's
/// `fchmod`, since some libc implementations such as musl emulate `O_PATH`
/// support by emulating it with /proc.
fn file_set_permissions(file: &fs::File, perm: fs::Permissions) -> io::Result<()> {
    let fd = file.as_raw_fd();
    let mode = perm.mode();
    if unsafe { libc::syscall(libc::SYS_fchmod, fd, mode) } == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}
