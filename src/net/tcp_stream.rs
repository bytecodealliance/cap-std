use crate::net::{Shutdown, SocketAddr};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};
use std::{io, net, time::Duration};

/// A TCP stream between a local and a remote socket.
///
/// This corresponds to [`std::net::TcpStream`].
///
/// Note that this `TcpStream` has no `connect` method. To create a `TcpStream`,
/// you must first obtain a [`Catalog`] permitting the address, and then call
/// [`Catalog::connect_tcp_stream`].
///
/// [`std::net::TcpStream`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html
/// [`Catalog`]: struct.Catalog.html
/// [`Catalog::connect_tcp_stream`]: struct.Catalog.html#method.connect_tcp_stream
pub struct TcpStream {
    tcp_stream: net::TcpStream,
}

impl TcpStream {
    /// Constructs a new instance of `Self` from the given `std::net::TcpStream`.
    pub fn from_ambient(tcp_stream: net::TcpStream) -> Self {
        Self { tcp_stream }
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`std::net::TcpStream::local_addr`].
    ///
    /// [`std::net::TcpStream::local_addr`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.local_addr
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.tcp_stream.local_addr()
    }

    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This corresponds to [`std::net::TcpStream::shutdown`].
    ///
    /// [`std::net::TcpStream::shutdown`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.shutdown
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.tcp_stream.shutdown(how)
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// This corresponds to [`std::net::TcpStream::try_clone`].
    ///
    /// [`std::net::TcpStream::try_clone`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.try_clone
    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self::from_ambient(self.tcp_stream.try_clone()?))
    }

    /// Sets the read timeout to the timeout specified.
    ///
    /// This corresponds to [`std::net::TcpStream::set_read_timeout`].
    ///
    /// [`std::net::TcpStream::set_read_timeout`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_read_timeout
    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.tcp_stream.set_read_timeout(dur)
    }

    /// pub fn set_write_timeout(&self, dur: Option<Duration>) -> Result<()>
    ///
    /// This corresponds to [`std::net::TcpStream::set_write_timeout`].
    ///
    /// [`std::net::TcpStream::set_write_timeout`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_write_timeout
    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.tcp_stream.set_write_timeout(dur)
    }

    /// Returns the read timeout of this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::read_timeout`].
    ///
    /// [`std::net::TcpStream::read_timeout`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.read_timeout
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.tcp_stream.read_timeout()
    }

    /// Returns the write timeout of this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::write_timeout`].
    ///
    /// [`std::net::TcpStream::write_timeout`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.write_timeout
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.tcp_stream.write_timeout()
    }

    /// Receives data on the socket from the remote address to which it is connected, without
    /// removing that data from the queue.
    ///
    /// This corresponds to [`std::net::TcpStream::peek`].
    ///
    /// [`std::net::TcpStream::peek`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.peek
    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.tcp_stream.peek(buf)
    }

    /// Sets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::set_nodelay`].
    ///
    /// [`std::net::TcpStream::set_nodelay`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_nodelay
    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        self.tcp_stream.set_nodelay(nodelay)
    }

    /// Gets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::nodelay`].
    ///
    /// [`std::net::TcpStream::nodelay`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.nodelay
    pub fn nodelay(&self) -> io::Result<bool> {
        self.tcp_stream.nodelay()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::set_ttl`].
    ///
    /// [`std::net::TcpStream::set_ttl`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_ttl
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.tcp_stream.set_ttl(ttl)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::ttl`].
    ///
    /// [`std::net::TcpStream::ttl`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.ttl
    pub fn ttl(&self) -> io::Result<u32> {
        self.tcp_stream.ttl()
    }

    /// Gets the value of the `SO_ERROR` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::take_error`].
    ///
    /// [`std::net::TcpStream::take_error`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.take_error
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.tcp_stream.take_error()
    }

    /// Moves this TCP stream into or out of nonblocking mode.
    ///
    /// This corresponds to [`std::net::TcpStream::set_nonblocking`].
    ///
    /// [`std::net::TcpStream::set_nonblocking`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_nonblocking
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.tcp_stream.set_nonblocking(nonblocking)
    }
}

#[cfg(unix)]
impl FromRawFd for TcpStream {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_ambient(net::TcpStream::from_raw_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawSocket for TcpStream {
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        Self::from_ambient(net::TcpStream::from_raw_socket(handle))
    }
}

#[cfg(unix)]
impl AsRawFd for TcpStream {
    fn as_raw_fd(&self) -> RawFd {
        self.tcp_stream.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawSocket for TcpStream {
    fn as_raw_socket(&self) -> RawSocket {
        self.tcp_stream.as_raw_socket()
    }
}

#[cfg(unix)]
impl IntoRawFd for TcpStream {
    fn into_raw_fd(self) -> RawFd {
        self.tcp_stream.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawHandle for TcpStream {
    fn into_raw_handle(self) -> RawHandle {
        self.tcp_stream.into_raw_handle()
    }
}

impl io::Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.tcp_stream.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut]) -> io::Result<usize> {
        self.tcp_stream.read_vectored(bufs)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.tcp_stream.read_exact(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.tcp_stream.read_to_end(buf)
    }

    // TODO: nightly-only APIs initializer?
}

impl io::Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.tcp_stream.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.tcp_stream.flush()
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice]) -> io::Result<usize> {
        self.tcp_stream.write_vectored(bufs)
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.tcp_stream.write_all(buf)
    }
}

// TODO: impl Debug for TcpStream?
