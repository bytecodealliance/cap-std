use crate::os::unix::net::{Incoming, SocketAddr, UnixStream};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::{io, os::unix};

pub struct UnixListener {
    unix_listener: unix::net::UnixListener,
}

impl UnixListener {
    pub fn from_net_unix_listener(unix_listener: unix::net::UnixListener) -> Self {
        Self { unix_listener }
    }

    // TODO: figure out where UnixListener::bind should live

    pub fn accept(&self) -> io::Result<(UnixStream, SocketAddr)> {
        self.unix_listener
            .accept()
            .map(|(unix_stream, addr)| (UnixStream::from_net_unix_stream(unix_stream), addr))
    }

    pub fn try_clone(&self) -> io::Result<UnixListener> {
        Ok(UnixListener::from_net_unix_listener(
            self.unix_listener.try_clone()?,
        ))
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.unix_listener.local_addr()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.unix_listener.set_nonblocking(nonblocking)
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.unix_listener.take_error()
    }

    pub fn incoming(&self) -> Incoming {
        self.unix_listener.incoming()
    }
}

impl FromRawFd for UnixListener {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        UnixListener::from_net_unix_listener(unix::net::UnixListener::from_raw_fd(fd))
    }
}

impl AsRawFd for UnixListener {
    fn as_raw_fd(&self) -> RawFd {
        self.unix_listener.as_raw_fd()
    }
}

impl IntoRawFd for UnixListener {
    fn into_raw_fd(self) -> RawFd {
        self.unix_listener.into_raw_fd()
    }
}

// TODO: impl Debug for UnixListener?

// TODO: impl IntoIterator for UnixListener
