use crate::fs::{Metadata, OpenOptions, Permissions};
use crate::fs_utf8::from_utf8;
use crate::try_into_os::*;
use camino::Utf8Path;
use cap_primitives::AmbientAuthority;
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(windows)]
use io_lifetimes::{AsHandle, BorrowedHandle, OwnedHandle};
use std::fmt;
use std::io;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd, RawFd};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite, ReadBuf};
#[cfg(windows)]
use {
    io_extras::os::windows::{
        AsHandleOrSocket, AsRawHandleOrSocket, BorrowedHandleOrSocket, RawHandleOrSocket,
    },
    std::os::windows::io::{AsRawHandle, FromRawHandle, RawHandle},
};

/// A reference to an open file on a filesystem.
///
/// This corresponds to [`tokio::fs::File`].
///
/// This `File` has no `open` or `create` methods. To open or create a file,
/// first obtain a [`Dir`] containing the path, and then call [`Dir::open`] or
/// [`Dir::create`].
///
/// [`Dir`]: crate::fs::Dir
/// [`Dir::open`]: crate::fs::Dir::open
/// [`Dir::create`]: crate::fs::Dir::create
pub struct File {
    cap_std: crate::fs::File,
}

impl File {
    /// Constructs a new instance of `Self` from the given
    /// `tokio::fs::File`.
    ///
    /// This grants access the resources the `tokio::fs::File` instance
    /// already has access to.
    #[inline]
    pub fn from_std(std: tokio::fs::File) -> Self {
        Self::from_cap_std(crate::fs::File::from_std(std))
    }

    /// Constructs a new instance of `Self` from the given `cap_std::fs::File`.
    #[inline]
    pub fn from_cap_std(cap_std: crate::fs::File) -> Self {
        Self { cap_std }
    }

    /// Consumes `self` and returns a `tokio::fs::File`.
    #[inline]
    pub fn into_std(self) -> tokio::fs::File {
        self.cap_std.into_std()
    }

    /// Attempts to sync all OS-internal metadata to disk.
    ///
    /// This corresponds to [`tokio::fs::File::sync_all`].
    #[inline]
    pub async fn sync_all(&self) -> io::Result<()> {
        self.cap_std.sync_all().await
    }

    /// This function is similar to `sync_all`, except that it may not
    /// synchronize file metadata to a filesystem.
    ///
    /// This corresponds to [`tokio::fs::File::sync_data`].
    #[inline]
    pub async fn sync_data(&self) -> io::Result<()> {
        self.cap_std.sync_data().await
    }

    /// Truncates or extends the underlying file, updating the size of this
    /// file to become size.
    ///
    /// This corresponds to [`tokio::fs::File::set_len`].
    #[inline]
    pub async fn set_len(&self, size: u64) -> io::Result<()> {
        self.cap_std.set_len(size).await
    }

    /// Queries metadata about the underlying file.
    ///
    /// This corresponds to [`tokio::fs::File::metadata`].
    #[inline]
    pub async fn metadata(&self) -> io::Result<Metadata> {
        self.cap_std.metadata().await
    }

    /// Changes the permissions on the underlying file.
    ///
    /// This corresponds to [`tokio::fs::File::set_permissions`].
    #[inline]
    pub async fn set_permissions(&self, perm: Permissions) -> io::Result<()> {
        self.cap_std.set_permissions(perm).await
    }

    /// Constructs a new instance of `Self` in read-only mode by opening the
    /// given path as a file using the host process' ambient authority.
    ///
    /// # Ambient Authority
    ///
    /// This function is not sandboxed and may access any path that the host
    /// process has access to.
    #[inline]
    pub async fn open_ambient<P: AsRef<Utf8Path>>(
        path: P,
        ambient_authority: AmbientAuthority,
    ) -> io::Result<Self> {
        let path = from_utf8(path.as_ref())?;
        crate::fs::File::open_ambient(path, ambient_authority)
            .await
            .map(Self::from_cap_std)
    }

    /// Constructs a new instance of `Self` in write-only mode by opening,
    /// creating or truncating, the given path as a file using the host
    /// process' ambient authority.
    ///
    /// # Ambient Authority
    ///
    /// This function is not sandboxed and may access any path that the host
    /// process has access to.
    #[inline]
    pub async fn create_ambient<P: AsRef<Utf8Path>>(
        path: P,
        ambient_authority: AmbientAuthority,
    ) -> io::Result<Self> {
        let path = from_utf8(path.as_ref())?;
        crate::fs::File::create_ambient(path, ambient_authority)
            .await
            .map(Self::from_cap_std)
    }

    /// Constructs a new instance of `Self` with the options specified by
    /// `options` by opening the given path as a file using the host process'
    /// ambient authority.
    ///
    /// # Ambient Authority
    ///
    /// This function is not sandboxed and may access any path that the host
    /// process has access to.
    #[inline]
    pub async fn open_ambient_with<P: AsRef<Utf8Path>>(
        path: P,
        options: &OpenOptions,
        ambient_authority: AmbientAuthority,
    ) -> io::Result<Self> {
        let path = from_utf8(path.as_ref())?;
        crate::fs::File::open_ambient_with(path, options, ambient_authority)
            .await
            .map(Self::from_cap_std)
    }

    /// Returns a new `OpenOptions` object.
    ///
    /// This corresponds to [`tokio::fs::File::options`].
    #[must_use]
    #[inline]
    pub fn options() -> OpenOptions {
        OpenOptions::new()
    }
}

#[cfg(not(windows))]
impl FromRawFd for File {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(tokio::fs::File::from_std(std::fs::File::from_raw_fd(fd)))
    }
}

#[cfg(not(windows))]
impl From<OwnedFd> for File {
    #[inline]
    fn from(fd: OwnedFd) -> Self {
        Self::from_std(tokio::fs::File::from_std(std::fs::File::from(fd)))
    }
}

#[cfg(windows)]
impl FromRawHandle for File {
    #[inline]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self::from_std(tokio::fs::File::from_std(std::fs::File::from_raw_handle(
            handle,
        )))
    }
}

#[cfg(windows)]
impl From<OwnedHandle> for File {
    #[inline]
    fn from(handle: OwnedHandle) -> Self {
        Self::from_std(tokio::fs::File::from_std(std::fs::File::from(handle)))
    }
}

#[cfg(not(windows))]
impl AsRawFd for File {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.cap_std.as_raw_fd()
    }
}

#[cfg(not(windows))]
impl AsFd for File {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.cap_std.as_fd()
    }
}

#[cfg(windows)]
impl AsRawHandle for File {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.cap_std.as_raw_handle()
    }
}

#[cfg(windows)]
impl AsHandle for File {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.cap_std.as_handle()
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for File {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.cap_std.as_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl AsHandleOrSocket for File {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.cap_std.as_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl TryIntoRawFd for File {
    fn try_into_raw_fd(self) -> std::io::Result<RawFd> {
        TryIntoRawFd::try_into_raw_fd(self.cap_std)
    }
}

#[cfg(not(windows))]
impl TryFrom<File> for OwnedFd {
    type Error = io::Error;
    #[inline]
    fn try_from(file: File) -> io::Result<OwnedFd> {
        let raw = TryIntoRawFd::try_into_raw_fd(file)?;
        Ok(unsafe { std::os::unix::io::OwnedFd::from_raw_fd(raw) })
    }
}

#[cfg(windows)]
impl TryIntoRawHandle for File {
    fn try_into_raw_handle(self) -> std::io::Result<RawHandle> {
        TryIntoRawHandle::try_into_raw_handle(self.cap_std)
    }
}

#[cfg(windows)]
impl TryFrom<File> for OwnedHandle {
    type Error = io::Error;
    #[inline]
    fn try_from(file: File) -> io::Result<OwnedHandle> {
        use std::os::windows::io::IntoRawHandle;
        let raw = TryIntoRawHandle::try_into_raw_handle(file)?;
        Ok(unsafe { std::os::windows::io::OwnedHandle::from_raw_handle(raw) })
    }
}

#[cfg(windows)]
impl TryIntoRawHandleOrSocket for File {
    fn try_into_raw_handle_or_socket(
        self,
    ) -> std::io::Result<io_extras::os::windows::RawHandleOrSocket> {
        TryIntoRawHandleOrSocket::try_into_raw_handle_or_socket(self.cap_std)
    }
}

#[cfg(windows)]
impl TryFrom<File> for io_extras::os::windows::OwnedHandleOrSocket {
    type Error = io::Error;
    fn try_from(file: File) -> io::Result<io_extras::os::windows::OwnedHandleOrSocket> {
        let handle: OwnedHandle = file.try_into()?;
        Ok(handle.into())
    }
}

impl AsyncRead for File {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        AsyncRead::poll_read(Pin::new(&mut self.cap_std), cx, buf)
    }
}

impl AsyncWrite for File {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        AsyncWrite::poll_write(Pin::new(&mut self.cap_std), cx, buf)
    }

    #[inline]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        AsyncWrite::poll_flush(Pin::new(&mut self.cap_std), cx)
    }

    #[inline]
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        AsyncWrite::poll_shutdown(Pin::new(&mut self.cap_std), cx)
    }

    #[inline]
    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        AsyncWrite::poll_write_vectored(Pin::new(&mut self.cap_std), cx, bufs)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.cap_std.is_write_vectored()
    }
}

impl AsyncSeek for File {
    #[inline]
    fn start_seek(mut self: Pin<&mut Self>, position: io::SeekFrom) -> io::Result<()> {
        Pin::new(&mut self.cap_std).start_seek(position)
    }

    #[inline]
    fn poll_complete(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<u64>> {
        Pin::new(&mut self.cap_std).poll_complete(cx)
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cap_std.fmt(f)
    }
}
