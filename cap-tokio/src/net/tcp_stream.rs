use crate::net::SocketAddr;
use crate::try_into_os::*;
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(windows)]
use io_lifetimes::{AsSocket, BorrowedSocket, OwnedSocket};
use std::fmt;
use std::io;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net;
#[cfg(windows)]
use {
    io_extras::os::windows::{
        AsHandleOrSocket, AsRawHandleOrSocket, BorrowedHandleOrSocket, RawHandleOrSocket,
    },
    std::os::windows::io::{AsRawSocket, IntoRawSocket, RawSocket},
};

/// A TCP stream between a local and a remote socket.
///
/// This corresponds to [`tokio::net::TcpStream`].
///
/// This `TcpStream` has no `connect` method. To create a `TcpStream`, first
/// obtain a [`Pool`] permitting the address, and then call
/// [`Pool::connect_tcp_stream`].
///
/// [`Pool`]: struct.Pool.html
/// [`Pool::connect_tcp_stream`]: struct.Pool.html#method.connect_tcp_stream
pub struct TcpStream {
    std: net::TcpStream,
}

impl TcpStream {
    /// Constructs a new instance of `Self` from the given
    /// `tokio::net::TcpStream`.
    ///
    /// This grants access the resources the `tokio::net::TcpStream`
    /// instance already has access to.
    #[inline]
    pub fn from_std(std: net::TcpStream) -> Self {
        Self { std }
    }

    /// Returns the remote address that this stream is connected to.
    ///
    /// This corresponds to [`tokio::net::TcpStream::peer_addr`].
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.std.peer_addr()
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`tokio::net::TcpStream::local_addr`].
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Receives data on the socket from the remote address to which it is
    /// connected, without removing that data from the queue.
    ///
    /// This corresponds to [`tokio::net::TcpStream::peek`].
    #[inline]
    pub async fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.peek(buf).await
    }

    /// Sets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// This corresponds to [`tokio::net::TcpStream::set_nodelay`].
    #[inline]
    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        self.std.set_nodelay(nodelay)
    }

    /// Gets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// This corresponds to [`tokio::net::TcpStream::nodelay`].
    #[inline]
    pub fn nodelay(&self) -> io::Result<bool> {
        self.std.nodelay()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This corresponds to [`tokio::net::TcpStream::set_ttl`].
    #[inline]
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.std.set_ttl(ttl)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This corresponds to [`tokio::net::TcpStream::ttl`].
    #[inline]
    pub fn ttl(&self) -> io::Result<u32> {
        self.std.ttl()
    }
}

#[cfg(not(windows))]
impl TryFrom<RawFd> for TcpStream {
    type Error = io::Error;
    #[inline]
    fn try_from(fd: RawFd) -> io::Result<Self> {
        use std::os::unix::io::FromRawFd;
        let std_stream = unsafe { std::net::TcpStream::from_raw_fd(fd) };
        std_stream.set_nonblocking(true)?;
        Ok(Self::from_std(net::TcpStream::from_std(std_stream)?))
    }
}

#[cfg(not(windows))]
impl TryFrom<OwnedFd> for TcpStream {
    type Error = io::Error;
    #[inline]
    fn try_from(fd: OwnedFd) -> io::Result<Self> {
        let std_stream = std::net::TcpStream::from(fd);
        std_stream.set_nonblocking(true)?;
        Ok(Self::from_std(net::TcpStream::from_std(std_stream)?))
    }
}

#[cfg(windows)]
impl TryFrom<RawSocket> for TcpStream {
    type Error = io::Error;
    #[inline]
    fn try_from(socket: RawSocket) -> io::Result<Self> {
        use std::os::windows::io::FromRawSocket;
        let std_stream = unsafe { std::net::TcpStream::from_raw_socket(socket) };
        std_stream.set_nonblocking(true)?;
        Ok(Self::from_std(net::TcpStream::from_std(std_stream)?))
    }
}

#[cfg(windows)]
impl TryFrom<OwnedSocket> for TcpStream {
    type Error = io::Error;
    #[inline]
    fn try_from(socket: OwnedSocket) -> io::Result<Self> {
        let std_stream = std::net::TcpStream::from(socket);
        std_stream.set_nonblocking(true)?;
        Ok(Self::from_std(net::TcpStream::from_std(std_stream)?))
    }
}

#[cfg(not(windows))]
impl AsRawFd for TcpStream {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

#[cfg(not(windows))]
impl AsFd for TcpStream {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std.as_fd()
    }
}

#[cfg(windows)]
impl AsRawSocket for TcpStream {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.std.as_raw_socket()
    }
}

#[cfg(windows)]
impl AsSocket for TcpStream {
    #[inline]
    fn as_socket(&self) -> BorrowedSocket<'_> {
        self.std.as_socket()
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for TcpStream {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.std.as_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl AsHandleOrSocket for TcpStream {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.std.as_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl TryIntoRawFd for TcpStream {
    fn try_into_raw_fd(self) -> std::io::Result<RawFd> {
        Ok(self.std.into_std()?.into_raw_fd())
    }
}

#[cfg(not(windows))]
impl TryFrom<TcpStream> for OwnedFd {
    type Error = io::Error;
    #[inline]
    fn try_from(val: TcpStream) -> io::Result<OwnedFd> {
        use std::os::unix::io::FromRawFd;
        let raw = TryIntoRawFd::try_into_raw_fd(val)?;
        Ok(unsafe { OwnedFd::from_raw_fd(raw) })
    }
}

#[cfg(windows)]
impl TryIntoRawSocket for TcpStream {
    fn try_into_raw_socket(self) -> std::io::Result<RawSocket> {
        Ok(self.std.into_std()?.into_raw_socket())
    }
}

#[cfg(windows)]
impl TryFrom<TcpStream> for OwnedSocket {
    type Error = io::Error;
    #[inline]
    fn try_from(val: TcpStream) -> io::Result<OwnedSocket> {
        use std::os::windows::io::FromRawSocket;
        let raw = TryIntoRawSocket::try_into_raw_socket(val)?;
        Ok(unsafe { OwnedSocket::from_raw_socket(raw) })
    }
}

#[cfg(windows)]
impl TryIntoRawHandleOrSocket for TcpStream {
    fn try_into_raw_handle_or_socket(
        self,
    ) -> std::io::Result<io_extras::os::windows::RawHandleOrSocket> {
        use TryIntoRawSocket;
        let raw = self.try_into_raw_socket()?;
        Ok(io_extras::os::windows::RawHandleOrSocket::unowned_from_raw_socket(raw))
    }
}

#[cfg(windows)]
impl TryFrom<TcpStream> for io_extras::os::windows::OwnedHandleOrSocket {
    type Error = io::Error;
    fn try_from(val: TcpStream) -> io::Result<io_extras::os::windows::OwnedHandleOrSocket> {
        let socket: OwnedSocket = val.try_into()?;
        Ok(io_extras::os::windows::OwnedHandleOrSocket::from_socket(
            socket,
        ))
    }
}

impl AsyncRead for TcpStream {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        AsyncRead::poll_read(Pin::new(&mut self.std), cx, buf)
    }
}

impl AsyncWrite for TcpStream {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        AsyncWrite::poll_write(Pin::new(&mut self.std), cx, buf)
    }

    #[inline]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        AsyncWrite::poll_flush(Pin::new(&mut self.std), cx)
    }

    #[inline]
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        AsyncWrite::poll_shutdown(Pin::new(&mut self.std), cx)
    }

    #[inline]
    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        AsyncWrite::poll_write_vectored(Pin::new(&mut self.std), cx, bufs)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.std.is_write_vectored()
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
