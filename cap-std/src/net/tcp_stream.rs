use crate::net::{Shutdown, SocketAddr};
use cap_primitives::{ambient_authority, AmbientAuthority};
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, FromFd, IntoFd, OwnedFd};
#[cfg(windows)]
use io_lifetimes::{AsHandle, BorrowedHandle, FromHandle, IntoHandle, OwnedHandle};
use std::{
    fmt,
    io::{self, IoSlice, IoSliceMut, Read, Write},
    net,
    time::Duration,
};
#[cfg(not(windows))]
use unsafe_io::os::posish::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use unsafe_io::OwnsRaw;
#[cfg(windows)]
use {
    std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket, RawSocket},
    unsafe_io::os::windows::{AsRawHandleOrSocket, IntoRawHandleOrSocket, RawHandleOrSocket},
};

/// A TCP stream between a local and a remote socket.
///
/// This corresponds to [`std::net::TcpStream`].
///
/// Note that this `TcpStream` has no `connect` method. To create a `TcpStream`,
/// you must first obtain a [`Pool`] permitting the address, and then call
/// [`Pool::connect_tcp_stream`].
///
/// [`Pool`]: struct.Pool.html
/// [`Pool::connect_tcp_stream`]: struct.Pool.html#method.connect_tcp_stream
pub struct TcpStream {
    std: net::TcpStream,
}

impl TcpStream {
    /// Constructs a new instance of `Self` from the given `std::net::TcpStream`.
    ///
    /// # Ambient Authority
    ///
    /// `std::net::TcpStream` is not sandboxed and may access any address that the host
    /// process has access to.
    #[inline]
    pub fn from_std(std: net::TcpStream, _: AmbientAuthority) -> Self {
        Self { std }
    }

    /// Returns the socket address of the remote peer of this TCP connection.
    ///
    /// This corresponds to [`std::net::TcpStream::peer_addr`].
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.std.peer_addr()
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`std::net::TcpStream::local_addr`].
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This corresponds to [`std::net::TcpStream::shutdown`].
    #[inline]
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.std.shutdown(how)
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// This corresponds to [`std::net::TcpStream::try_clone`].
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        let tcp_stream = self.std.try_clone()?;
        Ok(Self::from_std(tcp_stream, ambient_authority()))
    }

    /// Sets the read timeout to the timeout specified.
    ///
    /// This corresponds to [`std::net::TcpStream::set_read_timeout`].
    #[inline]
    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.std.set_read_timeout(dur)
    }

    /// Sets the write timeout to the timeout specified.
    ///
    /// This corresponds to [`std::net::TcpStream::set_write_timeout`].
    #[inline]
    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.std.set_write_timeout(dur)
    }

    /// Returns the read timeout of this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::read_timeout`].
    #[inline]
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.std.read_timeout()
    }

    /// Returns the write timeout of this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::write_timeout`].
    #[inline]
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.std.write_timeout()
    }

    /// Receives data on the socket from the remote address to which it is connected, without
    /// removing that data from the queue.
    ///
    /// This corresponds to [`std::net::TcpStream::peek`].
    #[inline]
    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.peek(buf)
    }

    /// Sets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::set_nodelay`].
    #[inline]
    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        self.std.set_nodelay(nodelay)
    }

    /// Gets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::nodelay`].
    #[inline]
    pub fn nodelay(&self) -> io::Result<bool> {
        self.std.nodelay()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::set_ttl`].
    #[inline]
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.std.set_ttl(ttl)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::ttl`].
    #[inline]
    pub fn ttl(&self) -> io::Result<u32> {
        self.std.ttl()
    }

    /// Gets the value of the `SO_ERROR` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::take_error`].
    #[inline]
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.std.take_error()
    }

    /// Moves this TCP stream into or out of nonblocking mode.
    ///
    /// This corresponds to [`std::net::TcpStream::set_nonblocking`].
    #[inline]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.std.set_nonblocking(nonblocking)
    }
}

#[cfg(not(windows))]
impl FromRawFd for TcpStream {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(net::TcpStream::from_raw_fd(fd), ambient_authority())
    }
}

#[cfg(not(windows))]
impl FromFd for TcpStream {
    #[inline]
    fn from_fd(fd: OwnedFd) -> Self {
        Self::from_std(net::TcpStream::from_fd(fd), ambient_authority())
    }
}

#[cfg(windows)]
impl FromRawSocket for TcpStream {
    #[inline]
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        Self::from_std(net::TcpStream::from_raw_socket(socket), ambient_authority())
    }
}

#[cfg(windows)]
impl FromSocket for TcpStream {
    #[inline]
    fn from_socket(socket: OwnedSocket) -> Self {
        Self::from_std(net::TcpStream::from_socket(socket), ambient_authority())
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
impl<'f> AsFd<'f> for &'f TcpStream {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'f> {
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
impl<'s> AsSocket<'s> for &'s TcpStream {
    #[inline]
    fn as_raw_socket(self) -> BorrowedSocket<'s> {
        self.std.as_raw_socket()
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
impl IntoFd for TcpStream {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        self.std.into_fd()
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
impl IntoSocket for TcpStream {
    #[inline]
    fn into_socket(self) -> OwnedSocket {
        self.std.into_socket()
    }
}

#[cfg(windows)]
impl IntoRawHandleOrSocket for TcpStream {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.std.into_raw_handle_or_socket()
    }
}

// Safety: `TcpStream` wraps a `net::TcpStream` which owns its handle.
unsafe impl OwnsRaw for TcpStream {}

impl Read for TcpStream {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        self.std.read_vectored(bufs)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.std.read_exact(buf)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.std.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.std.read_to_string(buf)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        self.std.is_read_vectored()
    }
}

impl Read for &TcpStream {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&mut &self.std).read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        (&mut &self.std).read_vectored(bufs)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        (&mut &self.std).read_exact(buf)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        (&mut &self.std).read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        (&mut &self.std).read_to_string(buf)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        self.std.is_read_vectored()
    }
}

impl Write for TcpStream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.std.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.std.flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice]) -> io::Result<usize> {
        self.std.write_vectored(bufs)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.std.write_all(buf)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.std.is_write_vectored()
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice]) -> io::Result<()> {
        self.std.write_all_vectored(bufs)
    }
}

impl Write for &TcpStream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&mut &self.std).write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        (&mut &self.std).flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice]) -> io::Result<usize> {
        (&mut &self.std).write_vectored(bufs)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        (&mut &self.std).write_all(buf)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.std.is_write_vectored()
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice]) -> io::Result<()> {
        (&mut &self.std).write_all_vectored(bufs)
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
