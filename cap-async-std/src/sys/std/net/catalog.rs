use crate::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use async_std::{io, net};
use ipnet::IpNet;

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
    pub(crate) async fn bind_tcp_listener(
        &self,
        addrs: impl Iterator<Item = SocketAddr>,
    ) -> io::Result<TcpListener> {
        let mut last_err = None;
        for addr in addrs {
            self.check_addr(&addr)?;
            // TODO: when compiling for WASI, use WASI-specific methods instead
            match net::TcpListener::bind(addr).await {
                Ok(tcp_listener) => return Ok(TcpListener::from_std(tcp_listener)),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::TcpListener::bind(NO_SOCKET_ADDRS).await.unwrap_err()),
        }
    }

    pub(crate) async fn connect_tcp_stream(
        &self,
        addrs: impl Iterator<Item = SocketAddr>,
    ) -> io::Result<TcpStream> {
        let mut last_err = None;
        for addr in addrs {
            self.check_addr(&addr)?;
            // TODO: when compiling for WASI, use WASI-specific methods instead
            match net::TcpStream::connect(addr).await {
                Ok(tcp_stream) => return Ok(TcpStream::from_std(tcp_stream)),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::TcpStream::connect(NO_SOCKET_ADDRS).await.unwrap_err()),
        }
    }

    // async_std doesn't have `connect_timeout`.

    pub(crate) async fn bind_udp_socket(
        &self,
        addrs: impl Iterator<Item = SocketAddr>,
    ) -> io::Result<UdpSocket> {
        let mut last_err = None;
        for addr in addrs {
            self.check_addr(&addr)?;
            match net::UdpSocket::bind(addr).await {
                Ok(udp_socket) => return Ok(UdpSocket::from_std(udp_socket)),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::UdpSocket::bind(NO_SOCKET_ADDRS).await.unwrap_err()),
        }
    }

    pub(crate) async fn send_to_udp_socket_addr(
        &self,
        udp_socket: &UdpSocket,
        buf: &[u8],
        mut addrs: impl Iterator<Item = SocketAddr>,
    ) -> io::Result<usize> {
        // `UdpSocket::send_to` only sends to the first address.
        let addr = match addrs.next() {
            None => return Err(net::UdpSocket::bind(NO_SOCKET_ADDRS).await.unwrap_err()),
            Some(addr) => addr,
        };
        self.check_addr(&addr)?;
        udp_socket.std.send_to(buf, addr).await
    }

    pub(crate) async fn connect_udp_socket(
        &self,
        udp_socket: &UdpSocket,
        addrs: impl Iterator<Item = SocketAddr>,
    ) -> io::Result<()> {
        let mut last_err = None;
        for addr in addrs {
            self.check_addr(&addr)?;
            match udp_socket.std.connect(addr).await {
                Ok(()) => return Ok(()),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::UdpSocket::bind(NO_SOCKET_ADDRS).await.unwrap_err()),
        }
    }

    fn check_addr(&self, addr: &SocketAddr) -> io::Result<()> {
        unimplemented!("Catalog::check_addr({:?})", addr)
        //self.grants.iter().any(|grant| grant.
        //PermissionDenied
    }
}

const NO_SOCKET_ADDRS: &[net::SocketAddr] = &[];
