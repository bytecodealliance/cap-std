use crate::os::unix::net::SocketAddr;
use crate::try_into_os::*;
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
use std::fmt;
use std::io;
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};
use tokio::net;

/// A Unix datagram socket.
///
/// This corresponds to [`tokio::net::UnixDatagram`].
///
/// This `UnixDatagram` has no `bind`, `connect`, or `send_to` methods. To
/// create a `UnixDatagram`, first obtain a [`Dir`] containing the path, and
/// then call [`Dir::bind_unix_datagram`], [`Dir::connect_unix_datagram`], or
/// [`Dir::send_to_unix_datagram_addr`].
///
/// [`Dir`]: struct.Dir.html
/// [`Dir::connect_unix_datagram`]: struct.Dir.html#method.connect_unix_datagram
/// [`Dir::bind_unix_datagram`]: struct.Dir.html#method.bind_unix_datagram
/// [`Dir::send_to_unix_datagram_addr`]: struct.Dir.html#method.send_to_unix_datagram_addr
pub struct UnixDatagram {
    std: net::UnixDatagram,
}

impl UnixDatagram {
    /// Constructs a new instance of `Self` from the given
    /// `tokio::net::UnixDatagram`.
    ///
    /// This grants access the resources the
    /// `tokio::net::UnixDatagram` instance already has access
    /// to.
    #[inline]
    pub fn from_std(std: net::UnixDatagram) -> Self {
        Self { std }
    }

    /// Creates a Unix Datagram socket which is not bound to any address.
    ///
    /// This corresponds to
    /// [`tokio::net::UnixDatagram::unbound`].
    ///
    /// TODO: should this require a capability?
    #[inline]
    pub fn unbound() -> io::Result<Self> {
        let unix_datagram = net::UnixDatagram::unbound()?;
        Ok(Self::from_std(unix_datagram))
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// This corresponds to [`tokio::net::UnixDatagram::pair`].
    ///
    /// TODO: should this require a capability?
    #[inline]
    pub fn pair() -> io::Result<(Self, Self)> {
        net::UnixDatagram::pair().map(|(a, b)| (Self::from_std(a), Self::from_std(b)))
    }

    /// Returns the address of this socket.
    ///
    /// This corresponds to
    /// [`tokio::net::UnixDatagram::local_addr`].
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Returns the address of this socket's peer.
    ///
    /// This corresponds to
    /// [`tokio::net::UnixDatagram::peer_addr`].
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.std.peer_addr()
    }

    /// Receives data from the socket.
    ///
    /// This corresponds to
    /// [`tokio::net::UnixDatagram::recv_from`].
    #[inline]
    pub async fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.std.recv_from(buf).await
    }

    /// Receives data from the socket.
    ///
    /// This corresponds to [`tokio::net::UnixDatagram::recv`].
    #[inline]
    pub async fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.recv(buf).await
    }

    /// Sends data on the socket to the socket's peer.
    ///
    /// This corresponds to [`tokio::net::UnixDatagram::send`].
    #[inline]
    pub async fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.std.send(buf).await
    }

    /// Shut down the read, write, or both halves of this connection.
    ///
    /// This corresponds to
    /// [`tokio::net::UnixDatagram::shutdown`].
    #[inline]
    pub fn shutdown(&self, how: std::net::Shutdown) -> io::Result<()> {
        self.std.shutdown(how)
    }
}

impl TryFrom<RawFd> for UnixDatagram {
    type Error = io::Error;
    #[inline]
    fn try_from(fd: RawFd) -> io::Result<Self> {
        use std::os::unix::io::FromRawFd;
        let std_datagram = unsafe { std::os::unix::net::UnixDatagram::from_raw_fd(fd) };
        std_datagram.set_nonblocking(true)?;
        Ok(Self::from_std(net::UnixDatagram::from_std(std_datagram)?))
    }
}

impl TryFrom<OwnedFd> for UnixDatagram {
    type Error = io::Error;
    #[inline]
    fn try_from(fd: OwnedFd) -> io::Result<Self> {
        let std_datagram = std::os::unix::net::UnixDatagram::from(fd);
        std_datagram.set_nonblocking(true)?;
        Ok(Self::from_std(net::UnixDatagram::from_std(std_datagram)?))
    }
}

impl AsRawFd for UnixDatagram {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

impl AsFd for UnixDatagram {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std.as_fd()
    }
}

impl TryIntoRawFd for UnixDatagram {
    fn try_into_raw_fd(self) -> std::io::Result<RawFd> {
        Ok(self.std.into_std()?.into_raw_fd())
    }
}

impl TryFrom<UnixDatagram> for OwnedFd {
    type Error = io::Error;
    #[inline]
    fn try_from(val: UnixDatagram) -> io::Result<OwnedFd> {
        use std::os::unix::io::FromRawFd;
        let raw = TryIntoRawFd::try_into_raw_fd(val)?;
        Ok(unsafe { OwnedFd::from_raw_fd(raw) })
    }
}

impl fmt::Debug for UnixDatagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
