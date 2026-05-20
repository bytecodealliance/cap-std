use crate::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use crate::try_into_os::*;
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(windows)]
use io_lifetimes::{AsSocket, BorrowedSocket, OwnedSocket};
use std::fmt;
use std::io;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};
use tokio::net;
#[cfg(windows)]
use {
    io_extras::os::windows::{
        AsHandleOrSocket, AsRawHandleOrSocket, BorrowedHandleOrSocket, RawHandleOrSocket,
    },
    std::os::windows::io::{AsRawSocket, IntoRawSocket, RawSocket},
};

/// A UDP socket.
///
/// This corresponds to [`tokio::net::UdpSocket`].
///
/// This `UdpSocket` has no `bind`, `connect`, or `send_to` methods. To create
/// a `UdpSocket` bound to an address or to send a message to an address, first
/// obtain a [`Pool`] permitting the address, and then call
/// [`Pool::bind_udp_socket`], or [`Pool::connect_udp_socket`], or
/// [`Pool::send_to_udp_socket_addr`].
///
/// [`Pool`]: struct.Pool.html
/// [`Pool::bind_udp_socket`]: struct.Pool.html#method.bind_udp_socket
/// [`Pool::connect_udp_socket`]: struct.Pool.html#method.connect_udp_socket
/// [`Pool::send_to_udp_socket_addr`]: struct.Pool.html#method.send_to_udp_socket_addr
pub struct UdpSocket {
    pub(crate) std: net::UdpSocket,
}

impl UdpSocket {
    /// Constructs a new instance of `Self` from the given
    /// `tokio::net::UdpSocket`.
    ///
    /// This grants access the resources the `tokio::net::UdpSocket`
    /// instance already has access to.
    #[inline]
    pub fn from_std(std: net::UdpSocket) -> Self {
        Self { std }
    }

    /// Receives a single datagram message on the socket.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::recv_from`].
    #[inline]
    pub async fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.std.recv_from(buf).await
    }

    /// Returns the socket address of the remote peer this socket was connected
    /// to.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::peer_addr`].
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.std.peer_addr()
    }

    /// Returns the socket address that this socket was created from.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::local_addr`].
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Sets the value of the `SO_BROADCAST` option for this socket.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::set_broadcast`].
    #[inline]
    pub fn set_broadcast(&self, broadcast: bool) -> io::Result<()> {
        self.std.set_broadcast(broadcast)
    }

    /// Gets the value of the `SO_BROADCAST` option for this socket.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::broadcast`].
    #[inline]
    pub fn broadcast(&self) -> io::Result<bool> {
        self.std.broadcast()
    }

    /// Sets the value of the `IP_MULTICAST_LOOP` option for this socket.
    ///
    /// This corresponds to
    /// [`tokio::net::UdpSocket::set_multicast_loop_v4`].
    #[inline]
    pub fn set_multicast_loop_v4(&self, multicast_loop_v4: bool) -> io::Result<()> {
        // tokio doesn't expose set_multicast_loop_v4 directly;
        // use the underlying socket2 or std approach via raw fd
        let socket = socket2::SockRef::from(&self.std);
        socket.set_multicast_loop_v4(multicast_loop_v4)
    }

    /// Gets the value of the `IP_MULTICAST_LOOP` option for this socket.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::multicast_loop_v4`].
    #[inline]
    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        let socket = socket2::SockRef::from(&self.std);
        socket.multicast_loop_v4()
    }

    /// Sets the value of the `IP_MULTICAST_TTL` option for this socket.
    ///
    /// This corresponds to
    /// [`tokio::net::UdpSocket::set_multicast_ttl_v4`].
    #[inline]
    pub fn set_multicast_ttl_v4(&self, multicast_ttl_v4: u32) -> io::Result<()> {
        let socket = socket2::SockRef::from(&self.std);
        socket.set_multicast_ttl_v4(multicast_ttl_v4)
    }

    /// Gets the value of the `IP_MULTICAST_TTL` option for this socket.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::multicast_ttl_v4`].
    #[inline]
    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        let socket = socket2::SockRef::from(&self.std);
        socket.multicast_ttl_v4()
    }

    /// Sets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    ///
    /// This corresponds to
    /// [`tokio::net::UdpSocket::set_multicast_loop_v6`].
    #[inline]
    pub fn set_multicast_loop_v6(&self, multicast_loop_v6: bool) -> io::Result<()> {
        let socket = socket2::SockRef::from(&self.std);
        socket.set_multicast_loop_v6(multicast_loop_v6)
    }

    /// Gets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::multicast_loop_v6`].
    #[inline]
    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        let socket = socket2::SockRef::from(&self.std);
        socket.multicast_loop_v6()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::set_ttl`].
    #[inline]
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.std.set_ttl(ttl)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::ttl`].
    #[inline]
    pub fn ttl(&self) -> io::Result<u32> {
        self.std.ttl()
    }

    /// Executes an operation of the `IP_ADD_MEMBERSHIP` type.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::join_multicast_v4`].
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    pub fn join_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> io::Result<()> {
        self.std.join_multicast_v4(multiaddr, interface)
    }

    /// Executes an operation of the `IPV6_ADD_MEMBERSHIP` type.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::join_multicast_v6`].
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    pub fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.std.join_multicast_v6(multiaddr, interface)
    }

    /// Executes an operation of the `IP_DROP_MEMBERSHIP` type.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::leave_multicast_v4`].
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    pub fn leave_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> io::Result<()> {
        self.std.leave_multicast_v4(multiaddr, interface)
    }

    /// Executes an operation of the `IPV6_DROP_MEMBERSHIP` type.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::leave_multicast_v6`].
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    pub fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.std.leave_multicast_v6(multiaddr, interface)
    }

    /// Sends data on the socket to the remote address to which it is
    /// connected.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::send`].
    #[inline]
    pub async fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.std.send(buf).await
    }

    /// Receives a single datagram message on the socket from the remote
    /// address to which it is connected.
    ///
    /// This corresponds to [`tokio::net::UdpSocket::recv`].
    #[inline]
    pub async fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.recv(buf).await
    }
}

#[cfg(not(windows))]
impl TryFrom<RawFd> for UdpSocket {
    type Error = io::Error;
    #[inline]
    fn try_from(fd: RawFd) -> io::Result<Self> {
        use std::os::unix::io::FromRawFd;
        let std_socket = unsafe { std::net::UdpSocket::from_raw_fd(fd) };
        std_socket.set_nonblocking(true)?;
        Ok(Self::from_std(net::UdpSocket::from_std(std_socket)?))
    }
}

#[cfg(not(windows))]
impl TryFrom<OwnedFd> for UdpSocket {
    type Error = io::Error;
    #[inline]
    fn try_from(fd: OwnedFd) -> io::Result<Self> {
        let std_socket = std::net::UdpSocket::from(fd);
        std_socket.set_nonblocking(true)?;
        Ok(Self::from_std(net::UdpSocket::from_std(std_socket)?))
    }
}

#[cfg(windows)]
impl TryFrom<RawSocket> for UdpSocket {
    type Error = io::Error;
    #[inline]
    fn try_from(socket: RawSocket) -> io::Result<Self> {
        use std::os::windows::io::FromRawSocket;
        let std_socket = unsafe { std::net::UdpSocket::from_raw_socket(socket) };
        std_socket.set_nonblocking(true)?;
        Ok(Self::from_std(net::UdpSocket::from_std(std_socket)?))
    }
}

#[cfg(windows)]
impl TryFrom<OwnedSocket> for UdpSocket {
    type Error = io::Error;
    #[inline]
    fn try_from(socket: OwnedSocket) -> io::Result<Self> {
        let std_socket = std::net::UdpSocket::from(socket);
        std_socket.set_nonblocking(true)?;
        Ok(Self::from_std(net::UdpSocket::from_std(std_socket)?))
    }
}

#[cfg(not(windows))]
impl AsRawFd for UdpSocket {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

#[cfg(not(windows))]
impl AsFd for UdpSocket {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std.as_fd()
    }
}

#[cfg(windows)]
impl AsRawSocket for UdpSocket {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.std.as_raw_socket()
    }
}

#[cfg(windows)]
impl AsSocket for UdpSocket {
    #[inline]
    fn as_socket(&self) -> BorrowedSocket<'_> {
        self.std.as_socket()
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for UdpSocket {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.std.as_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl AsHandleOrSocket for UdpSocket {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.std.as_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl TryIntoRawFd for UdpSocket {
    fn try_into_raw_fd(self) -> std::io::Result<RawFd> {
        Ok(self.std.into_std()?.into_raw_fd())
    }
}

#[cfg(not(windows))]
impl TryFrom<UdpSocket> for OwnedFd {
    type Error = io::Error;
    #[inline]
    fn try_from(val: UdpSocket) -> io::Result<OwnedFd> {
        use std::os::unix::io::FromRawFd;
        let raw = TryIntoRawFd::try_into_raw_fd(val)?;
        Ok(unsafe { OwnedFd::from_raw_fd(raw) })
    }
}

#[cfg(windows)]
impl TryIntoRawSocket for UdpSocket {
    fn try_into_raw_socket(self) -> std::io::Result<RawSocket> {
        Ok(self.std.into_std()?.into_raw_socket())
    }
}

#[cfg(windows)]
impl TryFrom<UdpSocket> for OwnedSocket {
    type Error = io::Error;
    #[inline]
    fn try_from(val: UdpSocket) -> io::Result<OwnedSocket> {
        use std::os::windows::io::FromRawSocket;
        let raw = TryIntoRawSocket::try_into_raw_socket(val)?;
        Ok(unsafe { OwnedSocket::from_raw_socket(raw) })
    }
}

#[cfg(windows)]
impl TryIntoRawHandleOrSocket for UdpSocket {
    fn try_into_raw_handle_or_socket(
        self,
    ) -> std::io::Result<io_extras::os::windows::RawHandleOrSocket> {
        use TryIntoRawSocket;
        let raw = self.try_into_raw_socket()?;
        Ok(io_extras::os::windows::RawHandleOrSocket::unowned_from_raw_socket(raw))
    }
}

#[cfg(windows)]
impl TryFrom<UdpSocket> for io_extras::os::windows::OwnedHandleOrSocket {
    type Error = io::Error;
    fn try_from(val: UdpSocket) -> io::Result<io_extras::os::windows::OwnedHandleOrSocket> {
        let socket: OwnedSocket = val.try_into()?;
        Ok(io_extras::os::windows::OwnedHandleOrSocket::from_socket(
            socket,
        ))
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
