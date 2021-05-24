use crate::net::{TcpListener, TcpStream, ToSocketAddrs, UdpSocket};
use async_std::{io, net};
use cap_primitives::net::NO_SOCKET_ADDRS;

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

    /// Add a range of network addresses with a specific port to the pool.
    ///
    /// # Safety
    ///
    /// This function allows ambient access to any IP address.
    pub unsafe fn insert_ip_net(&mut self, ip_net: ipnet::IpNet, port: u16) {
        self.cap.insert_ip_net(ip_net, port)
    }

    /// Add a specific [`net::SocketAddr`] to the pool.
    ///
    /// # Safety
    ///
    /// This function allows ambient access to any IP address.
    pub unsafe fn insert_socket_addr(&mut self, addr: net::SocketAddr) {
        self.cap.insert_socket_addr(addr)
    }

    /// Creates a new `TcpListener` which will be bound to the specified
    /// address.
    ///
    /// This corresponds to [`async_std::net::TcpListener::bind`].
    #[inline]
    pub async fn bind_tcp_listener<A: ToSocketAddrs>(&self, addr: A) -> io::Result<TcpListener> {
        let addrs = addr.to_socket_addrs().await?;

        let mut last_err = None;
        for addr in addrs {
            self.cap.check_addr(&addr)?;
            // TODO: when compiling for WASI, use WASI-specific methods instead
            match net::TcpListener::bind(addr).await {
                Ok(tcp_listener) => return Ok(unsafe { TcpListener::from_std(tcp_listener) }),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::TcpListener::bind(NO_SOCKET_ADDRS).await.unwrap_err()),
        }
    }

    /// Creates a new TCP stream connected to the specified address.
    ///
    /// This corresponds to [`async_std::net::TcpStream::connect`].
    #[inline]
    pub async fn connect_tcp_stream<A: ToSocketAddrs>(&self, addr: A) -> io::Result<TcpStream> {
        let addrs = addr.to_socket_addrs().await?;

        let mut last_err = None;
        for addr in addrs {
            self.cap.check_addr(&addr)?;
            // TODO: when compiling for WASI, use WASI-specific methods instead
            match net::TcpStream::connect(addr).await {
                Ok(tcp_stream) => return Ok(unsafe { TcpStream::from_std(tcp_stream) }),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::TcpStream::connect(NO_SOCKET_ADDRS).await.unwrap_err()),
        }
    }

    // async_std doesn't have `connect_timeout`.

    /// Creates a UDP socket from the given address.
    ///
    /// This corresponds to [`async_std::net::UdpSocket::bind`].
    #[inline]
    pub async fn bind_udp_socket<A: ToSocketAddrs>(&self, addr: A) -> io::Result<UdpSocket> {
        let addrs = addr.to_socket_addrs().await?;

        let mut last_err = None;
        for addr in addrs {
            self.cap.check_addr(&addr)?;
            match net::UdpSocket::bind(addr).await {
                Ok(udp_socket) => return Ok(unsafe { UdpSocket::from_std(udp_socket) }),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::UdpSocket::bind(NO_SOCKET_ADDRS).await.unwrap_err()),
        }
    }

    /// Sends data on the socket to the given address.
    ///
    /// This corresponds to [`async_std::net::UdpSocket::send_to`].
    #[inline]
    pub async fn send_to_udp_socket_addr<A: ToSocketAddrs>(
        &self,
        udp_socket: &UdpSocket,
        buf: &[u8],
        addr: A,
    ) -> io::Result<usize> {
        let mut addrs = addr.to_socket_addrs().await?;

        // `UdpSocket::send_to` only sends to the first address.
        let addr = match addrs.next() {
            None => return Err(net::UdpSocket::bind(NO_SOCKET_ADDRS).await.unwrap_err()),
            Some(addr) => addr,
        };
        self.cap.check_addr(&addr)?;
        udp_socket.std.send_to(buf, addr).await
    }

    /// Connects the UDP socket to a remote address.
    ///
    /// This corresponds to [`async_std::net::UdpSocket::connect`].
    #[inline]
    pub async fn connect_udp_socket<A: ToSocketAddrs>(
        &self,
        udp_socket: &UdpSocket,
        addr: A,
    ) -> io::Result<()> {
        let addrs = addr.to_socket_addrs().await?;

        let mut last_err = None;
        for addr in addrs {
            self.cap.check_addr(&addr)?;
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
}
