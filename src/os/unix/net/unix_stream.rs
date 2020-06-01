use crate::net::Shutdown;
use crate::os::unix::net::SocketAddr;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::{io, os::unix, time::Duration};

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
    unix_stream: unix::net::UnixStream,
}

impl UnixStream {
    /// Constructs a new instance of `Self` from the given `std::os::unix::net::UnixStream`.
    pub fn from_ambient(unix_stream: unix::net::UnixStream) -> Self {
        Self { unix_stream }
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::pair`].
    ///
    /// TODO: should this require a capability?
    ///
    /// [`std::os::unix::net::UnixStream::pair`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.pair
    pub fn pair() -> io::Result<(Self, Self)> {
        unix::net::UnixStream::pair().map(|(a, b)| (Self::from_ambient(a), Self::from_ambient(b)))
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::try_clone`].
    ///
    /// [`std::os::unix::net::UnixStream::try_clone`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.try_clone
    pub fn try_clone(&self) -> io::Result<Self> {
        self.unix_stream.try_clone().map(Self::from_ambient)
    }

    /// Returns the socket address of the local half of this connection.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::local_addr`].
    ///
    /// [`std::os::unix::net::UnixStream::local_addr`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.local_addr
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.unix_stream.local_addr()
    }

    /// Returns the socket address of the remote half of this connection.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::peer_addr`].
    ///
    /// [`std::os::unix::net::UnixStream::peer_addr`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.peer_addr
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.unix_stream.peer_addr()
    }

    /// Sets the read timeout for the socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::set_read_timeout`].
    ///
    /// [`std::os::unix::net::UnixStream::set_read_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.set_read_timeout
    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.unix_stream.set_read_timeout(timeout)
    }

    /// Sets the write timeout for the socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::set_write_timeout`].
    ///
    /// [`std::os::unix::net::UnixStream::set_write_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.set_write_timeout
    pub fn set_write_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.unix_stream.set_write_timeout(timeout)
    }

    /// Returns the read timeout of this socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::read_timeout`].
    ///
    /// [`std::os::unix::net::UnixStream::read_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.read_timeout
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.unix_stream.read_timeout()
    }

    /// Returns the write timeout of this socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::write_timeout`].
    ///
    /// [`std::os::unix::net::UnixStream::write_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.write_timeout
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.unix_stream.write_timeout()
    }

    /// Moves the socket into or out of nonblocking mode.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::set_nonblocking`].
    ///
    /// [`std::os::unix::net::UnixStream::set_nonblocking`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.set_nonblocking
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.unix_stream.set_nonblocking(nonblocking)
    }

    /// Returns the value of the `SO_ERROR` option.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::take_error`].
    ///
    /// [`std::os::unix::net::UnixStream::take_error`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.take_error
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.unix_stream.take_error()
    }

    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::shutdown`].
    ///
    /// [`std::os::unix::net::UnixStream::shutdown`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.shutdown
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.unix_stream.shutdown(how)
    }
}

impl FromRawFd for UnixStream {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_ambient(unix::net::UnixStream::from_raw_fd(fd))
    }
}

impl AsRawFd for UnixStream {
    fn as_raw_fd(&self) -> RawFd {
        self.unix_stream.as_raw_fd()
    }
}

impl IntoRawFd for UnixStream {
    fn into_raw_fd(self) -> RawFd {
        self.unix_stream.into_raw_fd()
    }
}

impl io::Read for UnixStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.unix_stream.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut]) -> io::Result<usize> {
        self.unix_stream.read_vectored(bufs)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.unix_stream.read_exact(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.unix_stream.read_to_end(buf)
    }

    // TODO: nightly-only APIs initializer?
}

impl io::Write for UnixStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.unix_stream.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.unix_stream.flush()
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice]) -> io::Result<usize> {
        self.unix_stream.write_vectored(bufs)
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.unix_stream.write_all(buf)
    }
}

// TODO: impl Debug for UnixStream?
