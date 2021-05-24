use ipnet::IpNet;
use std::{io, net};

// TODO: Perhaps we should have our own version of `ToSocketAddrs` which
// returns hostnames rather than parsing them, so we can add unresolved
// hostnames to the pool.
#[derive(Clone)]
enum AddrSet {
    Net(IpNet),
}

impl AddrSet {
    fn contains(&self, addr: net::IpAddr) -> bool {
        match self {
            Self::Net(ip_net) => ip_net.contains(&addr),
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
///
/// This is presently a very simple concept, though it could grow in
/// sophistication in the future.
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

    /// Add a range of network addresses with a specific port to the pool.
    ///
    /// # Safety
    ///
    /// This function allows ambient access to any IP address.
    pub unsafe fn insert_ip_net(&mut self, ip_net: ipnet::IpNet, port: u16) {
        self.grants.push(IpGrant {
            set: AddrSet::Net(ip_net),
            port,
        })
    }

    /// Add a specific [`net::SocketAddr`] to the pool.
    ///
    /// # Safety
    ///
    /// This function allows ambient access to any IP address.
    pub unsafe fn insert_socket_addr(&mut self, addr: net::SocketAddr) {
        self.grants.push(IpGrant {
            set: AddrSet::Net(addr.ip().into()),
            port: addr.port(),
        })
    }

    /// Check whether the given address is within the pool.
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

/// An empty array of `SocketAddr`s.
pub const NO_SOCKET_ADDRS: &[net::SocketAddr] = &[];
