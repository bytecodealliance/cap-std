use crate::net::TcpStream;
use async_std::stream::Stream;
use async_std::task::{Context, Poll};
use async_std::{io, net};
use std::fmt;
use std::pin::Pin;

/// An iterator that infinitely `accept`s connections on a [`TcpListener`].
///
/// This corresponds to [`async_std::net::Incoming`].
///
/// [`TcpListener`]: struct.TcpListener.html
pub struct Incoming<'a> {
    std: net::Incoming<'a>,
}

impl<'a> Incoming<'a> {
    /// Constructs a new instance of `Self` from the given
    /// `async_std::net::Incoming`.
    ///
    /// This grants access the resources the `async_std::net::Incoming`
    /// instance already has access to.
    #[inline]
    pub fn from_std(std: net::Incoming<'a>) -> Self {
        Self { std }
    }
}

impl<'a> Stream for Incoming<'a> {
    type Item = io::Result<TcpStream>;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Stream::poll_next(Pin::new(&mut self.std), cx).map(|poll| {
            poll.map(|result| {
                let tcp_stream = result?;
                Ok(TcpStream::from_std(tcp_stream))
            })
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
