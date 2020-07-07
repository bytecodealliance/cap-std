use std::io;

#[cold]
pub(crate) fn rename_path_in_use() -> io::Error {
    io::Error::from_raw_os_error(libc::EBUSY)
}
