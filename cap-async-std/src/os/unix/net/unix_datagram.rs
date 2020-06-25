use crate::{net::Shutdown, os::unix::net::SocketAddr};
use async_std::{
    io,
    os::{
        unix,
        unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd},
    },
};

/// A Unix datagram socket.
///
/// This corresponds to [`std::os::unix::net::UnixDatagram`].
///
/// Note that this `UnixDatagram` has no `bind`, `connect`, or `send_to`
/// methods. To create a `UnixDatagram`,
/// you must first obtain a [`Dir`] containing the path, and then call
/// [`Dir::bind_unix_datagram`], [`Dir::connect_unix_datagram`], or
/// [`Dir::send_to_unix_datagram_addr`].
///
/// [`std::os::unix::net::UnixDatagram`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html
/// [`Dir`]: struct.Dir.html
/// [`Dir::connect_unix_datagram`]: struct.Dir.html#method.connect_unix_datagram
/// [`Dir::bind_unix_datagram`]: struct.Dir.html#method.bind_unix_datagram
/// [`Dir::send_to_unix_datagram_addr`]: struct.Dir.html#method.send_to_unix_datagram_addr
pub struct UnixDatagram {
    std: unix::net::UnixDatagram,
}

impl UnixDatagram {
    /// Constructs a new instance of `Self` from the given `async_std::os::unix::net::UnixDatagram`.
    #[inline]
    pub fn from_std(std: unix::net::UnixDatagram) -> Self {
        Self { std }
    }

    /// Creates a Unix Datagram socket which is not bound to any address.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::unbound`].
    ///
    /// TODO: should this require a capability?
    ///
    /// [`std::os::unix::net::UnixDatagram::unbound`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.unbound
    #[inline]
    pub fn unbound() -> io::Result<Self> {
        unix::net::UnixDatagram::unbound().map(Self::from_std)
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::pair`].
    ///
    /// TODO: should this require a capability?
    ///
    /// [`std::os::unix::net::UnixDatagram::pair`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.pair
    #[inline]
    pub fn pair() -> io::Result<(Self, Self)> {
        unix::net::UnixDatagram::pair().map(|(a, b)| (Self::from_std(a), Self::from_std(b)))
    }

    // async_std doesn't have `try_clone`.

    /// Returns the address of this socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::local_addr`].
    ///
    /// [`std::os::unix::net::UnixDatagram::local_addr`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.local_addr
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Returns the address of this socket's peer.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::peer_addr`].
    ///
    /// [`std::os::unix::net::UnixDatagram::peer_addr`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.peer_addr
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.std.peer_addr()
    }

    /// Receives data from the socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::recv_from`].
    ///
    /// [`std::os::unix::net::UnixDatagram::recv_from`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.recv_from
    #[inline]
    pub async fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.std.recv_from(buf).await
    }

    /// Receives data from the socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::recv`].
    ///
    /// [`std::os::unix::net::UnixDatagram::recv`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.recv
    #[inline]
    pub async fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.recv(buf).await
    }

    /// Sends data on the socket to the socket's peer.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::send`].
    ///
    /// [`std::os::unix::net::UnixDatagram::send`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.send
    #[inline]
    pub async fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.std.send(buf).await
    }

    // async_std doesn't have `set_read_timeout`.

    // async_std doesn't have `set_write_timeout`.

    // async_std doesn't have `read_timeout`.

    // async_std doesn't have `write_timeout`.

    // async_std doesn't have `set_nonblocking`.

    // async_std doesn't have `take_error`.

    /// Shut down the read, write, or both halves of this connection.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::shutdown`].
    ///
    /// [`std::os::unix::net::UnixDatagram::shutdown`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.shutdown
    #[inline]
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.std.shutdown(how)
    }
}

impl FromRawFd for UnixDatagram {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(unix::net::UnixDatagram::from_raw_fd(fd))
    }
}

impl AsRawFd for UnixDatagram {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

impl IntoRawFd for UnixDatagram {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

// TODO: impl Debug for UnixDatagram?
