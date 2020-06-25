use crate::os::unix::net::UnixStream;
use async_std::{
    io,
    os::unix,
    stream::Stream,
    task::{Context, Poll},
};
use std::pin::Pin;

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
    /// Constructs a new instance of `Self` from the given `async_std::os::unix::net::Incoming`.
    #[inline]
    pub fn from_std(std: unix::net::Incoming<'a>) -> Self {
        Self { std }
    }
}

impl<'a> Stream for Incoming<'a> {
    type Item = io::Result<UnixStream>;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Stream::poll_next(Pin::new(&mut self.std), cx)
            .map(|poll| poll.map(|result| result.map(UnixStream::from_std)))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.std.size_hint()
    }
}

// TODO: impl Debug for Incoming?
