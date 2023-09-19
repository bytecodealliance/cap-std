use crate::net::{TcpListener, TcpStream, ToSocketAddrs, UdpSocket};
use async_std::{io, net};
use cap_primitives::net::no_socket_addrs;
use cap_primitives::{ipnet, AmbientAuthority};

/// A pool of network addresses.
///
/// This does not directly correspond to anything in `async_std`, however its
/// methods correspond to the several functions in [`async_std::net`].
///
/// `Pool` implements `Clone`, which creates new independent entities that
/// carry the full authority of the originals. This means that in a borrow
/// of a `Pool`, the scope of the authority is not necessarily limited to
/// the scope of the borrow.
///
/// Similarly, the [`cap_net_ext::PoolExt`] class allows creating "binder"
/// and "connecter" objects which represent capabilities to bind and
/// connect to addresses.
///
/// [`cap_net_ext::PoolExt`]: https://docs.rs/cap-net-ext/latest/cap_net_ext/trait.PoolExt.html
#[derive(Clone, Default)]
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

    /// Add addresses to the pool.
    ///
    /// # Ambient Authority
    ///
    /// This function allows ambient access to any IP address.
    pub fn insert<A: ToSocketAddrs>(
        &mut self,
        addrs: A,
        ambient_authority: AmbientAuthority,
    ) -> io::Result<()> {
        self.cap.insert(addrs, ambient_authority)
    }

    /// Add a specific [`net::SocketAddr`] to the pool.
    ///
    /// # Ambient Authority
    ///
    /// This function allows ambient access to any IP address.
    pub fn insert_socket_addr(
        &mut self,
        addr: net::SocketAddr,
        ambient_authority: AmbientAuthority,
    ) {
        self.cap.insert_socket_addr(addr, ambient_authority)
    }

    /// Add a range of network addresses, accepting any port, to the pool.
    ///
    /// Unlike `insert_ip_net`, this function grants access to any requested
    /// port.
    ///
    /// # Ambient Authority
    ///
    /// This function allows ambient access to any IP address.
    pub fn insert_ip_net_port_any(&mut self, ip_net: ipnet::IpNet, ambient_authority: AmbientAuthority) {
        self.cap.insert_ip_net_port_any(ip_net, ambient_authority)
    }

    /// Add a range of network addresses, accepting a range of ports, to the pool.
    ///
    /// This grants access to the port range starting at `ports_start` and,
    /// if `ports_end` is provided, ending before `ports_end`.
    ///
    /// # Ambient Authority
    ///
    /// This function allows ambient access to any IP address.
    pub fn insert_ip_net_port_range(
        &mut self,
        ip_net: ipnet::IpNet,
        ports_start: u16,
        ports_end: Option<u16>,
        ambient_authority: AmbientAuthority,
    ) {
        self.cap.insert_ip_net_port_range(ip_net, ports_start, ports_end, ambient_authority)
    }

    /// Add a range of network addresses with a specific port to the pool.
    ///
    /// # Ambient Authority
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

    /// Creates a new `TcpListener` which will be bound to the specified
    /// address.
    ///
    /// This corresponds to [`async_std::net::TcpListener::bind`].
    #[doc(alias = "bind")]
    #[inline]
    pub async fn bind_tcp_listener<A: ToSocketAddrs>(&self, addr: A) -> io::Result<TcpListener> {
        let addrs = addr.to_socket_addrs().await?;

        let mut last_err = None;
        for addr in addrs {
            self.cap.check_addr(&addr)?;
            // TODO: when compiling for WASI, use WASI-specific methods instead
            match net::TcpListener::bind(addr).await {
                Ok(tcp_listener) => return Ok(TcpListener::from_std(tcp_listener)),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(no_socket_addrs()),
        }
    }

    /// Creates a new TCP stream connected to the specified address.
    ///
    /// This corresponds to [`async_std::net::TcpStream::connect`].
    #[doc(alias = "connect")]
    #[inline]
    pub async fn connect_tcp_stream<A: ToSocketAddrs>(&self, addr: A) -> io::Result<TcpStream> {
        let addrs = addr.to_socket_addrs().await?;

        let mut last_err = None;
        for addr in addrs {
            self.cap.check_addr(&addr)?;
            // TODO: when compiling for WASI, use WASI-specific methods instead
            match net::TcpStream::connect(addr).await {
                Ok(tcp_stream) => return Ok(TcpStream::from_std(tcp_stream)),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(no_socket_addrs()),
        }
    }

    // async_std doesn't have `connect_timeout`.

    /// Creates a UDP socket from the given address.
    ///
    /// This corresponds to [`async_std::net::UdpSocket::bind`].
    #[doc(alias = "bind")]
    #[inline]
    pub async fn bind_udp_socket<A: ToSocketAddrs>(&self, addr: A) -> io::Result<UdpSocket> {
        let addrs = addr.to_socket_addrs().await?;

        let mut last_err = None;
        for addr in addrs {
            self.cap.check_addr(&addr)?;
            match net::UdpSocket::bind(addr).await {
                Ok(udp_socket) => return Ok(UdpSocket::from_std(udp_socket)),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(no_socket_addrs()),
        }
    }

    /// Sends data on the socket to the given address.
    ///
    /// This corresponds to [`async_std::net::UdpSocket::send_to`].
    #[doc(alias = "send_to")]
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
            None => return Err(no_socket_addrs()),
            Some(addr) => addr,
        };
        self.cap.check_addr(&addr)?;
        udp_socket.std.send_to(buf, addr).await
    }

    /// Connects the UDP socket to a remote address.
    ///
    /// This corresponds to [`async_std::net::UdpSocket::connect`].
    #[doc(alias = "connect")]
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
            None => Err(no_socket_addrs()),
        }
    }
}
