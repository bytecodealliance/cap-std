use crate::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
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
pub(crate) struct Catalog {
    // TODO: when compiling for WASI, use WASI-specific handle instead
    grants: Vec<IpGrant>,
}

impl Catalog {
    pub(crate) fn bind_tcp_listener(
        &self,
        addrs: impl Iterator<Item = SocketAddr>,
    ) -> io::Result<TcpListener> {
        let mut last_err = None;
        for addr in addrs {
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

    pub(crate) fn connect(&self, addrs: impl Iterator<Item = SocketAddr>) -> io::Result<TcpStream> {
        let mut last_err = None;
        for addr in addrs {
            self.check_addr(&addr)?;
            // TODO: when compiling for WASI, use WASI-specific methods instead
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

    pub(crate) fn connect_timeout(
        &self,
        addr: &SocketAddr,
        timeout: Duration,
    ) -> io::Result<TcpStream> {
        unimplemented!("Catalog::connect_timeout({}, {:?}", addr, timeout)
    }

    pub(crate) fn bind_udp_socket(
        &self,
        addrs: impl Iterator<Item = SocketAddr>,
    ) -> io::Result<UdpSocket> {
        let mut last_err = None;
        for addr in addrs {
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

    pub(crate) fn send_to_udp_socket_addr(
        &self,
        udp_socket: &UdpSocket,
        buf: &[u8],
        addrs: impl Iterator<Item = SocketAddr>,
    ) -> io::Result<usize> {
        unimplemented!(
            "Catalog::send_to_udp_socket_addr({:?}, {:?}",
            buf,
            addrs.collect::<Vec<_>>()
        )
    }

    pub(crate) fn connect_udp_socket(
        &self,
        udp_socket: &UdpSocket,
        addrs: impl Iterator<Item = SocketAddr>,
    ) -> io::Result<()> {
        unimplemented!(
            "Catalog::connect_udp_socket({:?})",
            addrs.collect::<Vec<_>>()
        )
    }

    fn check_addr(&self, addr: &SocketAddr) -> io::Result<()> {
        unimplemented!("Catalog::check_addr({:?})", addr)
        //self.grants.iter().any(|grant| grant.
        //PermissionDenied
    }
}

const NO_SOCKET_ADDRS: &[net::SocketAddr] = &[];
