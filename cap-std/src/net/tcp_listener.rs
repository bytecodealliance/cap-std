use crate::net::{Incoming, SocketAddr, TcpStream};
use cap_primitives::{ambient_authority, AmbientAuthority};
use std::{fmt, io, net};
#[cfg(not(windows))]
use unsafe_io::os::posish::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use unsafe_io::OwnsRaw;
#[cfg(windows)]
use {
    std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket, RawSocket},
    unsafe_io::os::windows::{AsRawHandleOrSocket, IntoRawHandleOrSocket, RawHandleOrSocket},
};

/// A TCP socket server, listening for connections.
///
/// This corresponds to [`std::net::TcpListener`].
///
/// Note that this `TcpListener` has no `bind` method. To bind it to a socket
/// address, you must first obtain a [`Pool`] permitting the address, and
/// then call [`Pool::bind_tcp_listener`].
///
/// [`Pool`]: struct.Pool.html
/// [`Pool::bind_tcp_listener`]: struct.Pool.html#method.bind_tcp_listener
pub struct TcpListener {
    std: net::TcpListener,
}

impl TcpListener {
    /// Constructs a new instance of `Self` from the given `std::net::TcpListener`.
    ///
    /// # Ambient Authority
    ///
    /// `std::net::TcpListener` is not sandboxed and may access any address that the host
    /// process has access to.
    #[inline]
    pub fn from_std(std: net::TcpListener, _: AmbientAuthority) -> Self {
        Self { std }
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`std::net::TcpListener::local_addr`].
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// This corresponds to [`std::net::TcpListener::try_clone`].
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        let tcp_listener = self.std.try_clone()?;
        Ok(Self::from_std(tcp_listener, ambient_authority()))
    }

    /// Accept a new incoming connection from this listener.
    ///
    /// This corresponds to [`std::net::TcpListener::accept`].
    #[inline]
    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        self.std
            .accept()
            .map(|(tcp_stream, addr)| (TcpStream::from_std(tcp_stream, ambient_authority()), addr))
    }

    /// Returns an iterator over the connections being received on this listener.
    ///
    /// This corresponds to [`std::net::TcpListener::incoming`].
    #[inline]
    pub fn incoming(&self) -> Incoming {
        let incoming = self.std.incoming();
        Incoming::from_std(incoming, ambient_authority())
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpListener::set_ttl`].
    #[inline]
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.std.set_ttl(ttl)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This corresponds to [`std::net::TcpListener::ttl`].
    #[inline]
    pub fn ttl(&self) -> io::Result<u32> {
        self.std.ttl()
    }

    /// Gets the value of the `SO_ERROR` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpListener::take_error`].
    #[inline]
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.std.take_error()
    }

    /// Moves this TCP stream into or out of nonblocking mode.
    ///
    /// This corresponds to [`std::net::TcpListener::set_nonblocking`].
    #[inline]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.std.set_nonblocking(nonblocking)
    }
}

#[cfg(not(windows))]
impl FromRawFd for TcpListener {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(net::TcpListener::from_raw_fd(fd), ambient_authority())
    }
}

#[cfg(windows)]
impl FromRawSocket for TcpListener {
    #[inline]
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        Self::from_std(
            net::TcpListener::from_raw_socket(socket),
            ambient_authority(),
        )
    }
}

#[cfg(not(windows))]
impl AsRawFd for TcpListener {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawSocket for TcpListener {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.std.as_raw_socket()
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for TcpListener {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.std.as_raw_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl IntoRawFd for TcpListener {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawSocket for TcpListener {
    #[inline]
    fn into_raw_socket(self) -> RawSocket {
        self.std.into_raw_socket()
    }
}

#[cfg(windows)]
impl IntoRawHandleOrSocket for TcpListener {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.std.into_raw_handle_or_socket()
    }
}

// Safety: `TcpListener` wraps a `net::TcpListener` which owns its handle.
unsafe impl OwnsRaw for TcpListener {}

impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
