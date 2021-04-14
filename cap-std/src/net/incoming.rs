use crate::net::TcpStream;
use cap_primitives::{ambient_authority, AmbientAuthority};
use std::{fmt, io, net};

/// An iterator that infinitely `accept`s connections on a [`TcpListener`].
///
/// This corresponds to [`std::net::Incoming`].
///
/// [`TcpListener`]: struct.TcpListener.html
pub struct Incoming<'a> {
    std: net::Incoming<'a>,
}

impl<'a> Incoming<'a> {
    /// Constructs a new instance of `Self` from the given `std::net::Incoming`.
    ///
    /// # Ambient Authority
    ///
    /// `std::net::Incoming` is not sandboxed and may access any address that the host
    /// process has access to.
    #[inline]
    pub fn from_std(std: net::Incoming<'a>, _: AmbientAuthority) -> Self {
        Self { std }
    }
}

impl<'a> Iterator for Incoming<'a> {
    type Item = io::Result<TcpStream>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.std.next().map(|result| {
            let tcp_stream = result?;
            Ok(TcpStream::from_std(tcp_stream, ambient_authority()))
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.std.size_hint()
    }
}

impl<'a> fmt::Debug for Incoming<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
