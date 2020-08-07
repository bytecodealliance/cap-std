use crate::fs::{Metadata, Permissions};
#[cfg(unix)]
use async_std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use async_std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};
use async_std::{
    fs, io,
    task::{Context, Poll},
};
use std::{fmt, pin::Pin};

/// A reference to an open file on a filesystem.
///
/// This corresponds to [`async_std::fs::File`].
///
/// Note that this `File` has no `open` or `create` methods. To open or create
/// a file, you must first obtain a [`Dir`] containing the path, and then call
/// [`Dir::open`] or [`Dir::create`].
///
/// [`async_std::fs::File`]: https://docs.rs/async-std/latest/async_std/fs/struct.File.html
/// [`Dir`]: struct.Dir.html
/// [`Dir::open`]: struct.Dir.html#method.open
/// [`Dir::create`]: struct.Dir.html#method.create
pub struct File {
    cap_std: crate::fs::File,
}

impl File {
    /// Constructs a new instance of `Self` from the given `async_std::fs::File`.
    #[inline]
    pub fn from_std(std: fs::File) -> Self {
        Self::from_cap_std(crate::fs::File::from_std(std))
    }

    /// Constructs a new instance of `Self` from the given `cap_std::fs::File`.
    #[inline]
    pub fn from_cap_std(cap_std: crate::fs::File) -> Self {
        Self { cap_std }
    }

    /// Attempts to sync all OS-internal metadata to disk.
    ///
    /// This corresponds to [`async_std::fs::File::sync_all`].
    ///
    /// [`async_std::fs::File::sync_all`]: https://docs.rs/async-std/latest/async_std/fs/struct.File.html#method.sync_all
    #[inline]
    pub async fn sync_all(&self) -> io::Result<()> {
        self.cap_std.sync_all().await
    }

    /// This function is similar to `sync_all`, except that it may not synchronize
    /// file metadata to a filesystem.
    ///
    /// This corresponds to [`async_std::fs::File::sync_data`].
    ///
    /// [`async_std::fs::File::sync_data`]: https://docs.rs/async-std/latest/async_std/fs/struct.File.html#method.sync_data
    #[inline]
    pub async fn sync_data(&self) -> io::Result<()> {
        self.cap_std.sync_data().await
    }

    /// Truncates or extends the underlying file, updating the size of this file
    /// to become size.
    ///
    /// This corresponds to [`async_std::fs::File::set_len`].
    ///
    /// [`async_std::fs::File::set_len`]: https://docs.rs/async-std/latest/async_std/fs/struct.File.html#method.set_len
    #[inline]
    pub async fn set_len(&self, size: u64) -> io::Result<()> {
        self.cap_std.set_len(size).await
    }

    /// Queries metadata about the underlying file.
    ///
    /// This corresponds to [`async_std::fs::File::metadata`].
    ///
    /// [`async_std::fs::File::metadata`]: https://docs.rs/async-std/latest/async_std/fs/struct.File.html#method.metadata
    #[inline]
    pub async fn metadata(&self) -> io::Result<Metadata> {
        self.cap_std.metadata().await
    }

    // async_std doesn't have `try_clone`.

    /// Changes the permissions on the underlying file.
    ///
    /// This corresponds to [`async_std::fs::File::set_permissions`].
    ///
    /// [`async_std::fs::File::set_permissions`]: https://docs.rs/async-std/latest/async_std/fs/struct.File.html#method.set_permissions
    #[inline]
    pub async fn set_permissions(&self, perm: Permissions) -> io::Result<()> {
        self.cap_std.set_permissions(perm).await
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl FromRawFd for File {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(fs::File::from_raw_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawHandle for File {
    #[inline]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self::from_std(fs::File::from_raw_handle(handle))
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl AsRawFd for File {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.cap_std.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawHandle for File {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.cap_std.as_raw_handle()
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl IntoRawFd for File {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.cap_std.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawHandle for File {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.cap_std.into_raw_handle()
    }
}

impl io::Read for File {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        io::Read::poll_read(Pin::new(&mut self.cap_std), cx, buf)
    }

    #[inline]
    fn poll_read_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &mut [io::IoSliceMut],
    ) -> Poll<io::Result<usize>> {
        io::Read::poll_read_vectored(Pin::new(&mut self.cap_std), cx, bufs)
    }

    // TODO: nightly-only APIs initializer?
}

impl io::Write for File {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        io::Write::poll_write(Pin::new(&mut self.cap_std), cx, buf)
    }

    #[inline]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        io::Write::poll_flush(Pin::new(&mut self.cap_std), cx)
    }

    #[inline]
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        io::Write::poll_close(Pin::new(&mut self.cap_std), cx)
    }

    #[inline]
    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &[io::IoSlice],
    ) -> Poll<io::Result<usize>> {
        io::Write::poll_write_vectored(Pin::new(&mut self.cap_std), cx, bufs)
    }
}

impl io::Seek for File {
    #[inline]
    fn poll_seek(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        pos: io::SeekFrom,
    ) -> Poll<io::Result<u64>> {
        io::Seek::poll_seek(Pin::new(&mut self.cap_std), cx, pos)
    }

    // TODO: nightly-only APIs stream_len, stream_position?
}

// TODO: Can async_std implement `From<File>` for `process::Stdio`?

// async_std doesn't have `FileExt`.

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cap_std.fmt(f)
    }
}
