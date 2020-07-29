use std::io;

#[cold]
pub(crate) fn no_such_file_or_directory() -> io::Error {
    io::Error::from_raw_os_error(libc::ENOENT)
}

#[cold]
pub(crate) fn is_directory() -> io::Error {
    io::Error::from_raw_os_error(libc::EISDIR)
}

#[cold]
pub(crate) fn is_not_directory() -> io::Error {
    io::Error::from_raw_os_error(libc::ENOTDIR)
}

#[cold]
pub(crate) fn too_many_symlinks() -> io::Error {
    io::Error::from_raw_os_error(libc::ELOOP)
}
