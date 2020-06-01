use crate::os::unix::net::UnixStream;
use std::{io, os::unix};

/// An iterator over incoming connections to a [`UnixListener`].
///
/// This corresponds to [`std::os::unix::net::Incoming`].
///
/// [`std::os::unix::net::Incoming`]: https://doc.rust-lang.org/std/os/unix/net/struct.Incoming.html
/// [`UnixListener`]: struct.UnixListener.html
pub struct Incoming<'a> {
    incoming: unix::net::Incoming<'a>,
}

impl<'a> Incoming<'a> {
    /// Constructs a new instance of `Self` from the given `std::os::unix::net::Incoming`.
    pub fn from_ambient(incoming: unix::net::Incoming<'a>) -> Self {
        Self { incoming }
    }
}

impl<'a> Iterator for Incoming<'a> {
    type Item = io::Result<UnixStream>;

    fn next(&mut self) -> Option<Self::Item> {
        self.incoming
            .next()
            .map(|result| result.map(UnixStream::from_ambient))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.incoming.size_hint()
    }
}

// TODO: impl Debug for Incoming?
