use std::io;

#[cold]
pub(crate) fn no_such_file_or_directory() -> io::Error {
    todo!("no_such_file_or_directory")
}

#[cold]
pub(crate) fn is_directory() -> io::Error {
    todo!("is_directory")
}

#[cold]
pub(crate) fn is_not_directory() -> io::Error {
    todo!("is_not_directory")
}

#[cold]
pub(crate) fn escape_attempt() -> io::Error {
    todo!("escape_attempt")
}

#[cold]
pub(crate) fn too_many_symlinks() -> io::Error {
    io::Error::from_raw_os_error(winapi::shared::winerror::ERROR_TOO_MANY_LINKS as i32)
}
