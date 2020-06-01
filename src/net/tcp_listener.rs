use crate::net::{Incoming, SocketAddr, TcpStream};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};
use std::{io, net};

/// A TCP socket server, listening for connections.
///
/// This corresponds to [`std::net::TcpListener`].
///
/// Note that this `TcpListener` has no `bind` method. To bind it to a socket
/// address, you must first obtain a [`Catalog`] permitting the address, and
/// then call [`Catalog::bind_tcp_listener`].
///
/// [`std::net::TcpListener`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html
/// [`Catalog`]: struct.Catalog.html
/// [`Catalog::bind_tcp_listener`]: struct.Catalog.html#method.bind_tcp_listener
pub struct TcpListener {
    tcp_listener: net::TcpListener,
}

impl TcpListener {
    /// Constructs a new instance of `Self` from the given `std::net::TcpListener`.
    pub fn from_ambient(tcp_listener: net::TcpListener) -> Self {
        Self { tcp_listener }
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`std::net::TcpListener::local_addr`].
    ///
    /// [`std::net::TcpListener::local_addr`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.local_addr
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.tcp_listener.local_addr()
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// This corresponds to [`std::net::TcpListener::try_clone`].
    ///
    /// [`std::net::TcpListener::try_clone`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.try_clone
    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self::from_ambient(self.tcp_listener.try_clone()?))
    }

    /// Accept a new incoming connection from this listener.
    ///
    /// This corresponds to [`std::net::TcpListener::accept`].
    ///
    /// [`std::net::TcpListener::accept`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.accept
    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        self.tcp_listener
            .accept()
            .map(|(tcp_stream, addr)| (TcpStream::from_ambient(tcp_stream), addr))
    }

    /// Returns an iterator over the connections being received on this listener.
    ///
    /// This corresponds to [`std::net::TcpListener::incoming`].
    ///
    /// [`std::net::TcpListener::incoming`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.incoming
    pub fn incoming(&self) -> Incoming {
        self.tcp_listener.incoming()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpListener::set_ttl`].
    ///
    /// [`std::net::TcpListener::set_ttl`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.set_ttl
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.tcp_listener.set_ttl(ttl)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This corresponds to [`std::net::TcpListener::ttl`].
    ///
    /// [`std::net::TcpListener::ttl`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.ttl
    pub fn ttl(&self) -> io::Result<u32> {
        self.tcp_listener.ttl()
    }

    /// Gets the value of the `SO_ERROR` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpListener::take_error`].
    ///
    /// [`std::net::TcpListener::take_error`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.take_error
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.tcp_listener.take_error()
    }

    /// Moves this TCP stream into or out of nonblocking mode.
    ///
    /// This corresponds to [`std::net::TcpListener::set_nonblocking`].
    ///
    /// [`std::net::TcpListener::set_nonblocking`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.set_nonblocking
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.tcp_listener.set_nonblocking(nonblocking)
    }
}

#[cfg(unix)]
impl FromRawFd for TcpListener {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_ambient(net::TcpListener::from_raw_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawSocket for TcpListener {
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        Self::from_ambient(net::TcpListener::from_raw_socket(handle))
    }
}

#[cfg(unix)]
impl AsRawFd for TcpListener {
    fn as_raw_fd(&self) -> RawFd {
        self.tcp_listener.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawSocket for TcpListener {
    fn as_raw_socket(&self) -> RawSocket {
        self.tcp_listener.as_raw_socket()
    }
}

#[cfg(unix)]
impl IntoRawFd for TcpListener {
    fn into_raw_fd(self) -> RawFd {
        self.tcp_listener.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawHandle for TcpListener {
    fn into_raw_handle(self) -> RawHandle {
        self.tcp_listener.into_raw_handle()
    }
}

// TODO: impl Debug for TcpListener?
