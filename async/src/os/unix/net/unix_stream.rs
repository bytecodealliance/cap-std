use crate::{net::Shutdown, os::unix::net::SocketAddr};
use async_std::{
    io,
    os::{
        unix,
        unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd},
    },
    task::{Context, Poll},
};
use std::pin::Pin;

/// A Unix stream socket.
///
/// This corresponds to [`std::os::unix::net::UnixStream`].
///
/// Note that this `UnixStream` has no `connect` method. To create a `UnixStream`,
/// you must first obtain a [`Dir`] containing the path, and then call
/// [`Dir::connect_unix_stream`].
///
/// [`std::os::unix::net::UnixStream`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html
/// [`Dir`]: struct.Dir.html
/// [`Dir::connect_unix_stream`]: struct.Dir.html#method.connect_unix_stream
pub struct UnixStream {
    std: unix::net::UnixStream,
}

impl UnixStream {
    /// Constructs a new instance of `Self` from the given `async_std::os::unix::net::UnixStream`.
    #[inline]
    pub fn from_std(std: unix::net::UnixStream) -> Self {
        Self { std }
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::pair`].
    ///
    /// TODO: should this require a capability?
    ///
    /// [`std::os::unix::net::UnixStream::pair`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.pair
    #[inline]
    pub fn pair() -> io::Result<(Self, Self)> {
        unix::net::UnixStream::pair().map(|(a, b)| (Self::from_std(a), Self::from_std(b)))
    }

    // async_std doesn't have `try_clone`.

    /// Returns the socket address of the local half of this connection.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::local_addr`].
    ///
    /// [`std::os::unix::net::UnixStream::local_addr`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.local_addr
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Returns the socket address of the remote half of this connection.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::peer_addr`].
    ///
    /// [`std::os::unix::net::UnixStream::peer_addr`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.peer_addr
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
    /// This corresponds to [`std::os::unix::net::UnixStream::shutdown`].
    ///
    /// [`std::os::unix::net::UnixStream::shutdown`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.shutdown
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

impl AsRawFd for UnixStream {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

impl IntoRawFd for UnixStream {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

impl io::Read for UnixStream {
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

impl io::Write for UnixStream {
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

// TODO: impl Debug for UnixStream?
