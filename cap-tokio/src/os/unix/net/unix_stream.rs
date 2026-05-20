use crate::os::unix::net::SocketAddr;
use crate::try_into_os::*;
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
use std::fmt;
use std::io;
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net;

/// A Unix stream socket.
///
/// This corresponds to [`tokio::net::UnixStream`].
///
/// This `UnixStream` has no `connect` method. To create a `UnixStream`, first
/// obtain a [`Dir`] containing the path, and then call
/// [`Dir::connect_unix_stream`].
///
/// [`Dir`]: struct.Dir.html
/// [`Dir::connect_unix_stream`]: struct.Dir.html#method.connect_unix_stream
pub struct UnixStream {
    std: net::UnixStream,
}

impl UnixStream {
    /// Constructs a new instance of `Self` from the given
    /// `tokio::net::UnixStream`.
    ///
    /// This grants access the resources the
    /// `tokio::net::UnixStream` instance already has access
    /// to.
    #[inline]
    pub fn from_std(std: net::UnixStream) -> Self {
        Self { std }
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// This corresponds to [`tokio::net::UnixStream::pair`].
    ///
    /// TODO: should this require a capability?
    #[inline]
    pub fn pair() -> io::Result<(Self, Self)> {
        net::UnixStream::pair().map(|(a, b)| (Self::from_std(a), Self::from_std(b)))
    }

    /// Returns the socket address of the local half of this connection.
    ///
    /// This corresponds to
    /// [`tokio::net::UnixStream::local_addr`].
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Returns the socket address of the remote half of this connection.
    ///
    /// This corresponds to
    /// [`tokio::net::UnixStream::peer_addr`].
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.std.peer_addr()
    }
}

impl TryFrom<RawFd> for UnixStream {
    type Error = io::Error;
    #[inline]
    fn try_from(fd: RawFd) -> io::Result<Self> {
        use std::os::unix::io::FromRawFd;
        let std_stream = unsafe { std::os::unix::net::UnixStream::from_raw_fd(fd) };
        std_stream.set_nonblocking(true)?;
        Ok(Self::from_std(net::UnixStream::from_std(std_stream)?))
    }
}

impl TryFrom<OwnedFd> for UnixStream {
    type Error = io::Error;
    #[inline]
    fn try_from(fd: OwnedFd) -> io::Result<Self> {
        let std_stream = std::os::unix::net::UnixStream::from(fd);
        std_stream.set_nonblocking(true)?;
        Ok(Self::from_std(net::UnixStream::from_std(std_stream)?))
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

impl TryIntoRawFd for UnixStream {
    fn try_into_raw_fd(self) -> std::io::Result<RawFd> {
        Ok(self.std.into_std()?.into_raw_fd())
    }
}

impl TryFrom<UnixStream> for OwnedFd {
    type Error = io::Error;
    #[inline]
    fn try_from(val: UnixStream) -> io::Result<OwnedFd> {
        use std::os::unix::io::FromRawFd;
        let raw = TryIntoRawFd::try_into_raw_fd(val)?;
        Ok(unsafe { OwnedFd::from_raw_fd(raw) })
    }
}

impl AsyncRead for UnixStream {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        AsyncRead::poll_read(Pin::new(&mut self.std), cx, buf)
    }
}

impl AsyncWrite for UnixStream {
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

impl fmt::Debug for UnixStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
