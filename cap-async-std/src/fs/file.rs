use crate::fs::{Metadata, OpenOptions, Permissions};
use async_std::fs;
use async_std::io::{self, IoSlice, IoSliceMut, Read, Seek, SeekFrom, Write};
#[cfg(unix)]
use async_std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(target_os = "wasi")]
use async_std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use async_std::task::{spawn_blocking, Context, Poll};
use cap_primitives::fs::{is_file_read_write, open_ambient};
use cap_primitives::AmbientAuthority;
use io_lifetimes::AsFilelike;
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, FromFd, IntoFd, OwnedFd};
#[cfg(windows)]
use io_lifetimes::{AsHandle, BorrowedHandle, FromHandle, IntoHandle, OwnedHandle};
use std::fmt;
use std::path::Path;
use std::pin::Pin;
#[cfg(windows)]
use {
    async_std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle},
    io_extras::os::windows::{AsRawHandleOrSocket, IntoRawHandleOrSocket, RawHandleOrSocket},
};

/// A reference to an open file on a filesystem.
///
/// This corresponds to [`async_std::fs::File`].
///
/// Note that this `File` has no `open` or `create` methods. To open or create
/// a file, you must first obtain a [`Dir`] containing the path, and then call
/// [`Dir::open`] or [`Dir::create`].
///
/// [`Dir`]: crate::fs::Dir
/// [`Dir::open`]: crate::fs::Dir::open
/// [`Dir::create`]: crate::fs::Dir::create
#[derive(Clone)]
pub struct File {
    pub(crate) std: fs::File,
}

impl File {
    /// Constructs a new instance of `Self` from the given
    /// `async_std::fs::File`.
    ///
    /// This grants access the resources the `async_std::fs::File` instance
    /// already has access to.
    #[inline]
    pub fn from_std(std: fs::File) -> Self {
        Self { std }
    }

    /// Consumes `self` and returns an `async_std::fs::File`.
    #[inline]
    pub fn into_std(self) -> fs::File {
        self.std
    }

    // async_std doesn't have `with_options`.

    /// Attempts to sync all OS-internal metadata to disk.
    ///
    /// This corresponds to [`async_std::fs::File::sync_all`].
    #[inline]
    pub async fn sync_all(&self) -> io::Result<()> {
        self.std.sync_all().await
    }

    /// This function is similar to `sync_all`, except that it may not
    /// synchronize file metadata to a filesystem.
    ///
    /// This corresponds to [`async_std::fs::File::sync_data`].
    #[inline]
    pub async fn sync_data(&self) -> io::Result<()> {
        self.std.sync_data().await
    }

    /// Truncates or extends the underlying file, updating the size of this
    /// file to become size.
    ///
    /// This corresponds to [`async_std::fs::File::set_len`].
    #[inline]
    pub async fn set_len(&self, size: u64) -> io::Result<()> {
        self.std.set_len(size).await
    }

    /// Queries metadata about the underlying file.
    ///
    /// This corresponds to [`async_std::fs::File::metadata`].
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        let sync = self.std.as_filelike_view::<std::fs::File>();
        metadata_from(&*sync)
    }

    // async_std doesn't have `try_clone`.

    /// Changes the permissions on the underlying file.
    ///
    /// This corresponds to [`async_std::fs::File::set_permissions`].
    #[inline]
    pub async fn set_permissions(&self, perm: Permissions) -> io::Result<()> {
        let sync = self.std.as_filelike_view::<std::fs::File>();
        self.std
            .set_permissions(permissions_into_std(&sync, perm)?)
            .await
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
                &OpenOptions::new().read(true),
                ambient_authority,
            )
        })
        .await
        .map(|f| Self::from_std(f.into()))
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
            .await
            .map(|f| Self::from_std(f.into()))
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
) -> io::Result<fs::Permissions> {
    permissions.into_std(file)
}

#[cfg(target_os = "wasi")]
#[inline]
fn permissions_into_std(
    _file: &std::fs::File,
    permissions: Permissions,
) -> io::Result<fs::Permissions> {
    permissions
}

#[cfg(not(windows))]
impl FromRawFd for File {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(fs::File::from_raw_fd(fd))
    }
}

#[cfg(not(windows))]
impl FromFd for File {
    #[inline]
    fn from_fd(fd: OwnedFd) -> Self {
        Self::from_std(fs::File::from_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawHandle for File {
    #[inline]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self::from_std(fs::File::from_raw_handle(handle))
    }
}

#[cfg(windows)]
impl FromHandle for File {
    #[inline]
    fn from_handle(handle: OwnedHandle) -> Self {
        Self::from_std(fs::File::from_handle(handle))
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

#[cfg(not(windows))]
impl IntoRawFd for File {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

#[cfg(not(windows))]
impl IntoFd for File {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        self.std.into_fd()
    }
}

#[cfg(windows)]
impl IntoRawHandle for File {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.std.into_raw_handle()
    }
}

#[cfg(windows)]
impl IntoHandle for File {
    #[inline]
    fn into_handle(self) -> OwnedHandle {
        self.std.into_handle()
    }
}

#[cfg(windows)]
impl IntoRawHandleOrSocket for File {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.std.into_raw_handle_or_socket()
    }
}

impl Read for File {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Read::poll_read(Pin::new(&mut self.std), cx, buf)
    }

    #[inline]
    fn poll_read_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &mut [IoSliceMut],
    ) -> Poll<io::Result<usize>> {
        Read::poll_read_vectored(Pin::new(&mut self.std), cx, bufs)
    }

    // async_std doesn't have `is_read_vectored`.

    // async_std doesn't have `initializer`.
}

impl Read for &File {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Read::poll_read(Pin::new(&mut &self.std), cx, buf)
    }

    #[inline]
    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &mut [IoSliceMut],
    ) -> Poll<io::Result<usize>> {
        Read::poll_read_vectored(Pin::new(&mut &self.std), cx, bufs)
    }

    // async_std doesn't have `is_read_vectored`.

    // async_std doesn't have `initializer`.
}

impl Write for File {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Write::poll_write(Pin::new(&mut self.std), cx, buf)
    }

    #[inline]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Write::poll_flush(Pin::new(&mut self.std), cx)
    }

    #[inline]
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Write::poll_close(Pin::new(&mut self.std), cx)
    }

    #[inline]
    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &[IoSlice],
    ) -> Poll<io::Result<usize>> {
        Write::poll_write_vectored(Pin::new(&mut self.std), cx, bufs)
    }

    // async_std doesn't have `is_write_vectored`.

    // async_std doesn't have `write_all_vectored`.
}

impl Write for &File {
    #[inline]
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        Write::poll_write(Pin::new(&mut &self.std), cx, buf)
    }

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Write::poll_flush(Pin::new(&mut &self.std), cx)
    }

    #[inline]
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Write::poll_close(Pin::new(&mut &self.std), cx)
    }

    #[inline]
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &[IoSlice],
    ) -> Poll<io::Result<usize>> {
        Write::poll_write_vectored(Pin::new(&mut &self.std), cx, bufs)
    }

    // async_std doesn't have `is_write_vectored`.

    // async_std doesn't have `write_all_vectored`.
}

impl Seek for File {
    #[inline]
    fn poll_seek(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        pos: SeekFrom,
    ) -> Poll<io::Result<u64>> {
        Seek::poll_seek(Pin::new(&mut self.std), cx, pos)
    }

    // async_std doesn't have `seek_convenience`.
}

impl Seek for &File {
    #[inline]
    fn poll_seek(self: Pin<&mut Self>, cx: &mut Context, pos: SeekFrom) -> Poll<io::Result<u64>> {
        Seek::poll_seek(Pin::new(&mut &self.std), cx, pos)
    }

    // async_std doesn't have `seek_convenience`.
}

// TODO: Can async_std implement `From<File>` for `process::Stdio`?

// async_std doesn't have `FileExt`.

impl fmt::Debug for File {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b = f.debug_struct("File");
        let file = self.std.as_filelike_view::<std::fs::File>();
        #[cfg(not(windows))]
        b.field("fd", &file.as_raw_fd());
        #[cfg(windows)]
        b.field("handle", &file.as_raw_handle());
        if let Ok((read, write)) = is_file_read_write(&file) {
            b.field("read", &read).field("write", &write);
        }
        b.finish()
    }
}
