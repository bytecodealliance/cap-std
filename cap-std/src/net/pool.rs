use crate::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs, UdpSocket};
use cap_primitives::net::NO_SOCKET_ADDRS;
use cap_primitives::AmbientAuthority;
use std::time::Duration;
use std::{io, net};

/// A pool of network addresses.
///
/// This does not directly correspond to anything in `std`, however its methods
/// correspond to the several functions in [`std::net`].
#[derive(Clone)]
pub struct Pool {
    cap: cap_primitives::net::Pool,
}

impl Pool {
    /// Construct a new empty pool.
    pub fn new() -> Self {
        Self {
            cap: cap_primitives::net::Pool::new(),
        }
    }

    /// Add a range of network addresses to the pool.
    ///
    /// Unlike `insert_ip_net`, this function grants access to any requested
    /// port.
    ///
    /// # Ambient Authority
    ///
    /// This function allows ambient access to any IP address.
    pub fn insert_ip_net_any_port(
        &mut self,
        ip_net: ipnet::IpNet,
        ambient_authority: AmbientAuthority,
    ) {
        self.cap.insert_ip_net_any_port(ip_net, ambient_authority)
    }

    /// Add a range of network addresses with a specific port to the pool.
    ///
    /// # AmbientAuthority
    ///
    /// This function allows ambient access to any IP address.
    pub fn insert_ip_net(
        &mut self,
        ip_net: ipnet::IpNet,
        port: u16,
        ambient_authority: AmbientAuthority,
    ) {
        self.cap.insert_ip_net(ip_net, port, ambient_authority)
    }

    /// Add a specific [`net::SocketAddr`] to the pool.
    ///
    /// # AmbientAuthority
    ///
    /// This function allows ambient access to any IP address.
    pub fn insert_socket_addr(
        &mut self,
        addr: net::SocketAddr,
        ambient_authority: AmbientAuthority,
    ) {
        self.cap.insert_socket_addr(addr, ambient_authority)
    }

    /// Creates a new `TcpListener` which will be bound to the specified
    /// address.
    ///
    /// This corresponds to [`std::net::TcpListener::bind`].
    #[inline]
    pub fn bind_tcp_listener<A: ToSocketAddrs>(&self, addr: A) -> io::Result<TcpListener> {
        let addrs = addr.to_socket_addrs()?;

        let mut last_err = None;
        for addr in addrs {
            self.cap.check_addr(&addr)?;
            // TODO: when compiling for WASI, use WASI-specific methods instead
            match net::TcpListener::bind(addr) {
                Ok(tcp_listener) => return Ok(TcpListener::from_std(tcp_listener)),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::TcpListener::bind(NO_SOCKET_ADDRS).unwrap_err()),
        }
    }

    /// Opens a TCP connection to a remote host.
    ///
    /// This corresponds to [`std::net::TcpStream::connect`].
    #[inline]
    pub fn connect_tcp_stream<A: ToSocketAddrs>(&self, addr: A) -> io::Result<TcpStream> {
        let addrs = addr.to_socket_addrs()?;

        let mut last_err = None;
        for addr in addrs {
            self.cap.check_addr(&addr)?;
            // TODO: when compiling for WASI, use WASI-specific methods instead
            match net::TcpStream::connect(addr) {
                Ok(tcp_stream) => return Ok(TcpStream::from_std(tcp_stream)),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::TcpStream::connect(NO_SOCKET_ADDRS).unwrap_err()),
        }
    }

    /// Opens a TCP connection to a remote host with a timeout.
    ///
    /// This corresponds to [`std::net::TcpStream::connect_timeout`].
    #[inline]
    pub fn connect_timeout_tcp_stream(
        &self,
        addr: &SocketAddr,
        timeout: Duration,
    ) -> io::Result<TcpStream> {
        self.cap.check_addr(addr)?;
        let tcp_stream = net::TcpStream::connect_timeout(addr, timeout)?;
        Ok(TcpStream::from_std(tcp_stream))
    }

    /// Creates a UDP socket from the given address.
    ///
    /// This corresponds to [`std::net::UdpSocket::bind`].
    #[inline]
    pub fn bind_udp_socket<A: ToSocketAddrs>(&self, addr: A) -> io::Result<UdpSocket> {
        let addrs = addr.to_socket_addrs()?;

        let mut last_err = None;
        for addr in addrs {
            self.cap.check_addr(&addr)?;
            match net::UdpSocket::bind(addr) {
                Ok(udp_socket) => return Ok(UdpSocket::from_std(udp_socket)),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::UdpSocket::bind(NO_SOCKET_ADDRS).unwrap_err()),
        }
    }

    /// Sends data on the socket to the given address. On success, returns the
    /// number of bytes written.
    ///
    /// This corresponds to [`std::net::UdpSocket::send_to`].
    #[inline]
    pub fn send_to_udp_socket_addr<A: ToSocketAddrs>(
        &self,
        udp_socket: &UdpSocket,
        buf: &[u8],
        addr: A,
    ) -> io::Result<usize> {
        let mut addrs = addr.to_socket_addrs()?;

        // `UdpSocket::send_to` only sends to the first address.
        let addr = addrs
            .next()
            .ok_or_else(|| net::UdpSocket::bind(NO_SOCKET_ADDRS).unwrap_err())?;
        self.cap.check_addr(&addr)?;
        udp_socket.std.send_to(buf, addr)
    }

    /// Connects this UDP socket to a remote address, allowing the `send` and
    /// `recv` syscalls to be used to send data and also applies filters to
    /// only receive data from the specified address.
    ///
    /// This corresponds to [`std::net::UdpSocket::connect`].
    #[inline]
    pub fn connect_udp_socket<A: ToSocketAddrs>(
        &self,
        udp_socket: &UdpSocket,
        addr: A,
    ) -> io::Result<()> {
        let addrs = addr.to_socket_addrs()?;

        let mut last_err = None;
        for addr in addrs {
            self.cap.check_addr(&addr)?;
            match udp_socket.std.connect(addr) {
                Ok(()) => return Ok(()),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::UdpSocket::bind(NO_SOCKET_ADDRS).unwrap_err()),
        }
    }
}
