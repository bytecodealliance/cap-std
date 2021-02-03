use std::io;
use winapi::shared::winerror;

#[cold]
pub(crate) fn no_such_file_or_directory() -> io::Error {
    io::Error::from_raw_os_error(winerror::ERROR_FILE_NOT_FOUND as i32)
}

#[cold]
pub(crate) fn is_directory() -> io::Error {
    io::Error::from_raw_os_error(winerror::ERROR_DIRECTORY_NOT_SUPPORTED as i32)
}

#[cold]
pub(crate) fn is_not_directory() -> io::Error {
    io::Error::from_raw_os_error(winerror::ERROR_DIRECTORY as i32)
}

#[cold]
pub(crate) fn too_many_symlinks() -> io::Error {
    io::Error::from_raw_os_error(winerror::ERROR_TOO_MANY_LINKS as i32)
}
