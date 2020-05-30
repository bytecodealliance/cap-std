use crate::net::{Incoming, SocketAddr, TcpStream};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::unix::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};
use std::{io, net};

pub struct TcpListener {
    tcp_listener: net::TcpListener,
}

impl TcpListener {
    pub fn from_net_tcp_listener(tcp_listener: net::TcpListener) -> Self {
        Self { tcp_listener }
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.tcp_listener.local_addr()
    }

    pub fn try_clone(&self) -> io::Result<TcpListener> {
        Ok(TcpListener::from_net_tcp_listener(
            self.tcp_listener.try_clone()?,
        ))
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        self.tcp_listener
            .accept()
            .map(|(tcp_stream, addr)| (TcpStream::from_net_tcp_stream(tcp_stream), addr))
    }

    pub fn incoming(&self) -> Incoming {
        self.tcp_listener.incoming()
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.tcp_listener.set_ttl(ttl)
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.tcp_listener.ttl()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.tcp_listener.take_error()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.tcp_listener.set_nonblocking(nonblocking)
    }
}

#[cfg(unix)]
impl FromRawFd for TcpListener {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        TcpListener::from_net_tcp_listener(net::TcpListener::from_raw_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawSocket for TcpListener {
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        TcpListener::from_net_tcp_listener(net::TcpListener::from_raw_socket(handle))
    }
}

#[cfg(unix)]
impl AsRawFd for TcpListener {
    fn as_raw_fd(&self) -> RawFd {
        self.tcp_listener.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawSocket for TcpListener {
    fn as_raw_socket(&self) -> RawSocket {
        self.tcp_listener.as_raw_socket()
    }
}

#[cfg(unix)]
impl IntoRawFd for TcpListener {
    fn into_raw_fd(self) -> RawFd {
        self.tcp_listener.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawHandle for TcpListener {
    fn into_raw_handle(self) -> RawHandle {
        self.tcp_listener.into_raw_handle()
    }
}

// TODO: impl Debug for TcpListener?
