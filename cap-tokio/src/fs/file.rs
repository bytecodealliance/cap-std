use crate::fs::{Metadata, OpenOptions, Permissions};
use crate::try_into_os::*;
use cap_primitives::fs::open_ambient;
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
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite, ReadBuf};
use tokio::task::spawn_blocking;
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
    pub(crate) std: tokio::fs::File,
}

impl File {
    /// Constructs a new instance of `Self` from the given
    /// `tokio::fs::File`.
    ///
    /// This grants access the resources the `tokio::fs::File` instance
    /// already has access to.
    #[inline]
    pub fn from_std(std: tokio::fs::File) -> Self {
        Self { std }
    }

    /// Consumes `self` and returns a `tokio::fs::File`.
    #[inline]
    pub fn into_std(self) -> tokio::fs::File {
        self.std
    }

    /// Attempts to sync all OS-internal metadata to disk.
    ///
    /// This corresponds to [`tokio::fs::File::sync_all`].
    #[inline]
    pub async fn sync_all(&self) -> io::Result<()> {
        self.std.sync_all().await
    }

    /// This function is similar to `sync_all`, except that it may not
    /// synchronize file metadata to a filesystem.
    ///
    /// This corresponds to [`tokio::fs::File::sync_data`].
    #[inline]
    pub async fn sync_data(&self) -> io::Result<()> {
        self.std.sync_data().await
    }

    /// Truncates or extends the underlying file, updating the size of this
    /// file to become size.
    ///
    /// This corresponds to [`tokio::fs::File::set_len`].
    #[inline]
    pub async fn set_len(&self, size: u64) -> io::Result<()> {
        self.std.set_len(size).await
    }

    /// Queries metadata about the underlying file.
    ///
    /// This corresponds to [`tokio::fs::File::metadata`].
    #[inline]
    pub async fn metadata(&self) -> io::Result<Metadata> {
        let std_file = self.std.try_clone().await?;
        let std_file = std_file.into_std().await;
        spawn_blocking(move || metadata_from(&std_file)).await?
    }

    /// Changes the permissions on the underlying file.
    ///
    /// This corresponds to [`tokio::fs::File::set_permissions`].
    #[inline]
    pub async fn set_permissions(&self, perm: Permissions) -> io::Result<()> {
        let std_file = self.std.try_clone().await?;
        let std_file = std_file.into_std().await;
        spawn_blocking(move || {
            let std_perm = permissions_into_std(&std_file, perm)?;
            std_file.set_permissions(std_perm)
        })
        .await?
    }

    /// Constructs a new instance of `Self` in read-only mode by opening the
    /// given path as a file using the host process' ambient authority.
    ///
    /// # Ambient Authority
    ///
    /// This function is not sandboxed and may access any path that the host
    /// process has access to.
    #[inline]
    pub async fn open_ambient<P: AsRef<Path>>(
        path: P,
        ambient_authority: AmbientAuthority,
    ) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        spawn_blocking(move || {
            open_ambient(
                path.as_ref(),
                OpenOptions::new().read(true),
                ambient_authority,
            )
        })
        .await?
        .map(|f| Self::from_std(tokio::fs::File::from_std(f)))
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
    pub async fn create_ambient<P: AsRef<Path>>(
        path: P,
        ambient_authority: AmbientAuthority,
    ) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        spawn_blocking(move || {
            open_ambient(
                path.as_ref(),
                OpenOptions::new().write(true).create(true).truncate(true),
                ambient_authority,
            )
        })
        .await?
        .map(|f| Self::from_std(tokio::fs::File::from_std(f)))
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
    pub async fn open_ambient_with<P: AsRef<Path>>(
        path: P,
        options: &OpenOptions,
        ambient_authority: AmbientAuthority,
    ) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let options = options.clone();
        spawn_blocking(move || open_ambient(path.as_ref(), &options, ambient_authority))
            .await?
            .map(|f| Self::from_std(tokio::fs::File::from_std(f)))
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

#[cfg(not(target_os = "wasi"))]
#[inline]
fn metadata_from(file: &std::fs::File) -> io::Result<Metadata> {
    Metadata::from_file(file)
}

#[cfg(target_os = "wasi")]
#[inline]
fn metadata_from(file: &std::fs::File) -> io::Result<Metadata> {
    file.metadata()
}

#[cfg(not(target_os = "wasi"))]
#[inline]
fn permissions_into_std(
    file: &std::fs::File,
    permissions: Permissions,
) -> io::Result<std::fs::Permissions> {
    permissions.into_std(file)
}

#[cfg(target_os = "wasi")]
#[inline]
fn permissions_into_std(
    _file: &std::fs::File,
    permissions: Permissions,
) -> io::Result<std::fs::Permissions> {
    permissions
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
        self.std.as_raw_fd()
    }
}

#[cfg(not(windows))]
impl AsFd for File {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std.as_fd()
    }
}

#[cfg(windows)]
impl AsRawHandle for File {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.std.as_raw_handle()
    }
}

#[cfg(windows)]
impl AsHandle for File {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.std.as_handle()
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for File {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.std.as_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl AsHandleOrSocket for File {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.std.as_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl TryIntoRawFd for File {
    fn try_into_raw_fd(self) -> io::Result<RawFd> {
        use std::os::unix::io::IntoRawFd;
        let std_file = self.std.try_into_std().map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                "cannot convert tokio File: background operation in progress",
            )
        })?;
        Ok(std_file.into_raw_fd())
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
    fn try_into_raw_handle(self) -> io::Result<RawHandle> {
        use std::os::windows::io::IntoRawHandle;
        let std_file = self.std.try_into_std().map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                "cannot convert tokio File: background operation in progress",
            )
        })?;
        Ok(std_file.into_raw_handle())
    }
}

#[cfg(windows)]
impl TryFrom<File> for OwnedHandle {
    type Error = io::Error;
    #[inline]
    fn try_from(file: File) -> io::Result<OwnedHandle> {
        let raw = TryIntoRawHandle::try_into_raw_handle(file)?;
        Ok(unsafe { std::os::windows::io::OwnedHandle::from_raw_handle(raw) })
    }
}

#[cfg(windows)]
impl TryIntoRawHandleOrSocket for File {
    fn try_into_raw_handle_or_socket(
        self,
    ) -> std::io::Result<io_extras::os::windows::RawHandleOrSocket> {
        use TryIntoRawHandle;
        let raw = self.try_into_raw_handle()?;
        Ok(io_extras::os::windows::RawHandleOrSocket::unowned_from_raw_handle(raw))
    }
}

#[cfg(windows)]
impl TryFrom<File> for io_extras::os::windows::OwnedHandleOrSocket {
    type Error = io::Error;
    fn try_from(file: File) -> io::Result<io_extras::os::windows::OwnedHandleOrSocket> {
        let handle: OwnedHandle = file.try_into()?;
        Ok(io_extras::os::windows::OwnedHandleOrSocket::from_handle(
            handle,
        ))
    }
}

impl AsyncRead for File {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        AsyncRead::poll_read(Pin::new(&mut self.std), cx, buf)
    }
}

impl AsyncWrite for File {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        AsyncWrite::poll_write(Pin::new(&mut self.std), cx, buf)
    }

    #[inline]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        AsyncWrite::poll_flush(Pin::new(&mut self.std), cx)
    }

    #[inline]
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        AsyncWrite::poll_shutdown(Pin::new(&mut self.std), cx)
    }

    #[inline]
    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        AsyncWrite::poll_write_vectored(Pin::new(&mut self.std), cx, bufs)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.std.is_write_vectored()
    }
}

impl AsyncSeek for File {
    #[inline]
    fn start_seek(mut self: Pin<&mut Self>, position: io::SeekFrom) -> io::Result<()> {
        Pin::new(&mut self.std).start_seek(position)
    }

    #[inline]
    fn poll_complete(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<u64>> {
        Pin::new(&mut self.std).poll_complete(cx)
    }
}

impl fmt::Debug for File {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b = f.debug_struct("File");
        #[cfg(not(windows))]
        b.field("fd", &self.std.as_raw_fd());
        #[cfg(windows)]
        b.field("handle", &self.std.as_raw_handle());
        b.finish()
    }
}
