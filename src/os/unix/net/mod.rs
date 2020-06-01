//! Unix-specific networking functionality
//!
//! This corresponds to [`std::os::unix::net`].
//!
//! [`std::os::unix::net`]: https://doc.rust-lang.org/std/os/unix/net/

mod unix_datagram;
mod unix_listener;
mod unix_stream;

pub use unix_datagram::*;
pub use unix_listener::*;
pub use unix_stream::*;

pub use std::os::unix::net::{Incoming, SocketAddr};
