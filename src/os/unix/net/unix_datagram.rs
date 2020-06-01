use crate::net::Shutdown;
use crate::os::unix::net::SocketAddr;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::{io, os::unix, time::Duration};

pub struct UnixDatagram {
    unix_datagram: unix::net::UnixDatagram,
}

impl UnixDatagram {
    pub fn from_net_unix_datagram(unix_datagram: unix::net::UnixDatagram) -> Self {
        Self { unix_datagram }
    }

    // TODO: figure out where UnixDatagram::bind should live
    // TODO: figure out where UnixDatagram::connect should live

    // TODO: should unbound require a capability?
    pub fn unbound() -> io::Result<UnixDatagram> {
        unix::net::UnixDatagram::unbound().map(UnixDatagram::from_net_unix_datagram)
    }

    pub fn pair() -> io::Result<(UnixDatagram, UnixDatagram)> {
        unix::net::UnixDatagram::pair().map(|(a, b)| {
            (
                UnixDatagram::from_net_unix_datagram(a),
                UnixDatagram::from_net_unix_datagram(b),
            )
        })
    }

    pub fn try_clone(&self) -> io::Result<UnixDatagram> {
        Ok(UnixDatagram::from_net_unix_datagram(
            self.unix_datagram.try_clone()?,
        ))
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.unix_datagram.local_addr()
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.unix_datagram.peer_addr()
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.unix_datagram.recv_from(buf)
    }

    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.unix_datagram.recv(buf)
    }

    // fixme: paths
    /*
    pub fn send_to<P: AsRef<Path>>(&self, buf: &[u8], path: P) -> io::Result<usize> {
        self.unix_datagram.send_to_buf, path)
    }
    */

    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.unix_datagram.send(buf)
    }

    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.unix_datagram.set_read_timeout(timeout)
    }

    pub fn set_write_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.unix_datagram.set_write_timeout(timeout)
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.unix_datagram.read_timeout()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.unix_datagram.write_timeout()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.unix_datagram.set_nonblocking(nonblocking)
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.unix_datagram.take_error()
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.unix_datagram.shutdown(how)
    }
}

impl FromRawFd for UnixDatagram {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        UnixDatagram::from_net_unix_datagram(unix::net::UnixDatagram::from_raw_fd(fd))
    }
}

impl AsRawFd for UnixDatagram {
    fn as_raw_fd(&self) -> RawFd {
        self.unix_datagram.as_raw_fd()
    }
}

impl IntoRawFd for UnixDatagram {
    fn into_raw_fd(self) -> RawFd {
        self.unix_datagram.into_raw_fd()
    }
}

// TODO: impl Debug for UnixDatagram?
