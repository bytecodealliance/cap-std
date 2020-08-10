use crate::fs::{Metadata, Permissions};
use cap_primitives::fs::flags;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};
#[cfg(target_os = "wasi")]
use std::path::Path;
use std::{fmt, fs, io, process};

/// A reference to an open file on a filesystem.
///
/// This corresponds to [`std::fs::File`].
///
/// Note that this `File` has no `open` or `create` methods. To open or create
/// a file, you must first obtain a [`Dir`] containing the path, and then call
/// [`Dir::open`] or [`Dir::create`].
///
/// [`std::fs::File`]: https://doc.rust-lang.org/std/fs/struct.File.html
/// [`Dir`]: struct.Dir.html
/// [`Dir::open`]: struct.Dir.html#method.open
/// [`Dir::create`]: struct.Dir.html#method.create
pub struct File {
    pub(crate) std: fs::File,
}

impl File {
    /// Constructs a new instance of `Self` from the given `std::fs::File`.
    #[inline]
    pub fn from_std(std: fs::File) -> Self {
        Self { std }
    }

    /// Attempts to sync all OS-internal metadata to disk.
    ///
    /// This corresponds to [`std::fs::File::sync_all`].
    ///
    /// [`std::fs::File::sync_all`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all
    #[inline]
    pub fn sync_all(&self) -> io::Result<()> {
        self.std.sync_all()
    }

    /// This function is similar to `sync_all`, except that it may not synchronize
    /// file metadata to a filesystem.
    ///
    /// This corresponds to [`std::fs::File::sync_data`].
    ///
    /// [`std::fs::File::sync_data`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_data
    #[inline]
    pub fn sync_data(&self) -> io::Result<()> {
        self.std.sync_data()
    }

    /// Truncates or extends the underlying file, updating the size of this file
    /// to become size.
    ///
    /// This corresponds to [`std::fs::File::set_len`].
    ///
    /// [`std::fs::File::set_len`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.set_len
    #[inline]
    pub fn set_len(&self, size: u64) -> io::Result<()> {
        self.std.set_len(size)
    }

    /// Queries metadata about the underlying file.
    ///
    /// This corresponds to [`std::fs::File::metadata`].
    ///
    /// [`std::fs::File::metadata`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.metadata
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.std.metadata().map(metadata_from_std)
    }

    /// Creates a new `File` instance that shares the same underlying file handle as the existing
    /// `File` instance.
    ///
    /// This corresponds to [`std::fs::File::try_clone`].
    ///
    /// [`std::fs::File::try_clone`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.try_clone
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self::from_std(self.std.try_clone()?))
    }

    /// Changes the permissions on the underlying file.
    ///
    /// This corresponds to [`std::fs::File::set_permissions`].
    ///
    /// [`std::fs::File::set_permissions`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.set_permissions
    #[inline]
    pub fn set_permissions(&self, perm: Permissions) -> io::Result<()> {
        self.std
            .set_permissions(permissions_into_std(&self.std, perm)?)
    }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
fn metadata_from_std(metadata: fs::Metadata) -> Metadata {
    Metadata::from_std(metadata)
}

#[cfg(target_os = "wasi")]
#[inline]
fn metadata_from_std(metadata: fs::Metadata) -> Metadata {
    metadata
}

#[cfg(not(target_os = "wasi"))]
#[inline]
fn permissions_into_std(file: &fs::File, permissions: Permissions) -> io::Result<fs::Permissions> {
    permissions.into_std(file)
}

#[cfg(target_os = "wasi")]
#[inline]
fn permissions_into_std(_file: &fs::File, permissions: Permissions) -> io::Result<fs::Permissions> {
    permissions
}

#[cfg(not(windows))]
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

#[cfg(not(windows))]
impl AsRawFd for File {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawHandle for File {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.std.as_raw_handle()
    }
}

#[cfg(not(windows))]
impl IntoRawFd for File {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawHandle for File {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.std.into_raw_handle()
    }
}

impl io::Read for File {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut]) -> io::Result<usize> {
        self.std.read_vectored(bufs)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.std.read_exact(buf)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.std.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.std.read_to_string(buf)
    }

    // TODO: nightly-only APIs initializer?
}

impl io::Write for File {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.std.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.std.flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[io::IoSlice]) -> io::Result<usize> {
        self.std.write_vectored(bufs)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.std.write_all(buf)
    }
}

impl io::Seek for File {
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.std.seek(pos)
    }

    // TODO: nightly-only APIs stream_len, stream_position?
}

impl From<File> for process::Stdio {
    #[inline]
    fn from(file: File) -> Self {
        From::<fs::File>::from(file.std)
    }
}

#[cfg(unix)]
impl std::os::unix::fs::FileExt for File {
    #[inline]
    fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        self.std.read_at(buf, offset)
    }

    #[inline]
    fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        self.std.write_at(buf, offset)
    }

    #[inline]
    fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> io::Result<()> {
        self.std.read_exact_at(buf, offset)
    }

    #[inline]
    fn write_all_at(&self, buf: &[u8], offset: u64) -> io::Result<()> {
        self.std.write_all_at(buf, offset)
    }
}

#[cfg(target_os = "wasi")]
impl std::os::wasi::fs::FileExt for File {
    #[inline]
    fn read_at(&self, bufs: &mut [io::IoSliceMut], offset: u64) -> io::Result<usize> {
        self.std.read_at(bufs, offset)
    }

    #[inline]
    fn write_at(&self, bufs: &[io::IoSlice], offset: u64) -> io::Result<usize> {
        self.std.write_at(bufs, offset)
    }

    #[inline]
    fn tell(&self) -> std::result::Result<u64, std::io::Error> {
        self.std.tell()
    }

    #[inline]
    fn fdstat_set_flags(&self, flags: u16) -> std::result::Result<(), std::io::Error> {
        self.std.fdstat_set_flags(flags)
    }

    #[inline]
    fn fdstat_set_rights(
        &self,
        rights: u64,
        inheriting: u64,
    ) -> std::result::Result<(), std::io::Error> {
        self.std.fdstat_set_rights(rights, inheriting)
    }

    #[inline]
    fn advise(&self, offset: u64, len: u64, advice: u8) -> std::result::Result<(), std::io::Error> {
        self.std.advise(offset, len, advice)
    }

    #[inline]
    fn allocate(&self, offset: u64, len: u64) -> std::result::Result<(), std::io::Error> {
        self.std.allocate(offset, len)
    }

    #[inline]
    fn create_directory<P: AsRef<Path>>(&self, dir: P) -> std::result::Result<(), std::io::Error> {
        self.std.create_directory(dir)
    }

    #[inline]
    fn read_link<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> std::result::Result<std::path::PathBuf, std::io::Error> {
        self.std.read_link(path)
    }

    #[inline]
    fn metadata_at<P: AsRef<Path>>(
        &self,
        lookup_flags: u32,
        path: P,
    ) -> std::result::Result<std::fs::Metadata, std::io::Error> {
        self.std.metadata_at(lookup_flags, path)
    }

    #[inline]
    fn remove_file<P: AsRef<Path>>(&self, path: P) -> std::result::Result<(), std::io::Error> {
        self.std.remove_file(path)
    }

    #[inline]
    fn remove_directory<P: AsRef<Path>>(&self, path: P) -> std::result::Result<(), std::io::Error> {
        self.std.remove_directory(path)
    }
}

#[cfg(windows)]
impl std::os::windows::fs::FileExt for File {
    #[inline]
    fn seek_read(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        self.std.seek_read(buf, offset)
    }

    #[inline]
    fn seek_write(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        self.std.seek_write(buf, offset)
    }
}

impl fmt::Debug for File {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b = f.debug_struct("File");
        fmt_debug_file(&self.std, &mut b);
        if let Ok((read, write)) = flags(&self.std) {
            b.field("read", &read).field("write", &write);
        }
        b.finish()
    }
}

#[cfg(not(windows))]
fn fmt_debug_file(file: &fs::File, b: &mut fmt::DebugStruct) {
    let fd = file.as_raw_fd();
    b.field("fd", &fd);
}

#[cfg(windows)]
fn fmt_debug_file(file: &fs::File, b: &mut fmt::DebugStruct) {
    let handle = file.as_raw_handle();
    b.field("handle", &handle);
}
