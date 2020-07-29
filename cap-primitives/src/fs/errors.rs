use std::io;

cfg_if::cfg_if! {
    if #[cfg(any(unix, target_os = "fuchsia"))] {
        pub(crate) use crate::yanix::fs::errors::*;
    } else if #[cfg(windows)] {
        pub(crate) use crate::winx::fs::errors::*;
    }
}

#[cold]
pub(crate) fn escape_attempt() -> io::Error {
    io::Error::new(
        io::ErrorKind::PermissionDenied,
        "a path led outside of the filesystem",
    )
}
