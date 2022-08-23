use crate::net::{Shutdown, SocketAddr};
use async_std::io::{self, IoSlice, IoSliceMut, Read, Write};
use async_std::net;
#[cfg(unix)]
use async_std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use async_std::task::{Context, Poll};
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(windows)]
use io_lifetimes::{AsSocket, BorrowedSocket, OwnedSocket};
use std::fmt;
use std::pin::Pin;
#[cfg(windows)]
use {
    async_std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket, RawSocket},
    io_extras::os::windows::{AsRawHandleOrSocket, IntoRawHandleOrSocket, RawHandleOrSocket},
};

/// A TCP stream between a local and a remote socket.
///
/// This corresponds to [`async_std::net::TcpStream`].
///
/// This `TcpStream` has no `connect` method. To create a `TcpStream`, first
/// obtain a [`Pool`] permitting the address, and then call
/// [`Pool::connect_tcp_stream`].
///
/// [`Pool`]: struct.Pool.html
/// [`Pool::connect_tcp_stream`]: struct.Pool.html#method.connect_tcp_stream
#[derive(Clone)]
pub struct TcpStream {
    std: net::TcpStream,
}

impl TcpStream {
    /// Constructs a new instance of `Self` from the given
    /// `async_std::net::TcpStream`.
    ///
    /// This grants access the resources the `async_std::net::TcpStream`
    /// instance already has access to.
    #[inline]
    pub fn from_std(std: net::TcpStream) -> Self {
        Self { std }
    }

    /// Returns the remote address that this stream is connected to.
    ///
    /// This corresponds to [`async_std::net::TcpStream::peer_addr`].
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.std.peer_addr()
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`async_std::net::TcpStream::local_addr`].
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This corresponds to [`async_std::net::TcpStream::shutdown`].
    #[inline]
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.std.shutdown(how)
    }

    // async_std doesn't have `try_clone`.

    // async_std doesn't have `set_read_timeout`.

    // async_std doesn't have `set_write_timeout`.

    // async_std doesn't have `read_timeout`.

    // async_std doesn't have `write_timeout`.

    /// Receives data on the socket from the remote address to which it is
    /// connected, without removing that data from the queue.
    ///
    /// This corresponds to [`async_std::net::TcpStream::peek`].
    #[inline]
    pub async fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.peek(buf).await
    }

    /// Sets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// This corresponds to [`async_std::net::TcpStream::set_nodelay`].
    #[inline]
    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        self.std.set_nodelay(nodelay)
    }

    /// Gets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// This corresponds to [`async_std::net::TcpStream::nodelay`].
    #[inline]
    pub fn nodelay(&self) -> io::Result<bool> {
        self.std.nodelay()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This corresponds to [`async_std::net::TcpStream::set_ttl`].
    #[inline]
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.std.set_ttl(ttl)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This corresponds to [`async_std::net::TcpStream::ttl`].
    #[inline]
    pub fn ttl(&self) -> io::Result<u32> {
        self.std.ttl()
    }

    // async_std doesn't have `take_error`.

    // async_std doesn't have `set_nonblocking`.
}

#[cfg(not(windows))]
impl FromRawFd for TcpStream {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(net::TcpStream::from_raw_fd(fd))
    }
}

#[cfg(not(windows))]
impl From<OwnedFd> for TcpStream {
    #[inline]
    fn from(fd: OwnedFd) -> Self {
        Self::from_std(net::TcpStream::from(fd))
    }
}

#[cfg(windows)]
impl FromRawSocket for TcpStream {
    #[inline]
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        Self::from_std(net::TcpStream::from_raw_socket(socket))
    }
}

#[cfg(windows)]
impl From<OwnedSocket> for TcpStream {
    #[inline]
    fn from(socket: OwnedSocket) -> Self {
        Self::from_std(net::TcpStream::from(socket))
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

#[cfg(not(windows))]
impl IntoRawFd for TcpStream {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

#[cfg(not(windows))]
impl From<TcpStream> for OwnedFd {
    #[inline]
    fn from(stream: TcpStream) -> OwnedFd {
        stream.std.into()
    }
}

#[cfg(windows)]
impl IntoRawSocket for TcpStream {
    #[inline]
    fn into_raw_socket(self) -> RawSocket {
        self.std.into_raw_socket()
    }
}

#[cfg(windows)]
impl From<TcpStream> for OwnedSocket {
    #[inline]
    fn from(stream: TcpStream) -> OwnedSocket {
        stream.std.into()
    }
}

#[cfg(windows)]
impl IntoRawHandleOrSocket for TcpStream {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.std.into_raw_handle_or_socket()
    }
}

impl Read for TcpStream {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Read::poll_read(Pin::new(&mut self.std), cx, buf)
    }

    #[inline]
    fn poll_read_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &mut [IoSliceMut],
    ) -> Poll<io::Result<usize>> {
        Read::poll_read_vectored(Pin::new(&mut self.std), cx, bufs)
    }

    // async_std doesn't have `is_read_vectored`.

    // async_std doesn't have `initializer`.
}

impl Read for &TcpStream {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Read::poll_read(Pin::new(&mut &self.std), cx, buf)
    }

    #[inline]
    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &mut [IoSliceMut],
    ) -> Poll<io::Result<usize>> {
        Read::poll_read_vectored(Pin::new(&mut &self.std), cx, bufs)
    }

    // async_std doesn't have `is_read_vectored`.

    // async_std doesn't have `initializer`.
}

impl Write for TcpStream {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Write::poll_write(Pin::new(&mut self.std), cx, buf)
    }

    #[inline]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Write::poll_flush(Pin::new(&mut self.std), cx)
    }

    #[inline]
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Write::poll_close(Pin::new(&mut self.std), cx)
    }

    #[inline]
    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &[IoSlice],
    ) -> Poll<io::Result<usize>> {
        Write::poll_write_vectored(Pin::new(&mut self.std), cx, bufs)
    }

    // async_std doesn't have `is_write_vectored`.

    // async_std doesn't have `write_all_vectored`.
}

impl Write for &TcpStream {
    #[inline]
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        Write::poll_write(Pin::new(&mut &self.std), cx, buf)
    }

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Write::poll_flush(Pin::new(&mut &self.std), cx)
    }

    #[inline]
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Write::poll_close(Pin::new(&mut &self.std), cx)
    }

    #[inline]
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &[IoSlice],
    ) -> Poll<io::Result<usize>> {
        Write::poll_write_vectored(Pin::new(&mut &self.std), cx, bufs)
    }

    // async_std doesn't have `is_write_vectored`.

    // async_std doesn't have `write_all_vectored`.
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
