use crate::os::unix::net::{Incoming, SocketAddr, UnixStream};
use std::{
    io,
    os::{
        unix,
        unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd},
    },
};

/// A structure representing a Unix domain socket server.
///
/// This corresponds to [`std::os::unix::net::UnixListener`].
///
/// Note that this `UnixListener` has no `bind` method. To bind it to a socket
/// address, you must first obtain a [`Dir`] containing the path, and
/// then call [`Dir::bind_unix_listener`].
///
/// [`std::os::unix::net::UnixListener`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html
/// [`Dir`]: struct.Dir.html
/// [`Dir::bind_unix_listener`]: struct.Dir.html#method.bind_unix_listener
pub struct UnixListener {
    unix_listener: unix::net::UnixListener,
}

impl UnixListener {
    /// Constructs a new instance of `Self` from the given `std::os::unix::net::UnixListener`.
    #[inline]
    pub fn from_ambient(unix_listener: unix::net::UnixListener) -> Self {
        Self { unix_listener }
    }

    /// Accepts a new incoming connection to this listener.
    ///
    /// This corresponds to [`std::os::unix::net::UnixListener::accept`].
    ///
    /// [`std::os::unix::net::UnixListener::accept`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.accept
    #[inline]
    pub fn accept(&self) -> io::Result<(UnixStream, SocketAddr)> {
        self.unix_listener
            .accept()
            .map(|(unix_stream, addr)| (UnixStream::from_ambient(unix_stream), addr))
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixListener::try_clone`].
    ///
    /// [`std::os::unix::net::UnixListener::try_clone`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.try_clone
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self::from_ambient(self.unix_listener.try_clone()?))
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`std::os::unix::net::UnixListener::local_addr`].
    ///
    /// [`std::os::unix::net::UnixListener::local_addr`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.local_addr
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.unix_listener.local_addr()
    }

    /// Moves the socket into or out of nonblocking mode.
    ///
    /// This corresponds to [`std::os::unix::net::UnixListener::set_nonblocking`].
    ///
    /// [`std::os::unix::net::UnixListener::set_nonblocking`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.set_nonblocking
    #[inline]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.unix_listener.set_nonblocking(nonblocking)
    }

    /// Returns the value of the `SO_ERROR` option.
    ///
    /// This corresponds to [`std::os::unix::net::UnixListener::take_error`].
    ///
    /// [`std::os::unix::net::UnixListener::take_error`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.take_error
    #[inline]
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.unix_listener.take_error()
    }

    /// Returns an iterator over incoming connections.
    ///
    /// This corresponds to [`std::os::unix::net::UnixListener::incoming`].
    ///
    /// [`std::os::unix::net::UnixListener::incoming`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.incoming
    #[inline]
    pub fn incoming(&self) -> Incoming {
        Incoming::from_ambient(self.unix_listener.incoming())
    }
}

impl FromRawFd for UnixListener {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_ambient(unix::net::UnixListener::from_raw_fd(fd))
    }
}

impl AsRawFd for UnixListener {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.unix_listener.as_raw_fd()
    }
}

impl IntoRawFd for UnixListener {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.unix_listener.into_raw_fd()
    }
}

impl<'a> IntoIterator for &'a UnixListener {
    type Item = io::Result<UnixStream>;

    type IntoIter = Incoming<'a>;

    #[inline]
    fn into_iter(self) -> Incoming<'a> {
        Incoming::from_ambient(self.unix_listener.into_iter())
    }
}

// TODO: impl Debug for UnixListener?
