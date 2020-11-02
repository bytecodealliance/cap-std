use crate::net::TcpStream;
use std::{io, net};

/// An iterator that infinitely `accept`s connections on a [`TcpListener`].
///
/// This corresponds to [`std::net::Incoming`].
///
/// [`std::net::Incoming`]: https://doc.rust-lang.org/std/net/struct.Incoming.html
/// [`TcpListener`]: struct.TcpListener.html
pub struct Incoming<'a> {
    std: net::Incoming<'a>,
}

impl<'a> Incoming<'a> {
    /// Constructs a new instance of `Self` from the given `std::net::Incoming`.
    ///
    /// # Safety
    ///
    /// `std::net::Incoming` is not sandboxed and may access any address that the host
    /// process has access to.
    #[inline]
    pub unsafe fn from_std(std: net::Incoming<'a>) -> Self {
        Self { std }
    }
}

impl<'a> Iterator for Incoming<'a> {
    type Item = io::Result<TcpStream>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.std.next().map(|result| {
            let tcp_stream = result?;
            Ok(unsafe { TcpStream::from_std(tcp_stream) })
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.std.size_hint()
    }
}

// TODO: impl Debug for Incoming
