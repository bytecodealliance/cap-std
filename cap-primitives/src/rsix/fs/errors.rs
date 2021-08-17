use std::io;

#[cold]
pub(crate) fn invalid_flags() -> io::Error {
    rsix::io::Error::INVAL.into()
}

#[cold]
pub(crate) fn no_such_file_or_directory() -> io::Error {
    rsix::io::Error::NOENT.into()
}

#[cold]
pub(crate) fn is_directory() -> io::Error {
    rsix::io::Error::ISDIR.into()
}

#[cold]
pub(crate) fn is_not_directory() -> io::Error {
    rsix::io::Error::NOTDIR.into()
}

#[cold]
pub(crate) fn too_many_symlinks() -> io::Error {
    rsix::io::Error::LOOP.into()
}
