use crate::net::{SocketAddr, TcpStream};
use crate::try_into_os::*;
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(windows)]
use io_lifetimes::{AsSocket, BorrowedSocket, OwnedSocket};
use std::fmt;
use std::io;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};
use tokio::net;
#[cfg(windows)]
use {
    io_extras::os::windows::{
        AsHandleOrSocket, AsRawHandleOrSocket, BorrowedHandleOrSocket, RawHandleOrSocket,
    },
    std::os::windows::io::{AsRawSocket, IntoRawSocket, RawSocket},
};

/// A TCP socket server, listening for connections.
///
/// This corresponds to [`tokio::net::TcpListener`].
///
/// This `TcpListener` has no `bind` method. To bind it to a socket address,
/// first obtain a [`Pool`] permitting the address, and then call
/// [`Pool::bind_tcp_listener`].
///
/// [`Pool`]: struct.Pool.html
/// [`Pool::bind_tcp_listener`]: struct.Pool.html#method.bind_tcp_listener
pub struct TcpListener {
    std: net::TcpListener,
}

impl TcpListener {
    /// Constructs a new instance of `Self` from the given
    /// `tokio::net::TcpListener`.
    ///
    /// This grants access the resources the `tokio::net::TcpListener`
    /// instance already has access to.
    #[inline]
    pub fn from_std(std: net::TcpListener) -> Self {
        Self { std }
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`tokio::net::TcpListener::local_addr`].
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Accept a new incoming connection from this listener.
    ///
    /// This corresponds to [`tokio::net::TcpListener::accept`].
    #[inline]
    pub async fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        self.std
            .accept()
            .await
            .map(|(tcp_stream, addr)| (TcpStream::from_std(tcp_stream), addr))
    }
}

#[cfg(not(windows))]
impl TryFrom<RawFd> for TcpListener {
    type Error = io::Error;
    #[inline]
    fn try_from(fd: RawFd) -> io::Result<Self> {
        use std::os::unix::io::FromRawFd;
        let std_listener = unsafe { std::net::TcpListener::from_raw_fd(fd) };
        std_listener.set_nonblocking(true)?;
        Ok(Self::from_std(net::TcpListener::from_std(std_listener)?))
    }
}

#[cfg(not(windows))]
impl TryFrom<OwnedFd> for TcpListener {
    type Error = io::Error;
    #[inline]
    fn try_from(fd: OwnedFd) -> io::Result<Self> {
        let std_listener = std::net::TcpListener::from(fd);
        std_listener.set_nonblocking(true)?;
        Ok(Self::from_std(net::TcpListener::from_std(std_listener)?))
    }
}

#[cfg(windows)]
impl TryFrom<RawSocket> for TcpListener {
    type Error = io::Error;
    #[inline]
    fn try_from(socket: RawSocket) -> io::Result<Self> {
        use std::os::windows::io::FromRawSocket;
        let std_listener = unsafe { std::net::TcpListener::from_raw_socket(socket) };
        std_listener.set_nonblocking(true)?;
        Ok(Self::from_std(net::TcpListener::from_std(std_listener)?))
    }
}

#[cfg(windows)]
impl TryFrom<OwnedSocket> for TcpListener {
    type Error = io::Error;
    #[inline]
    fn try_from(socket: OwnedSocket) -> io::Result<Self> {
        let std_listener = std::net::TcpListener::from(socket);
        std_listener.set_nonblocking(true)?;
        Ok(Self::from_std(net::TcpListener::from_std(std_listener)?))
    }
}

#[cfg(not(windows))]
impl AsRawFd for TcpListener {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

#[cfg(not(windows))]
impl AsFd for TcpListener {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std.as_fd()
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
impl AsSocket for TcpListener {
    #[inline]
    fn as_socket(&self) -> BorrowedSocket<'_> {
        self.std.as_socket()
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for TcpListener {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.std.as_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl AsHandleOrSocket for TcpListener {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.std.as_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl TryIntoRawFd for TcpListener {
    fn try_into_raw_fd(self) -> std::io::Result<RawFd> {
        Ok(self.std.into_std()?.into_raw_fd())
    }
}

#[cfg(not(windows))]
impl TryFrom<TcpListener> for OwnedFd {
    type Error = io::Error;
    #[inline]
    fn try_from(val: TcpListener) -> io::Result<OwnedFd> {
        use std::os::unix::io::FromRawFd;
        let raw = TryIntoRawFd::try_into_raw_fd(val)?;
        Ok(unsafe { OwnedFd::from_raw_fd(raw) })
    }
}

#[cfg(windows)]
impl TryIntoRawSocket for TcpListener {
    fn try_into_raw_socket(self) -> std::io::Result<RawSocket> {
        Ok(self.std.into_std()?.into_raw_socket())
    }
}

#[cfg(windows)]
impl TryFrom<TcpListener> for OwnedSocket {
    type Error = io::Error;
    #[inline]
    fn try_from(val: TcpListener) -> io::Result<OwnedSocket> {
        use std::os::windows::io::{FromRawSocket, IntoRawSocket};
        let raw = TryIntoRawSocket::try_into_raw_socket(val)?;
        Ok(unsafe { OwnedSocket::from_raw_socket(raw) })
    }
}

#[cfg(windows)]
impl TryIntoRawHandleOrSocket for TcpListener {
    fn try_into_raw_handle_or_socket(
        self,
    ) -> std::io::Result<io_extras::os::windows::RawHandleOrSocket> {
        use TryIntoRawSocket;
        let raw = self.try_into_raw_socket()?;
        Ok(io_extras::os::windows::RawHandleOrSocket::from_raw_socket(
            raw,
        ))
    }
}

#[cfg(windows)]
impl TryFrom<TcpListener> for io_extras::os::windows::OwnedHandleOrSocket {
    type Error = io::Error;
    fn try_from(val: TcpListener) -> io::Result<io_extras::os::windows::OwnedHandleOrSocket> {
        let socket: OwnedSocket = val.try_into()?;
        Ok(socket.into())
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
