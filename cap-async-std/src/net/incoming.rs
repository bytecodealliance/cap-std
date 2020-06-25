use crate::net::TcpStream;
use async_std::{
    io, net,
    stream::Stream,
    task::{Context, Poll},
};
use std::pin::Pin;

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
    /// Constructs a new instance of `Self` from the given `async_std::net::Incoming`.
    #[inline]
    pub fn from_std(std: net::Incoming<'a>) -> Self {
        Self { std }
    }
}

impl<'a> Stream for Incoming<'a> {
    type Item = io::Result<TcpStream>;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Stream::poll_next(Pin::new(&mut self.std), cx)
            .map(|poll| poll.map(|result| result.map(TcpStream::from_std)))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.std.size_hint()
    }
}

// TODO: impl Debug for Incoming?
