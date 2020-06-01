use crate::net::Shutdown;
use crate::os::unix::net::SocketAddr;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::{io, os::unix, time::Duration};

pub struct UnixStream {
    unix_stream: unix::net::UnixStream,
}

impl UnixStream {
    pub fn from_net_unix_stream(unix_stream: unix::net::UnixStream) -> Self {
        Self { unix_stream }
    }

    // TODO: figure out where connect should live

    // TODO: should this require a capability?
    pub fn pair() -> io::Result<(UnixStream, UnixStream)> {
        unix::net::UnixStream::pair().map(|(a, b)| {
            (
                UnixStream::from_net_unix_stream(a),
                UnixStream::from_net_unix_stream(b),
            )
        })
    }

    pub fn try_clone(&self) -> io::Result<UnixStream> {
        self.unix_stream
            .try_clone()
            .map(|unix_stream| UnixStream::from_net_unix_stream(unix_stream))
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.unix_stream.local_addr()
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.unix_stream.peer_addr()
    }

    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.unix_stream.set_read_timeout(timeout)
    }

    pub fn set_write_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.unix_stream.set_write_timeout(timeout)
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.unix_stream.read_timeout()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.unix_stream.write_timeout()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.unix_stream.set_nonblocking(nonblocking)
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.unix_stream.take_error()
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.unix_stream.shutdown(how)
    }
}

impl FromRawFd for UnixStream {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        UnixStream::from_net_unix_stream(unix::net::UnixStream::from_raw_fd(fd))
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
