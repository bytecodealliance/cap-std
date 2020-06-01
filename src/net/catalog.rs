#![allow(missing_docs)] // TODO: add docs

use crate::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs, UdpSocket};
use ipnet::IpNet;
use std::{io, net, time::Duration};

// FIXME: lots more to do here

enum AddrSet {
    Net(IpNet),
    NameWildcard(String),
}

struct IpGrant {
    set: AddrSet,
    port: u16, // TODO: IANA port names, TODO: range
}

// TODO: rename this?
pub struct Catalog {
    // TODO: when compiling for WASI, use WASI-specific handle instead
    grants: Vec<IpGrant>,
}

// TODO: protocol, address domain, connect/listen/udp

const NO_SOCKET_ADDRS: &[net::SocketAddr] = &[];

impl Catalog {
    pub fn bind_tcp_listener<A: ToSocketAddrs>(&mut self, addr: A) -> io::Result<TcpListener> {
        let mut last_err = None;
        for addr in addr.to_socket_addrs()? {
            self.check_addr(&addr)?;
            // TODO: when compiling for WASI, use WASI-specific methods instead
            match net::TcpListener::bind(addr) {
                Ok(tcp_listener) => return Ok(TcpListener::from_ambient(tcp_listener)),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::TcpListener::bind(NO_SOCKET_ADDRS).unwrap_err()),
        }
    }

    pub fn connect<A: ToSocketAddrs>(&mut self, addr: A) -> io::Result<TcpStream> {
        let mut last_err = None;
        for addr in addr.to_socket_addrs()? {
            self.check_addr(&addr)?;
            match net::TcpStream::connect(addr) {
                Ok(tcp_stream) => return Ok(TcpStream::from_ambient(tcp_stream)),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::TcpStream::connect(NO_SOCKET_ADDRS).unwrap_err()),
        }
    }

    pub fn connect_timeout(addr: &SocketAddr, timeout: Duration) -> io::Result<TcpStream> {
        Ok(TcpStream::from_ambient(net::TcpStream::connect_timeout(
            addr, timeout,
        )?))
    }

    pub fn bind_udp_socket<A: ToSocketAddrs>(&mut self, addr: A) -> io::Result<UdpSocket> {
        let mut last_err = None;
        for addr in addr.to_socket_addrs()? {
            self.check_addr(&addr)?;
            match net::UdpSocket::bind(addr) {
                Ok(udp_socket) => return Ok(UdpSocket::from_ambient(udp_socket)),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::UdpSocket::bind(NO_SOCKET_ADDRS).unwrap_err()),
        }
    }

    pub fn send_to_udp_socket_addr<A: ToSocketAddrs>(
        &mut self,
        udp_socket: &UdpSocket,
        buf: &[u8],
        addr: A,
    ) -> io::Result<usize> {
        unimplemented!(
            "Catalog::send_to_udp_socket_addr({:?}, {:?}",
            buf,
            addr.to_socket_addrs()?.collect::<Vec<_>>()
        )
    }

    pub fn connect_udp_socket<A: ToSocketAddrs>(
        &mut self,
        udp_socket: &UdpSocket,
        addr: A,
    ) -> io::Result<()> {
        unimplemented!(
            "Catalog::connect_udp_socket({:?})",
            addr.to_socket_addrs()?.collect::<Vec<_>>()
        )
    }

    fn check_addr(&self, addr: &SocketAddr) -> io::Result<()> {
        unimplemented!(
            "Catalog::check_addr({:?})",
            addr.to_socket_addrs()?.collect::<Vec<_>>()
        )
        //self.grants.iter().any(|grant| grant.
        //PermissionDenied
    }
}
