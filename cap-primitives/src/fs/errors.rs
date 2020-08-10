use std::io;

#[cfg(windows)]
pub(crate) use crate::winx::fs::errors::*;
#[cfg(not(windows))]
pub(crate) use crate::yanix::fs::errors::*;

#[cold]
pub(crate) fn escape_attempt() -> io::Error {
    io::Error::new(
        io::ErrorKind::PermissionDenied,
        "a path led outside of the filesystem",
    )
}
