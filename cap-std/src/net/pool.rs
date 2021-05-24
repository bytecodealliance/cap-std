#![allow(missing_docs)] // TODO: add docs

use crate::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs, UdpSocket};
use cap_primitives::net::NO_SOCKET_ADDRS;
use std::{io, net, time::Duration};

// FIXME: lots more to do here

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

    /// # Safety
    ///
    /// This function allows ambient access to any IP address.
    pub unsafe fn insert_ip_net(&mut self, ip_net: ipnet::IpNet, port: u16) {
        self.cap.insert_ip_net(ip_net, port)
    }

    /// # Safety
    ///
    /// This function allows ambient access to any IP address.
    pub unsafe fn insert_socket_addr(&mut self, addr: net::SocketAddr) {
        self.cap.insert_socket_addr(addr)
    }

    #[inline]
    pub fn bind_tcp_listener<A: ToSocketAddrs>(&self, addr: A) -> io::Result<TcpListener> {
        let addrs = addr.to_socket_addrs()?;

        let mut last_err = None;
        for addr in addrs {
            self.cap.check_addr(&addr)?;
            // TODO: when compiling for WASI, use WASI-specific methods instead
            match net::TcpListener::bind(addr) {
                Ok(tcp_listener) => return Ok(unsafe { TcpListener::from_std(tcp_listener) }),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::TcpListener::bind(NO_SOCKET_ADDRS).unwrap_err()),
        }
    }

    #[inline]
    pub fn connect_tcp_stream<A: ToSocketAddrs>(&self, addr: A) -> io::Result<TcpStream> {
        let addrs = addr.to_socket_addrs()?;

        let mut last_err = None;
        for addr in addrs {
            self.cap.check_addr(&addr)?;
            // TODO: when compiling for WASI, use WASI-specific methods instead
            match net::TcpStream::connect(addr) {
                Ok(tcp_stream) => return Ok(unsafe { TcpStream::from_std(tcp_stream) }),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::TcpStream::connect(NO_SOCKET_ADDRS).unwrap_err()),
        }
    }

    #[inline]
    pub fn connect_timeout_tcp_stream(
        &self,
        addr: &SocketAddr,
        timeout: Duration,
    ) -> io::Result<TcpStream> {
        self.cap.check_addr(addr)?;
        let tcp_stream = net::TcpStream::connect_timeout(addr, timeout)?;
        Ok(unsafe { TcpStream::from_std(tcp_stream) })
    }

    #[inline]
    pub fn bind_udp_socket<A: ToSocketAddrs>(&self, addr: A) -> io::Result<UdpSocket> {
        let addrs = addr.to_socket_addrs()?;

        let mut last_err = None;
        for addr in addrs {
            self.cap.check_addr(&addr)?;
            match net::UdpSocket::bind(addr) {
                Ok(udp_socket) => return Ok(unsafe { UdpSocket::from_std(udp_socket) }),
                Err(e) => last_err = Some(e),
            }
        }
        match last_err {
            Some(e) => Err(e),
            None => Err(net::UdpSocket::bind(NO_SOCKET_ADDRS).unwrap_err()),
        }
    }

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
