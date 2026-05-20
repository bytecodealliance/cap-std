use crate::os::unix::net::{SocketAddr, UnixStream};
use crate::try_into_os::*;
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
use std::fmt;
use std::io;
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};
use tokio::net;

/// A structure representing a Unix domain socket server.
///
/// This corresponds to [`tokio::net::UnixListener`].
///
/// This `UnixListener` has no `bind` method. To bind it to a socket address,
/// first obtain a [`Dir`] containing the path, and then call
/// [`Dir::bind_unix_listener`].
///
/// [`Dir`]: struct.Dir.html
/// [`Dir::bind_unix_listener`]: struct.Dir.html#method.bind_unix_listener
pub struct UnixListener {
    std: net::UnixListener,
}

impl UnixListener {
    /// Constructs a new instance of `Self` from the given
    /// `tokio::net::UnixListener`.
    ///
    /// This grants access the resources the
    /// `tokio::net::UnixListener` instance already has access
    /// to.
    #[inline]
    pub fn from_std(std: net::UnixListener) -> Self {
        Self { std }
    }

    /// Accepts a new incoming connection to this listener.
    ///
    /// This corresponds to [`tokio::net::UnixListener::accept`].
    #[inline]
    pub async fn accept(&self) -> io::Result<(UnixStream, SocketAddr)> {
        self.std
            .accept()
            .await
            .map(|(unix_stream, addr)| (UnixStream::from_std(unix_stream), addr))
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to
    /// [`tokio::net::UnixListener::local_addr`].
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }
}

impl TryFrom<RawFd> for UnixListener {
    type Error = io::Error;
    #[inline]
    fn try_from(fd: RawFd) -> io::Result<Self> {
        use std::os::unix::io::FromRawFd;
        let std_listener = unsafe { std::os::unix::net::UnixListener::from_raw_fd(fd) };
        std_listener.set_nonblocking(true)?;
        Ok(Self::from_std(net::UnixListener::from_std(std_listener)?))
    }
}

impl TryFrom<OwnedFd> for UnixListener {
    type Error = io::Error;
    #[inline]
    fn try_from(fd: OwnedFd) -> io::Result<Self> {
        let std_listener = std::os::unix::net::UnixListener::from(fd);
        std_listener.set_nonblocking(true)?;
        Ok(Self::from_std(net::UnixListener::from_std(std_listener)?))
    }
}

impl AsRawFd for UnixListener {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

impl AsFd for UnixListener {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std.as_fd()
    }
}

impl TryIntoRawFd for UnixListener {
    fn try_into_raw_fd(self) -> std::io::Result<RawFd> {
        Ok(self.std.into_std()?.into_raw_fd())
    }
}

impl TryFrom<UnixListener> for OwnedFd {
    type Error = io::Error;
    #[inline]
    fn try_from(val: UnixListener) -> io::Result<OwnedFd> {
        use std::os::unix::io::FromRawFd;
        let raw = TryIntoRawFd::try_into_raw_fd(val)?;
        Ok(unsafe { OwnedFd::from_raw_fd(raw) })
    }
}

impl fmt::Debug for UnixListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
