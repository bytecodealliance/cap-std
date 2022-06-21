use crate::net::Shutdown;
use crate::os::unix::net::SocketAddr;
use async_std::io::{self, IoSlice, IoSliceMut, Read, Write};
use async_std::os::unix;
use async_std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use async_std::task::{Context, Poll};
use io_lifetimes::{AsFd, BorrowedFd, FromFd, IntoFd, OwnedFd};
use std::fmt;
use std::pin::Pin;

/// A Unix stream socket.
///
/// This corresponds to [`async_std::os::unix::net::UnixStream`].
///
/// This `UnixStream` has no `connect` method. To create a `UnixStream`, you
/// must first obtain a [`Dir`] containing the path, and then call
/// [`Dir::connect_unix_stream`].
///
/// [`async_std::os::unix::net::UnixStream`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixStream.html
/// [`Dir`]: struct.Dir.html
/// [`Dir::connect_unix_stream`]: struct.Dir.html#method.connect_unix_stream
#[derive(Clone)]
pub struct UnixStream {
    std: unix::net::UnixStream,
}

impl UnixStream {
    /// Constructs a new instance of `Self` from the given
    /// `async_std::os::unix::net::UnixStream`.
    ///
    /// This grants access the resources the
    /// `async_std::os::unix::net::UnixStream` instance already has access
    /// to.
    #[inline]
    pub fn from_std(std: unix::net::UnixStream) -> Self {
        Self { std }
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// This corresponds to [`async_std::os::unix::net::UnixStream::pair`].
    ///
    /// TODO: should this require a capability?
    ///
    /// [`async_std::os::unix::net::UnixStream::pair`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixStream.html#method.pair
    #[inline]
    pub fn pair() -> io::Result<(Self, Self)> {
        unix::net::UnixStream::pair().map(|(a, b)| (Self::from_std(a), Self::from_std(b)))
    }

    // async_std doesn't have `try_clone`.

    /// Returns the socket address of the local half of this connection.
    ///
    /// This corresponds to
    /// [`async_std::os::unix::net::UnixStream::local_addr`].
    ///
    /// [`async_std::os::unix::net::UnixStream::local_addr`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixStream.html#method.local_addr
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Returns the socket address of the remote half of this connection.
    ///
    /// This corresponds to
    /// [`async_std::os::unix::net::UnixStream::peer_addr`].
    ///
    /// [`async_std::os::unix::net::UnixStream::peer_addr`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixStream.html#method.peer_addr
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.std.peer_addr()
    }

    // async_std doesn't have `set_read_timeout`.

    // async_std doesn't have `set_write_timeout`.

    // async_std doesn't have `read_timeout`.

    // async_std doesn't have `write_timeout`.

    // async_std doesn't have `set_nonblocking`.

    // async_std doesn't have `take_error`.

    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This corresponds to [`async_std::os::unix::net::UnixStream::shutdown`].
    ///
    /// [`async_std::os::unix::net::UnixStream::shutdown`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixStream.html#method.shutdown
    #[inline]
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.std.shutdown(how)
    }
}

impl FromRawFd for UnixStream {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(unix::net::UnixStream::from_raw_fd(fd))
    }
}

impl FromFd for UnixStream {
    #[inline]
    fn from_fd(fd: OwnedFd) -> Self {
        Self::from_std(unix::net::UnixStream::from_fd(fd))
    }
}

impl AsRawFd for UnixStream {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

impl AsFd for UnixStream {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std.as_fd()
    }
}

impl IntoRawFd for UnixStream {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

impl IntoFd for UnixStream {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        self.std.into_fd()
    }
}

impl Read for UnixStream {
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

impl Read for &UnixStream {
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

impl Write for UnixStream {
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

impl Write for &UnixStream {
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

impl fmt::Debug for UnixStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
