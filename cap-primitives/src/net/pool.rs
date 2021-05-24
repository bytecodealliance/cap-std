#![allow(missing_docs)] // TODO: add docs
#![allow(dead_code, unused_variables)] // TODO: When more things are implemented, remove these.

use ipnet::IpNet;
use std::{io, net};

// FIXME: lots more to do here

#[derive(Clone)]
enum AddrSet {
    Net(IpNet),

    // TODO: Perhaps we should have our own version of `ToSocketAddrs`
    // which returns hostnames rather than parsing them, so we can
    // look up hostnames here.
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

#[derive(Clone)]
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
#[derive(Clone)]
pub struct Pool {
    // TODO: when compiling for WASI, use WASI-specific handle instead
    grants: Vec<IpGrant>,
}

impl Pool {
    /// Construct a new empty pool.
    pub fn new() -> Self {
        Self { grants: Vec::new() }
    }

    /// # Safety
    ///
    /// This function allows ambient access to any IP address.
    pub unsafe fn insert_ip_net(&mut self, ip_net: ipnet::IpNet, port: u16) {
        self.grants.push(IpGrant {
            set: AddrSet::Net(ip_net),
            port,
        })
    }

    /// # Safety
    ///
    /// This function allows ambient access to any IP address.
    pub unsafe fn insert_socket_addr(&mut self, addr: net::SocketAddr) {
        self.grants.push(IpGrant {
            set: AddrSet::Net(addr.ip().into()),
            port: addr.port(),
        })
    }

    pub fn check_addr(&self, addr: &net::SocketAddr) -> io::Result<()> {
        if self.grants.iter().any(|grant| grant.contains(addr)) {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "An address was outside the pool",
            ))
        }
    }
}

pub const NO_SOCKET_ADDRS: &[net::SocketAddr] = &[];
