#![allow(missing_docs)] // TODO: add docs

use crate::{
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs, UdpSocket},
    sys,
};
use std::{io, time::Duration};

// FIXME: lots more to do here

pub struct Catalog {
    sys: sys::net::Catalog,
}

impl Catalog {
    #[inline]
    pub fn bind_tcp_listener<A: ToSocketAddrs>(&self, addr: A) -> io::Result<TcpListener> {
        self.sys.bind_tcp_listener(addr.to_socket_addrs()?)
    }

    #[inline]
    pub fn connect_tcp_stream<A: ToSocketAddrs>(&self, addr: A) -> io::Result<TcpStream> {
        self.sys.connect_tcp_stream(addr.to_socket_addrs()?)
    }

    #[inline]
    pub fn connect_timeout_tcp_stream(
        &self,
        addr: &SocketAddr,
        timeout: Duration,
    ) -> io::Result<TcpStream> {
        self.sys.connect_timeout_tcp_stream(addr, timeout)
    }

    #[inline]
    pub fn bind_udp_socket<A: ToSocketAddrs>(&self, addr: A) -> io::Result<UdpSocket> {
        self.sys.bind_udp_socket(addr.to_socket_addrs()?)
    }

    #[inline]
    pub fn send_to_udp_socket_addr<A: ToSocketAddrs>(
        &self,
        udp_socket: &UdpSocket,
        buf: &[u8],
        addr: A,
    ) -> io::Result<usize> {
        self.sys
            .send_to_udp_socket_addr(udp_socket, buf, addr.to_socket_addrs()?)
    }

    #[inline]
    pub fn connect_udp_socket<A: ToSocketAddrs>(
        &self,
        udp_socket: &UdpSocket,
        addr: A,
    ) -> io::Result<()> {
        self.sys
            .connect_udp_socket(udp_socket, addr.to_socket_addrs()?)
    }
}
