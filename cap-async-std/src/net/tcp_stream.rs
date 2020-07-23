use crate::net::{Shutdown, SocketAddr};
#[cfg(unix)]
use async_std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use async_std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket, RawSocket};
use async_std::{
    io, net,
    task::{Context, Poll},
};
use std::pin::Pin;

/// A TCP stream between a local and a remote socket.
///
/// This corresponds to [`async_std::net::TcpStream`].
///
/// Note that this `TcpStream` has no `connect` method. To create a `TcpStream`,
/// you must first obtain a [`Catalog`] permitting the address, and then call
/// [`Catalog::connect_tcp_stream`].
///
/// [`async_std::net::TcpStream`]: https://docs.rs/async-std/latest/async_std/net/struct.TcpStream.html
/// [`Catalog`]: struct.Catalog.html
/// [`Catalog::connect_tcp_stream`]: struct.Catalog.html#method.connect_tcp_stream
pub struct TcpStream {
    std: net::TcpStream,
}

impl TcpStream {
    /// Constructs a new instance of `Self` from the given `async_std::net::TcpStream`.
    #[inline]
    pub fn from_std(std: net::TcpStream) -> Self {
        Self { std }
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`async_std::net::TcpStream::local_addr`].
    ///
    /// [`async_std::net::TcpStream::local_addr`]: https://docs.rs/async-std/latest/async_std/net/struct.TcpStream.html#method.local_addr
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This corresponds to [`async_std::net::TcpStream::shutdown`].
    ///
    /// [`async_std::net::TcpStream::shutdown`]: https://docs.rs/async-std/latest/async_std/net/struct.TcpStream.html#method.shutdown
    #[inline]
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.std.shutdown(how)
    }

    // async_std doesn't have `try_clone`.

    // async_std doesn't have `set_read_timeout`.

    // async_std doesn't have `set_write_timeout`.

    // async_std doesn't have `read_timeout`.

    // async_std doesn't have `write_timeout`.

    /// Receives data on the socket from the remote address to which it is connected, without
    /// removing that data from the queue.
    ///
    /// This corresponds to [`async_std::net::TcpStream::peek`].
    ///
    /// [`async_std::net::TcpStream::peek`]: https://docs.rs/async-std/latest/async_std/net/struct.TcpStream.html#method.peek
    #[inline]
    pub async fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.peek(buf).await
    }

    /// Sets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// This corresponds to [`async_std::net::TcpStream::set_nodelay`].
    ///
    /// [`async_std::net::TcpStream::set_nodelay`]: https://docs.rs/async-std/latest/async_std/net/struct.TcpStream.html#method.set_nodelay
    #[inline]
    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        self.std.set_nodelay(nodelay)
    }

    /// Gets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// This corresponds to [`async_std::net::TcpStream::nodelay`].
    ///
    /// [`async_std::net::TcpStream::nodelay`]: https://docs.rs/async-std/latest/async_std/net/struct.TcpStream.html#method.nodelay
    #[inline]
    pub fn nodelay(&self) -> io::Result<bool> {
        self.std.nodelay()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This corresponds to [`async_std::net::TcpStream::set_ttl`].
    ///
    /// [`async_std::net::TcpStream::set_ttl`]: https://docs.rs/async-std/latest/async_std/net/struct.TcpStream.html#method.set_ttl
    #[inline]
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.std.set_ttl(ttl)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This corresponds to [`async_std::net::TcpStream::ttl`].
    ///
    /// [`async_std::net::TcpStream::ttl`]: https://docs.rs/async-std/latest/async_std/net/struct.TcpStream.html#method.ttl
    #[inline]
    pub fn ttl(&self) -> io::Result<u32> {
        self.std.ttl()
    }

    // async_std doesn't have `take_error`.

    // async_std doesn't have `set_nonblocking`.
}

#[cfg(unix)]
impl FromRawFd for TcpStream {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(net::TcpStream::from_raw_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawSocket for TcpStream {
    #[inline]
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        Self::from_std(net::TcpStream::from_raw_socket(socket))
    }
}

#[cfg(unix)]
impl AsRawFd for TcpStream {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawSocket for TcpStream {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.std.as_raw_socket()
    }
}

#[cfg(unix)]
impl IntoRawFd for TcpStream {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawSocket for TcpStream {
    #[inline]
    fn into_raw_socket(self) -> RawSocket {
        self.std.into_raw_socket()
    }
}

impl io::Read for TcpStream {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        io::Read::poll_read(Pin::new(&mut self.std), cx, buf)
    }

    #[inline]
    fn poll_read_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &mut [io::IoSliceMut],
    ) -> Poll<io::Result<usize>> {
        io::Read::poll_read_vectored(Pin::new(&mut self.std), cx, bufs)
    }

    // TODO: nightly-only APIs initializer?
}

impl io::Write for TcpStream {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        io::Write::poll_write(Pin::new(&mut self.std), cx, buf)
    }

    #[inline]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        io::Write::poll_flush(Pin::new(&mut self.std), cx)
    }

    #[inline]
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        io::Write::poll_close(Pin::new(&mut self.std), cx)
    }

    #[inline]
    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &[io::IoSlice],
    ) -> Poll<io::Result<usize>> {
        io::Write::poll_write_vectored(Pin::new(&mut self.std), cx, bufs)
    }
}

// TODO: impl Debug for TcpStream
