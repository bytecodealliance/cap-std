use std::{fs, io, os::unix::io::AsRawFd};

pub(crate) fn flags_impl(file: &fs::File) -> io::Result<(bool, bool)> {
    let mode = unsafe { yanix::fcntl::get_status_flags(file.as_raw_fd()) }?;
    match mode & yanix::file::OFlags::ACCMODE {
        yanix::file::OFlags::RDONLY => Ok((true, false)),
        yanix::file::OFlags::RDWR => Ok((true, true)),
        yanix::file::OFlags::WRONLY => Ok((false, true)),
        _ => unreachable!(),
    }
}
