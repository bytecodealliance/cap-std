use crate::os::unix::net::{Incoming, SocketAddr, UnixStream};
use async_std::{
    io,
    os::unix::{
        self,
        io::{AsRawFd, FromRawFd, IntoRawFd, RawFd},
    },
};

/// A structure representing a Unix domain socket server.
///
/// This corresponds to [`async_std::os::unix::net::UnixListener`].
///
/// Note that this `UnixListener` has no `bind` method. To bind it to a socket
/// address, you must first obtain a [`Dir`] containing the path, and
/// then call [`Dir::bind_unix_listener`].
///
/// [`async_std::os::unix::net::UnixListener`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixListener.html
/// [`Dir`]: struct.Dir.html
/// [`Dir::bind_unix_listener`]: struct.Dir.html#method.bind_unix_listener
pub struct UnixListener {
    std: unix::net::UnixListener,
}

impl UnixListener {
    /// Constructs a new instance of `Self` from the given `async_std::os::unix::net::UnixListener`.
    #[inline]
    pub fn from_std(std: unix::net::UnixListener) -> Self {
        Self { std }
    }

    /// Accepts a new incoming connection to this listener.
    ///
    /// This corresponds to [`async_std::os::unix::net::UnixListener::accept`].
    ///
    /// [`async_std::os::unix::net::UnixListener::accept`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixListener.html#method.accept
    #[inline]
    pub async fn accept(&self) -> io::Result<(UnixStream, SocketAddr)> {
        self.std
            .accept()
            .await
            .map(|(unix_stream, addr)| (UnixStream::from_std(unix_stream), addr))
    }

    // async_std doesn't have `try_clone`.

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`async_std::os::unix::net::UnixListener::local_addr`].
    ///
    /// [`async_std::os::unix::net::UnixListener::local_addr`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixListener.html#method.local_addr
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    // async_std doesn't have `set_nonblocking`.

    // async_std doesn't have `take_error`.

    /// Returns an iterator over incoming connections.
    ///
    /// This corresponds to [`async_std::os::unix::net::UnixListener::incoming`].
    ///
    /// [`async_std::os::unix::net::UnixListener::incoming`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixListener.html#method.incoming
    #[inline]
    pub fn incoming(&self) -> Incoming {
        Incoming::from_std(self.std.incoming())
    }
}

impl FromRawFd for UnixListener {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(unix::net::UnixListener::from_raw_fd(fd))
    }
}

impl AsRawFd for UnixListener {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

impl IntoRawFd for UnixListener {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

// async_std's `IntoStream` is unstable.

// TODO: impl Debug for UnixListener
