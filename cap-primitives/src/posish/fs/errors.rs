use posish::io::Errno;
use std::io;

#[cold]
pub(crate) fn invalid_flags() -> io::Error {
    Errno::INVAL.io_error()
}

#[cold]
pub(crate) fn no_such_file_or_directory() -> io::Error {
    Errno::NOENT.io_error()
}

#[cold]
pub(crate) fn is_directory() -> io::Error {
    Errno::ISDIR.io_error()
}

#[cold]
pub(crate) fn is_not_directory() -> io::Error {
    Errno::NOTDIR.io_error()
}

#[cold]
pub(crate) fn too_many_symlinks() -> io::Error {
    Errno::LOOP.io_error()
}
