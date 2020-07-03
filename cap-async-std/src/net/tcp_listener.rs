use crate::net::{Incoming, SocketAddr, TcpStream};
#[cfg(unix)]
use async_std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use async_std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};
use async_std::{io, net};

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
    std: net::TcpListener,
}

impl TcpListener {
    /// Constructs a new instance of `Self` from the given `async_std::net::TcpListener`.
    #[inline]
    pub fn from_std(std: net::TcpListener) -> Self {
        Self { std }
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`std::net::TcpListener::local_addr`].
    ///
    /// [`std::net::TcpListener::local_addr`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.local_addr
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    // async_std doesn't have `try_clone`.

    /// Accept a new incoming connection from this listener.
    ///
    /// This corresponds to [`std::net::TcpListener::accept`].
    ///
    /// [`std::net::TcpListener::accept`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.accept
    #[inline]
    pub async fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        self.std
            .accept()
            .await
            .map(|(tcp_stream, addr)| (TcpStream::from_std(tcp_stream), addr))
    }

    /// Returns an iterator over the connections being received on this listener.
    ///
    /// This corresponds to [`std::net::TcpListener::incoming`].
    ///
    /// [`std::net::TcpListener::incoming`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.incoming
    #[inline]
    pub fn incoming(&self) -> Incoming {
        Incoming::from_std(self.std.incoming())
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
        Self::from_std(net::TcpListener::from_raw_socket(handle))
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
impl IntoRawHandle for TcpListener {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.std.into_raw_handle()
    }
}

// TODO: impl Debug for TcpListener?
