use io_lifetimes::BorrowedFd;
use posish::fs::{fcntl_getfd, fcntl_setfd, FdFlags};
use std::io;

// Implementation derived from `ensure_cloexec` in Rust's
// library/std/src/sys/unix/fs.rs at revision
// 108e90ca78f052c0c1c49c42a22c85620be19712.

// Currently the standard library supports Linux 2.6.18 which did not have the
// `O_CLOEXEC` flag (passed above). If we're running on an older Linux kernel
// then the flag is just ignored by the OS. After we open the first file, we
// check whether it has CLOEXEC set. If it doesn't, we will explicitly ask for
// a CLOEXEC fd for every further file we open, if it does, we will skip that
// step.
//
// The CLOEXEC flag, however, is supported on versions of macOS/BSD/etc
// that we support, so we only do this on Linux currently.
pub(crate) fn ensure_cloexec(fd: BorrowedFd<'_>) -> io::Result<()> {
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
// `set_cloexec` in Rust's library/std/src/sys/unix/fd.rs at revision
// 108e90ca78f052c0c1c49c42a22c85620be19712.

fn get_cloexec(fd: BorrowedFd<'_>) -> io::Result<bool> {
    Ok(fcntl_getfd(&fd)?.contains(FdFlags::CLOEXEC))
}

fn set_cloexec(fd: BorrowedFd<'_>) -> io::Result<()> {
    let previous = fcntl_getfd(&fd)?;
    let new = previous | FdFlags::CLOEXEC;
    if new != previous {
        fcntl_setfd(&fd, new)?;
    }
    Ok(())
}
