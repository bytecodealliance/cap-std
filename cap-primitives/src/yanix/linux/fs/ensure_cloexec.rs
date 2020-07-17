use std::{io, os::unix::io::RawFd};
use yanix::{
    fcntl::{get_fd_flags, set_fd_flags},
    file::FdFlag,
};

// Implementation derived from `ensure_cloexec` in Rust's
// src/libstd/sys/unix/fs.rs at revision
// 7e11379f3b4c376fbb9a6c4d44f3286ccc28d149.

// Currently the standard library supports Linux 2.6.18 which did not
// have the O_CLOEXEC flag (passed above). If we're running on an older
// Linux kernel then the flag is just ignored by the OS. After we open
// the first file, we check whether it has CLOEXEC set. If it doesn't,
// we will explicitly ask for a CLOEXEC fd for every further file we
// open, if it does, we will skip that step.
//
// The CLOEXEC flag, however, is supported on versions of macOS/BSD/etc
// that we support, so we only do this on Linux currently.
pub(crate) unsafe fn ensure_cloexec(fd: RawFd) -> io::Result<()> {
    use std::sync::atomic::{AtomicUsize, Ordering};

    const OPEN_CLOEXEC_UNKNOWN: usize = 0;
    const OPEN_CLOEXEC_SUPPORTED: usize = 1;
    const OPEN_CLOEXEC_NOTSUPPORTED: usize = 2;
    static OPEN_CLOEXEC: AtomicUsize = AtomicUsize::new(OPEN_CLOEXEC_UNKNOWN);

    let need_to_set;
    match OPEN_CLOEXEC.load(Ordering::Relaxed) {
        OPEN_CLOEXEC_UNKNOWN => {
            need_to_set = !get_cloexec(fd)?;
            OPEN_CLOEXEC.store(
                if need_to_set {
                    OPEN_CLOEXEC_NOTSUPPORTED
                } else {
                    OPEN_CLOEXEC_SUPPORTED
                },
                Ordering::Relaxed,
            );
        }
        OPEN_CLOEXEC_SUPPORTED => need_to_set = false,
        OPEN_CLOEXEC_NOTSUPPORTED => need_to_set = true,
        _ => unreachable!(),
    }
    if need_to_set {
        set_cloexec(fd)?;
    }
    Ok(())
}

// Implementation derived from the Linux variants of `get_cloexec` and
// `set_cloexec` in Rust's src/libstd/sys/unix/fd.rs at revision
// 7e11379f3b4c376fbb9a6c4d44f3286ccc28d149.

unsafe fn get_cloexec(fd: RawFd) -> io::Result<bool> {
    Ok(get_fd_flags(fd)?.contains(FdFlag::CLOEXEC))
}

unsafe fn set_cloexec(fd: RawFd) -> io::Result<()> {
    let previous = get_fd_flags(fd)?;
    let new = previous | FdFlag::CLOEXEC;
    if new != previous {
        set_fd_flags(fd, new)?;
    }
    Ok(())
}
