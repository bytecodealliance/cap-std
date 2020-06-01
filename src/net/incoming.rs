use crate::net::TcpStream;
use std::{io, net};

/// An iterator that infinitely `accept`s connections on a [`TcpListener`].
///
/// This corresponds to [`std::net::Incoming`].
///
/// [`std::net::Incoming`]: https://doc.rust-lang.org/std/net/struct.Incoming.html
/// [`TcpListener`]: struct.TcpListener.html
pub struct Incoming<'a> {
    incoming: net::Incoming<'a>,
}

impl<'a> Incoming<'a> {
    /// Constructs a new instance of `Self` from the given `std::net::Incoming`.
    #[inline]
    pub fn from_ambient(incoming: net::Incoming<'a>) -> Self {
        Self { incoming }
    }
}

impl<'a> Iterator for Incoming<'a> {
    type Item = io::Result<TcpStream>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.incoming
            .next()
            .map(|result| result.map(TcpStream::from_ambient))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.incoming.size_hint()
    }
}

// TODO: impl Debug for Incoming?
