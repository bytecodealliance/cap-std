use crate::net::{Incoming, SocketAddr, TcpStream};
#[cfg(unix)]
use async_std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use async_std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket, RawSocket};
use async_std::{io, net};

/// A TCP socket server, listening for connections.
///
/// This corresponds to [`async_std::net::TcpListener`].
///
/// Note that this `TcpListener` has no `bind` method. To bind it to a socket
/// address, you must first obtain a [`Catalog`] permitting the address, and
/// then call [`Catalog::bind_tcp_listener`].
///
/// [`Catalog`]: struct.Catalog.html
/// [`Catalog::bind_tcp_listener`]: struct.Catalog.html#method.bind_tcp_listener
pub struct TcpListener {
    std: net::TcpListener,
}

impl TcpListener {
    /// Constructs a new instance of `Self` from the given `async_std::net::TcpListener`.
    ///
    /// # Safety
    ///
    /// `async_std::net::TcpListener` is not sandboxed and may access any address that the host
    /// process has access to.
    #[inline]
    pub unsafe fn from_std(std: net::TcpListener) -> Self {
        Self { std }
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`async_std::net::TcpListener::local_addr`].
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    // async_std doesn't have `try_clone`.

    /// Accept a new incoming connection from this listener.
    ///
    /// This corresponds to [`async_std::net::TcpListener::accept`].
    #[inline]
    pub async fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        self.std
            .accept()
            .await
            .map(|(tcp_stream, addr)| (unsafe { TcpStream::from_std(tcp_stream) }, addr))
    }

    /// Returns an iterator over the connections being received on this listener.
    ///
    /// This corresponds to [`async_std::net::TcpListener::incoming`].
    #[inline]
    pub fn incoming(&self) -> Incoming {
        let incoming = self.std.incoming();
        unsafe { Incoming::from_std(incoming) }
    }

    // async_std doesn't have `TcpListener::set_ttl`.

    // async_std doesn't have `TcpListener::ttl`.

    // async_std doesn't have `TcpListener::take_error`.

    // async_std doesn't have `TcpListener::set_nonblocking`.
}

#[cfg(unix)]
impl FromRawFd for TcpListener {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(net::TcpListener::from_raw_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawSocket for TcpListener {
    #[inline]
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        Self::from_std(net::TcpListener::from_raw_socket(socket))
    }
}

#[cfg(unix)]
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

#[cfg(unix)]
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

// TODO: impl Debug for TcpListener
