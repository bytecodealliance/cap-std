#![allow(missing_docs)] // TODO: add docs

use crate::{
    net::{TcpListener, TcpStream, ToSocketAddrs, UdpSocket},
    sys,
};
use async_std::io;

// FIXME: lots more to do here

pub struct Catalog {
    sys: sys::net::Catalog,
}

impl Catalog {
    #[inline]
    pub async fn bind_tcp_listener<A: ToSocketAddrs>(&self, addr: A) -> io::Result<TcpListener> {
        self.sys
            .bind_tcp_listener(addr.to_socket_addrs().await?)
            .await
    }

    #[inline]
    pub async fn connect_tcp_stream<A: ToSocketAddrs>(&self, addr: A) -> io::Result<TcpStream> {
        self.sys
            .connect_tcp_stream(addr.to_socket_addrs().await?)
            .await
    }

    // async_std doesn't have `connect_timeout`.

    #[inline]
    pub async fn bind_udp_socket<A: ToSocketAddrs>(&self, addr: A) -> io::Result<UdpSocket> {
        self.sys
            .bind_udp_socket(addr.to_socket_addrs().await?)
            .await
    }

    #[inline]
    pub async fn send_to_udp_socket_addr<A: ToSocketAddrs>(
        &self,
        udp_socket: &UdpSocket,
        buf: &[u8],
        addr: A,
    ) -> io::Result<usize> {
        self.sys
            .send_to_udp_socket_addr(udp_socket, buf, addr.to_socket_addrs().await?)
            .await
    }

    #[inline]
    pub async fn connect_udp_socket<A: ToSocketAddrs>(
        &self,
        udp_socket: &UdpSocket,
        addr: A,
    ) -> io::Result<()> {
        self.sys
            .connect_udp_socket(udp_socket, addr.to_socket_addrs().await?)
            .await
    }
}
