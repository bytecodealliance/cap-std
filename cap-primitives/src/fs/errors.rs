use std::io;

pub(crate) use super::errors_impl::*;

#[cold]
pub(crate) fn escape_attempt() -> io::Error {
    io::Error::new(
        io::ErrorKind::PermissionDenied,
        "a path led outside of the filesystem",
    )
}
