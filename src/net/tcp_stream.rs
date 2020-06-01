use crate::net::{Shutdown, SocketAddr};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::unix::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};
use std::{io, net, time::Duration};

// TODO: doc comments
// TODO: the lint to require doc comments

pub struct TcpStream {
    tcp_stream: net::TcpStream,
}

impl TcpStream {
    pub fn from_net_tcp_stream(tcp_stream: net::TcpStream) -> Self {
        Self { tcp_stream }
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.tcp_stream.local_addr()
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.tcp_stream.shutdown(how)
    }

    pub fn try_clone(&self) -> io::Result<TcpStream> {
        Ok(TcpStream::from_net_tcp_stream(self.tcp_stream.try_clone()?))
    }

    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.tcp_stream.set_read_timeout(dur)
    }

    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.tcp_stream.set_write_timeout(dur)
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.tcp_stream.read_timeout()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.tcp_stream.write_timeout()
    }

    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.tcp_stream.peek(buf)
    }

    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        self.tcp_stream.set_nodelay(nodelay)
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        self.tcp_stream.nodelay()
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.tcp_stream.set_ttl(ttl)
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.tcp_stream.ttl()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.tcp_stream.take_error()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.tcp_stream.set_nonblocking(nonblocking)
    }
}

#[cfg(unix)]
impl FromRawFd for TcpStream {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        TcpStream::from_net_tcp_stream(net::TcpStream::from_raw_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawSocket for TcpStream {
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        TcpStream::from_net_tcp_stream(net::TcpStream::from_raw_socket(handle))
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
