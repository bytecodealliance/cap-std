use crate::os::unix::net::UnixStream;
use std::{io, os::unix};

/// An iterator over incoming connections to a [`UnixListener`].
///
/// This corresponds to [`std::os::unix::net::Incoming`].
///
/// [`std::os::unix::net::Incoming`]: https://doc.rust-lang.org/std/os/unix/net/struct.Incoming.html
/// [`UnixListener`]: struct.UnixListener.html
pub struct Incoming<'a> {
    std: unix::net::Incoming<'a>,
}

impl<'a> Incoming<'a> {
    /// Constructs a new instance of `Self` from the given `std::os::unix::net::Incoming`.
    ///
    /// # Safety
    ///
    /// `std::net::Incoming` is not sandboxed and may access any address that the host
    /// process has access to.
    #[inline]
    pub unsafe fn from_std(std: unix::net::Incoming<'a>) -> Self {
        Self { std }
    }
}

impl<'a> Iterator for Incoming<'a> {
    type Item = io::Result<UnixStream>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.std.next().map(|result| {
            let unix_stream = result?;
            Ok(unsafe { UnixStream::from_std(unix_stream) })
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.std.size_hint()
    }
}

// TODO: impl Debug for Incoming
