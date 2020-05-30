use crate::net::{Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::unix::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};
use std::{io, net, time::Duration};

pub struct UdpSocket {
    udp_socket: net::UdpSocket,
}

impl UdpSocket {
    pub fn from_net_udp_socket(udp_socket: net::UdpSocket) -> Self {
        Self { udp_socket }
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.udp_socket.recv_from(buf)
    }

    pub fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.udp_socket.peek_from(buf)
    }

    pub fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], addr: A) -> io::Result<usize> {
        self.udp_socket.send_to(buf, addr)
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.udp_socket.peer_addr()
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.udp_socket.local_addr()
    }

    pub fn try_clone(&self) -> io::Result<UdpSocket> {
        Ok(UdpSocket::from_net_udp_socket(self.udp_socket.try_clone()?))
    }

    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.udp_socket.set_read_timeout(dur)
    }

    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.udp_socket.set_write_timeout(dur)
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.udp_socket.read_timeout()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.udp_socket.write_timeout()
    }

    pub fn set_broadcast(&self, broadcast: bool) -> io::Result<()> {
        self.udp_socket.set_broadcast(broadcast)
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        self.udp_socket.broadcast()
    }

    pub fn set_multicast_loop_v4(&self, multicast_loop_v4: bool) -> io::Result<()> {
        self.udp_socket.set_multicast_loop_v4(multicast_loop_v4)
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        self.udp_socket.multicast_loop_v4()
    }

    pub fn set_multicast_ttl_v4(&self, multicast_ttl_v4: u32) -> io::Result<()> {
        self.udp_socket.set_multicast_ttl_v4(multicast_ttl_v4)
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        self.udp_socket.multicast_ttl_v4()
    }

    pub fn set_multicast_loop_v6(&self, multicast_loop_v6: bool) -> io::Result<()> {
        self.udp_socket.set_multicast_loop_v6(multicast_loop_v6)
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        self.udp_socket.multicast_loop_v6()
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.udp_socket.set_ttl(ttl)
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.udp_socket.ttl()
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn join_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
        self.udp_socket.join_multicast_v4(multiaddr, interface)
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.udp_socket.join_multicast_v6(multiaddr, interface)
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn leave_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
        self.udp_socket.leave_multicast_v4(multiaddr, interface)
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.udp_socket.leave_multicast_v6(multiaddr, interface)
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.udp_socket.take_error()
    }

    pub fn connect<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()> {
        self.udp_socket.connect(addr)
    }

    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.udp_socket.send(buf)
    }

    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.udp_socket.recv(buf)
    }

    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.udp_socket.peek(buf)
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.udp_socket.set_nonblocking(nonblocking)
    }
}

#[cfg(unix)]
impl FromRawFd for UdpSocket {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        UdpSocket::from_net_udp_socket(net::UdpSocket::from_raw_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawSocket for UdpSockeUdpSocket {
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        UdpSocket::from_net_udp_socket(net::UdpSocket::from_raw_socket(handle))
    }
}

#[cfg(unix)]
impl AsRawFd for UdpSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.udp_socket.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawSocket for UdpSocket {
    fn as_raw_socket(&self) -> RawSocket {
        self.udp_socket.as_raw_socket()
    }
}

#[cfg(unix)]
impl IntoRawFd for UdpSocket {
    fn into_raw_fd(self) -> RawFd {
        self.udp_socket.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawHandle for UdpSocket {
    fn into_raw_handle(self) -> RawHandle {
        self.udp_socket.into_raw_handle()
    }
}

// TODO: impl Debug for UdpSocket?
