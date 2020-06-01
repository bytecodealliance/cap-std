use crate::net::Shutdown;
use crate::os::unix::net::SocketAddr;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::{io, os::unix, time::Duration};

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
    unix_datagram: unix::net::UnixDatagram,
}

impl UnixDatagram {
    /// Constructs a new instance of `Self` from the given `std::os::unix::net::UnixDatagram`.
    pub fn from_ambient(unix_datagram: unix::net::UnixDatagram) -> Self {
        Self { unix_datagram }
    }

    /// Creates a Unix Datagram socket which is not bound to any address.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::unbound`].
    ///
    /// TODO: should this require a capability?
    ///
    /// [`std::os::unix::net::UnixDatagram::unbound`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.unbound
    pub fn unbound() -> io::Result<Self> {
        unix::net::UnixDatagram::unbound().map(Self::from_ambient)
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::pair`].
    ///
    /// TODO: should this require a capability?
    ///
    /// [`std::os::unix::net::UnixDatagram::pair`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.pair
    pub fn pair() -> io::Result<(Self, Self)> {
        unix::net::UnixDatagram::pair().map(|(a, b)| (Self::from_ambient(a), Self::from_ambient(b)))
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::try_clone`].
    ///
    /// [`std::os::unix::net::UnixDatagram::try_clone`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.try_clone
    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self::from_ambient(self.unix_datagram.try_clone()?))
    }

    /// Returns the address of this socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::local_addr`].
    ///
    /// [`std::os::unix::net::UnixDatagram::local_addr`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.local_addr
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.unix_datagram.local_addr()
    }

    /// Returns the address of this socket's peer.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::peer_addr`].
    ///
    /// [`std::os::unix::net::UnixDatagram::peer_addr`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.peer_addr
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.unix_datagram.peer_addr()
    }

    /// Receives data from the socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::recv_from`].
    ///
    /// [`std::os::unix::net::UnixDatagram::recv_from`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.recv_from
    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.unix_datagram.recv_from(buf)
    }

    /// Receives data from the socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::recv`].
    ///
    /// [`std::os::unix::net::UnixDatagram::recv`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.recv
    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.unix_datagram.recv(buf)
    }

    /// Sends data on the socket to the socket's peer.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::send`].
    ///
    /// [`std::os::unix::net::UnixDatagram::send`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.send
    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.unix_datagram.send(buf)
    }

    /// Sets the read timeout for the socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::set_read_timeout`].
    ///
    /// [`std::os::unix::net::UnixDatagram::set_read_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.set_read_timeout
    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.unix_datagram.set_read_timeout(timeout)
    }

    /// Sets the write timeout for the socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::set_write_timeout`].
    ///
    /// [`std::os::unix::net::UnixDatagram::set_write_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.set_write_timeout
    pub fn set_write_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.unix_datagram.set_write_timeout(timeout)
    }

    /// Returns the read timeout of this socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::read_timeout`].
    ///
    /// [`std::os::unix::net::UnixDatagram::read_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.read_timeout
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.unix_datagram.read_timeout()
    }

    /// Returns the write timeout of this socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::write_timeout`].
    ///
    /// [`std::os::unix::net::UnixDatagram::write_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.write_timeout
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.unix_datagram.write_timeout()
    }

    /// Moves the socket into or out of nonblocking mode.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::set_nonblocking`].
    ///
    /// [`std::os::unix::net::UnixDatagram::set_nonblocking`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.set_nonblocking
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.unix_datagram.set_nonblocking(nonblocking)
    }

    /// Returns the value of the `SO_ERROR` option.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::take_error`].
    ///
    /// [`std::os::unix::net::UnixDatagram::take_error`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.take_error
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.unix_datagram.take_error()
    }

    /// Shut down the read, write, or both halves of this connection.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::shutdown`].
    ///
    /// [`std::os::unix::net::UnixDatagram::shutdown`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.shutdown
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.unix_datagram.shutdown(how)
    }
}

impl FromRawFd for UnixDatagram {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_ambient(unix::net::UnixDatagram::from_raw_fd(fd))
    }
}

impl AsRawFd for UnixDatagram {
    fn as_raw_fd(&self) -> RawFd {
        self.unix_datagram.as_raw_fd()
    }
}

impl IntoRawFd for UnixDatagram {
    fn into_raw_fd(self) -> RawFd {
        self.unix_datagram.into_raw_fd()
    }
}

// TODO: impl Debug for UnixDatagram?
