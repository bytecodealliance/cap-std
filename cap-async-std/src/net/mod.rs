//! A capability-based network API modeled after `async_std::net`.
//!
//! This corresponds to [`async_std::net`].
//!
//! Instead of [`async_std::net`]'s constructor methods which take an address
//! to connect to, this crates has methods on [`Pool`] which operate on
//! addresses which must be present in the pool.
//!
//! [`Pool`]: struct.Pool.html

mod incoming;
mod pool;
mod tcp_listener;
mod tcp_stream;
mod udp_socket;

pub use incoming::*;
pub use pool::*;
pub use tcp_listener::*;
pub use tcp_stream::*;
pub use udp_socket::*;

// Re-export things from `async_std::net` that we can use as-is.
pub use async_std::net::{
    AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4, SocketAddrV6,
    ToSocketAddrs,
};

// TODO: re-export experimental Ipv6MulticastScope?
