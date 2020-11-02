use crate::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket, RawSocket};
use std::{io, net, time::Duration};

/// A UDP socket.
///
/// This corresponds to [`std::net::UdpSocket`].
///
/// Note that this `UdpSocket` has no `bind`, `connect`, or `send_to` methods. To
/// create a `UdpSocket` bound to an address or to send a message to an address,
/// you must first obtain a [`Catalog`] permitting the address, and then call
/// [`Catalog::bind_udp_socket`], or [`Catalog::connect_udp_socket`], or
/// [`Catalog::send_to_udp_socket_addr`].
///
/// [`std::net::UdpSocket`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html
/// [`Catalog`]: struct.Catalog.html
/// [`Catalog::bind_udp_socket`]: struct.Catalog.html#method.bind_udp_socket
/// [`Catalog::connect_udp_socket`]: struct.Catalog.html#method.connect_udp_socket
/// [`Catalog::send_to_udp_socket_addr`]: struct.Catalog.html#method.send_to_udp_socket_addr
pub struct UdpSocket {
    pub(crate) std: net::UdpSocket,
}

impl UdpSocket {
    /// Constructs a new instance of `Self` from the given `std::net::UdpSocket`.
    ///
    /// # Safety
    ///
    /// `std::net::UdpSocket` is not sandboxed and may access any address that the host
    /// process has access to.
    #[inline]
    pub unsafe fn from_std(std: net::UdpSocket) -> Self {
        Self { std }
    }

    /// Receives a single datagram message on the socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::recv_from`].
    ///
    /// [`std::net::UdpSocket::recv_from`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.recv_from
    #[inline]
    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.std.recv_from(buf)
    }

    /// Receives a single datagram message on the socket, without removing it from the queue.
    ///
    /// This corresponds to [`std::net::UdpSocket::peek_from`].
    ///
    /// [`std::net::UdpSocket::peek_from`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.peek_from
    #[inline]
    pub fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.std.peek_from(buf)
    }

    /// Returns the socket address of the remote peer this socket was connected to.
    ///
    /// This corresponds to [`std::net::UdpSocket::peer_addr`].
    ///
    /// [`std::net::UdpSocket::peer_addr`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.peer_addr
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.std.peer_addr()
    }

    /// Returns the socket address that this socket was created from.
    ///
    /// This corresponds to [`std::net::UdpSocket::local_addr`].
    ///
    /// [`std::net::UdpSocket::local_addr`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.local_addr
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::try_clone`].
    ///
    /// [`std::net::UdpSocket::try_clone`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.try_clone
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        let udp_socket = self.std.try_clone()?;
        Ok(unsafe { Self::from_std(udp_socket) })
    }

    /// Sets the read timeout to the timeout specified.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_read_timeout`].
    ///
    /// [`std::net::UdpSocket::set_read_timeout`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.set_read_timeout
    #[inline]
    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.std.set_read_timeout(dur)
    }

    /// Sets the write timeout to the timeout specified.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_write_timeout`].
    ///
    /// [`std::net::UdpSocket::set_write_timeout`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.set_write_timeout
    #[inline]
    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.std.set_write_timeout(dur)
    }

    /// Returns the read timeout of this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::read_timeout`].
    ///
    /// [`std::net::UdpSocket::read_timeout`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.read_timeout
    #[inline]
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.std.read_timeout()
    }

    /// Returns the write timeout of this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::write_timeout`].
    ///
    /// [`std::net::UdpSocket::write_timeout`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.write_timeout
    #[inline]
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.std.write_timeout()
    }

    /// Sets the value of the `SO_BROADCAST` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_broadcast`].
    ///
    /// [`std::net::UdpSocket::set_broadcast`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.set_broadcast
    #[inline]
    pub fn set_broadcast(&self, broadcast: bool) -> io::Result<()> {
        self.std.set_broadcast(broadcast)
    }

    /// Gets the value of the `SO_BROADCAST` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::broadcast`].
    ///
    /// [`std::net::UdpSocket::broadcast`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.broadcast
    #[inline]
    pub fn broadcast(&self) -> io::Result<bool> {
        self.std.broadcast()
    }

    /// Sets the value of the `IP_MULTICAST_LOOP` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_multicast_loop_v4`].
    ///
    /// [`std::net::UdpSocket::set_multicast_loop_v4`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.set_multicast_loop_v4
    #[inline]
    pub fn set_multicast_loop_v4(&self, multicast_loop_v4: bool) -> io::Result<()> {
        self.std.set_multicast_loop_v4(multicast_loop_v4)
    }

    /// Gets the value of the `IP_MULTICAST_LOOP` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::multicast_loop_v4`].
    ///
    /// [`std::net::UdpSocket::multicast_loop_v4`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.multicast_loop_v4
    #[inline]
    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        self.std.multicast_loop_v4()
    }

    /// Sets the value of the `IP_MULTICAST_TTL` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_multicast_ttl_v4`].
    ///
    /// [`std::net::UdpSocket::set_multicast_ttl_v4`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.set_multicast_ttl_v4
    #[inline]
    pub fn set_multicast_ttl_v4(&self, multicast_ttl_v4: u32) -> io::Result<()> {
        self.std.set_multicast_ttl_v4(multicast_ttl_v4)
    }

    /// Gets the value of the `IP_MULTICAST_TTL` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::multicast_ttl_v4`].
    ///
    /// [`std::net::UdpSocket::multicast_ttl_v4`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.multicast_ttl_v4
    #[inline]
    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        self.std.multicast_ttl_v4()
    }

    /// Sets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_multicast_loop_v6`].
    ///
    /// [`std::net::UdpSocket::set_multicast_loop_v6`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.set_multicast_loop_v6
    #[inline]
    pub fn set_multicast_loop_v6(&self, multicast_loop_v6: bool) -> io::Result<()> {
        self.std.set_multicast_loop_v6(multicast_loop_v6)
    }

    /// Gets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::multicast_loop_v6`].
    ///
    /// [`std::net::UdpSocket::multicast_loop_v6`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.multicast_loop_v6
    #[inline]
    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        self.std.multicast_loop_v6()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_ttl`].
    ///
    /// [`std::net::UdpSocket::set_ttl`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.set_ttl
    #[inline]
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.std.set_ttl(ttl)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::ttl`].
    ///
    /// [`std::net::UdpSocket::ttl`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.ttl
    #[inline]
    pub fn ttl(&self) -> io::Result<u32> {
        self.std.ttl()
    }

    /// Executes an operation of the `IP_ADD_MEMBERSHIP` type.
    ///
    /// This corresponds to [`std::net::UdpSocket::join_multicast_v4`].
    ///
    /// [`std::net::UdpSocket::join_multicast_v4`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.join_multicast_v4
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    pub fn join_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
        self.std.join_multicast_v4(multiaddr, interface)
    }

    /// Executes an operation of the `IPV6_ADD_MEMBERSHIP` type.
    ///
    /// This corresponds to [`std::net::UdpSocket::join_multicast_v6`].
    ///
    /// [`std::net::UdpSocket::join_multicast_v6`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.join_multicast_v6
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    pub fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.std.join_multicast_v6(multiaddr, interface)
    }

    /// Executes an operation of the `IP_DROP_MEMBERSHIP` type.
    ///
    /// This corresponds to [`std::net::UdpSocket::leave_multicast_v4`].
    ///
    /// [`std::net::UdpSocket::leave_multicast_v4`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.leave_multicast_v4
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    pub fn leave_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
        self.std.leave_multicast_v4(multiaddr, interface)
    }

    /// Executes an operation of the `IPV6_DROP_MEMBERSHIP` type.
    ///
    /// This corresponds to [`std::net::UdpSocket::leave_multicast_v6`].
    ///
    /// [`std::net::UdpSocket::leave_multicast_v6`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.leave_multicast_v6
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    pub fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.std.leave_multicast_v6(multiaddr, interface)
    }

    /// Gets the value of the `SO_ERROR` option on this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::take_error`].
    ///
    /// [`std::net::UdpSocket::take_error`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.take_error
    #[inline]
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.std.take_error()
    }

    /// Sends data on the socket to the remote address to which it is connected.
    ///
    /// This corresponds to [`std::net::UdpSocket::send`].
    ///
    /// [`std::net::UdpSocket::send`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.send
    #[inline]
    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.std.send(buf)
    }

    /// Receives a single datagram message on the socket from the remote address to which it is
    /// connected.
    ///
    /// This corresponds to [`std::net::UdpSocket::recv`].
    ///
    /// [`std::net::UdpSocket::recv`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.recv
    #[inline]
    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.recv(buf)
    }

    /// Receives single datagram on the socket from the remote address to which it is connected,
    /// without removing the message from input queue.
    ///
    /// This corresponds to [`std::net::UdpSocket::peek`].
    ///
    /// [`std::net::UdpSocket::peek`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.peek
    #[inline]
    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.peek(buf)
    }

    /// Moves this UDP socket into or out of nonblocking mode.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_nonblocking`].
    ///
    /// [`std::net::UdpSocket::set_nonblocking`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.set_nonblocking
    #[inline]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.std.set_nonblocking(nonblocking)
    }
}

#[cfg(unix)]
impl FromRawFd for UdpSocket {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(net::UdpSocket::from_raw_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawSocket for UdpSocket {
    #[inline]
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        Self::from_std(net::UdpSocket::from_raw_socket(socket))
    }
}

#[cfg(unix)]
impl AsRawFd for UdpSocket {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawSocket for UdpSocket {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.std.as_raw_socket()
    }
}

#[cfg(unix)]
impl IntoRawFd for UdpSocket {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawSocket for UdpSocket {
    #[inline]
    fn into_raw_socket(self) -> RawSocket {
        self.std.into_raw_socket()
    }
}

// TODO: impl Debug for UdpSocket
