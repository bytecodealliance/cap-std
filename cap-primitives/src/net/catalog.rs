#![allow(missing_docs)] // TODO: add docs
#![allow(dead_code, unused_variables)] // TODO: When more things are implemented, remove these.

use ipnet::IpNet;
use std::{io, net};

// FIXME: lots more to do here

enum AddrSet {
    Net(IpNet),
    NameWildcard(String),
}

impl AddrSet {
    fn contains(&self, addr: net::IpAddr) -> bool {
        match self {
            Self::Net(ip_net) => ip_net.contains(&addr),
            Self::NameWildcard(name) => false,
        }
    }
}

struct IpGrant {
    set: AddrSet,
    port: u16, // TODO: IANA port names, TODO: range
}

impl IpGrant {
    fn contains(&self, addr: &net::SocketAddr) -> bool {
        self.set.contains(addr.ip()) && addr.port() == self.port
    }
}

/// A representation of a set of network resources that may be accessed.
/// This is presently a very incomplete concept.
///
/// TODO: rename this?
pub struct Catalog {
    // TODO: when compiling for WASI, use WASI-specific handle instead
    grants: Vec<IpGrant>,
}

impl Catalog {
    pub fn check_addr(&self, addr: &net::SocketAddr) -> io::Result<()> {
        if self.grants.iter().any(|grant| grant.contains(addr)) {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "An address led outside the catalog",
            ))
        }
    }
}

pub const NO_SOCKET_ADDRS: &[net::SocketAddr] = &[];
