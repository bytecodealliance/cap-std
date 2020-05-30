#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::unix::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};
use std::{fs, io, process};

/// A reference to an open file on the filesystem.
///
/// This corresponds to [`std::fs::File`].
///
/// Note that this `File` has no `open` or `create` methods. To open or create
/// a file, you must first obtain a [`Dir`] containing the file, and then call
/// [`Dir::open_file`] or [`Dir::create_file`].
///
/// [`std::fs::File`]: https://doc.rust-lang.org/std/fs/struct.File.html
/// [`Dir`]: struct.Dir.html
/// [`Dir::open_file`]: struct.Dir.html#method.open_file
/// [`Dir::create_file`]: struct.Dir.html#method.create_file
pub struct File {
    file: fs::File,
}

impl File {
    /// Constructs a new instance of `Self` from the given `std::fs::File`.
    ///
    /// This corresponds to [`std::fs::File::from_raw_fd`].
    ///
    /// [`std::fs::File::from_raw_fd`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.from_raw_fd
    pub fn from_fs_file(file: fs::File) -> Self {
        Self { file }
    }

    /// Attempts to sync all OS-internal metadata to disk.
    ///
    /// This corresponds to [`std::fs::File::sync_all`].
    ///
    /// [`std::fs::File::sync_all`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all
    pub fn sync_all(&self) -> io::Result<()> {
        self.file.sync_all()
    }

    /// This function is similar to `sync_all`, except that it may not synchronize
    /// file metadata to the filesystem.
    ///
    /// This corresponds to [`std::fs::File::sync_data`].
    ///
    /// [`std::fs::File::sync_data`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_data
    pub fn sync_data(&self) -> io::Result<()> {
        self.file.sync_data()
    }

    /// Truncates or extends the underlying file, updating the size of this file
    /// to become size.
    ///
    /// This corresponds to [`std::fs::File::set_len`].
    ///
    /// [`std::fs::File::set_len`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.set_len
    pub fn set_len(&self, size: u64) -> io::Result<()> {
        self.file.set_len(size)
    }

    /// Queries metadata about the underlying file.
    ///
    /// This corresponds to [`std::fs::File::metadata`].
    ///
    /// [`std::fs::File::metadata`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.metadata
    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        self.file.metadata()
    }
}

#[cfg(unix)]
impl FromRawFd for File {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        File::from_fs_file(fs::File::from_raw_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawHandle for File {
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        File::from_fs_file(fs::File::from_raw_handle(handle))
    }
}

#[cfg(unix)]
impl AsRawFd for File {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawHandle for File {
    fn as_raw_handle(&self) -> RawHandle {
        self.file.as_raw_handle()
    }
}

#[cfg(unix)]
impl IntoRawFd for File {
    fn into_raw_fd(self) -> RawFd {
        self.file.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawHandle for File {
    fn into_raw_handle(self) -> RawHandle {
        self.file.into_raw_handle()
    }
}

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut]) -> io::Result<usize> {
        self.file.read_vectored(bufs)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.file.read_exact(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.file.read_to_end(buf)
    }

    // TODO: nightly-only APIs initializer?
}

impl io::Write for File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice]) -> io::Result<usize> {
        self.file.write_vectored(bufs)
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.file.write_all(buf)
    }
}

impl io::Seek for File {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.file.seek(pos)
    }

    // TODO: nightly-only APIs stream_len, stream_position?
}

impl From<File> for process::Stdio {
    fn from(file: File) -> process::Stdio {
        From::<fs::File>::from(file.file)
    }
}

// TODO: functions from FileExt?

// TODO: Use winx to implement "unix" FileExt api on Windows?

// TODO: impl Debug for File? But don't expose File's path...
