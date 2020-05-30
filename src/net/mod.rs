//! A capability-based network API modeled after `std::net`.
//!
//! This corresponds to [`std::net`].
//!
//! Instead of [`std::net`]'s constructor methods which take an address to
//! connect to, this crates has methods on [`Catalog`] which operate on addresses
//! which must be present in the catalog.
//!
//! [`std::net`]: https://doc.rust-lang.org/std/net/index.html
//! [`Catalog`]: struct.Catalog.html

mod catalog;
mod tcp_listener;
mod tcp_stream;
mod udp_socket;

pub use catalog::*;
pub use tcp_listener::*;
pub use tcp_stream::*;
pub use udp_socket::*;

// Re-export things from std::net that we can use as-is.
pub use std::net::{
    AddrParseError, Incoming, IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4,
    SocketAddrV6, ToSocketAddrs,
};

// TODO: re-export experimental Ipv6MulticastScope?

// TODO: UnixStream
