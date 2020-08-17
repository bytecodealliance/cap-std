use super::accmode;
use std::{fs, io, os::unix::io::AsRawFd};
use yanix::{fcntl::get_status_flags, file::OFlags};

pub(crate) fn flags_impl(file: &fs::File) -> io::Result<(bool, bool)> {
    let fd = file.as_raw_fd();
    let mode = unsafe { get_status_flags(fd) }?;

    // Check for `O_PATH`.
    // TODO: use yanix's `OFlags::PATH` once it's available.
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "fuchsia",
        target_os = "emscripten"
    ))]
    if mode.contains(OFlags::from_bits(libc::O_PATH).unwrap()) {
        return Ok((false, false));
    }

    match mode & accmode() {
        OFlags::RDONLY => Ok((true, false)),
        OFlags::RDWR => Ok((true, true)),
        OFlags::WRONLY => Ok((false, true)),
        _ => unreachable!(),
    }
}
