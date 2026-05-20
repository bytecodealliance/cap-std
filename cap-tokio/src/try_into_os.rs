//! Fallible ownership-transfer traits for file descriptors/handles.
//!
//! These mirror [`std::os::unix::io::IntoRawFd`] and friends, but return
//! `Result` because tokio types must deregister from the reactor before
//! yielding the raw fd, and that deregistration can fail.

use std::io;

/// Fallible version of [`std::os::unix::io::IntoRawFd`].
///
/// Tokio types register with the async reactor; converting to a raw fd
/// requires deregistration, which can fail.
#[cfg(unix)]
pub trait TryIntoRawFd: Sized {
    /// Consume this object, returning the raw underlying file descriptor.
    fn try_into_raw_fd(self) -> io::Result<std::os::unix::io::RawFd>;
}

/// Fallible version of [`std::os::windows::io::IntoRawHandle`].
#[cfg(windows)]
pub trait TryIntoRawHandle: Sized {
    /// Consume this object, returning the raw underlying handle.
    fn try_into_raw_handle(self) -> io::Result<std::os::windows::io::RawHandle>;
}

/// Fallible version of [`std::os::windows::io::IntoRawSocket`].
#[cfg(windows)]
pub trait TryIntoRawSocket: Sized {
    /// Consume this object, returning the raw underlying socket.
    fn try_into_raw_socket(self) -> io::Result<std::os::windows::io::RawSocket>;
}

/// Fallible version of [`io_extras::os::windows::IntoRawHandleOrSocket`].
#[cfg(windows)]
pub trait TryIntoRawHandleOrSocket: Sized {
    /// Consume this object, returning the raw underlying handle or socket.
    fn try_into_raw_handle_or_socket(self)
        -> io::Result<io_extras::os::windows::RawHandleOrSocket>;
}
