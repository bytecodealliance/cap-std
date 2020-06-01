use crate::fs::{DirBuilder, File, Metadata, OpenOptions, Permissions, ReadDir};
#[cfg(unix)]
use crate::os::unix::net::{UnixDatagram, UnixListener, UnixStream};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// A reference to an open directory on the filesystem.
///
/// TODO: Add OFlag::CLOEXEC to yanix and use it in `open_file` and friends.
///
/// TODO: Windows support.
///
/// Unlike `std::fs`, this API's `canonicalize` returns a relative path since
/// absolute paths don't interoperate well with the capability-oriented security
/// model.
pub struct Dir {
    file: fs::File,
}

impl Dir {
    /// Constructs a new instance of `Self` from the given `std::fs::File`.
    #[inline]
    pub fn from_ambient(file: fs::File) -> Self {
        Self { file }
    }

    /// Attempts to open a file in read-only mode.
    ///
    /// This corresponds to [`std::fs::File::open`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::File::open`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
    pub fn open_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<File> {
        let path = path.as_ref();

        #[cfg(unix)]
        {
            use yanix::file::{openat, Mode, OFlag};
            unsafe {
                let fd = openat(self.file.as_raw_fd(), path, OFlag::RDONLY, Mode::empty())?;
                Ok(File::from_raw_fd(fd))
            }
        }
    }

    /// Opens a file at `path` with the options specified by `self`.
    ///
    /// This corresponds to [`std::fs::OpenOptions::open`].
    ///
    /// Instead of being a method on `OpenOptions`, this is a method on `Dir`,
    /// and it only accesses functions relative to `self`.
    ///
    /// [`std::fs::OpenOptions::open`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.open
    pub fn open_file_with<P: AsRef<Path>>(
        &mut self,
        path: P,
        options: &OpenOptions,
    ) -> io::Result<File> {
        unimplemented!(
            "Dir::open_file_with({}, {:?})",
            path.as_ref().display(),
            options
        );
    }

    /// Attempts to open a directory.
    pub fn open_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<Self> {
        let path = path.as_ref();

        #[cfg(unix)]
        {
            use yanix::file::{openat, Mode, OFlag};
            unsafe {
                let fd = openat(
                    self.file.as_raw_fd(),
                    path,
                    OFlag::RDONLY | OFlag::DIRECTORY,
                    Mode::empty(),
                )?;
                Ok(Self::from_raw_fd(fd))
            }
        }
    }

    /// Creates a new, empty directory at the provided path.
    ///
    /// This corresponds to [`std::fs::create_dir`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::create_dir`]: https://doc.rust-lang.org/std/fs/fn.create_dir.html
    pub fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        unimplemented!("Dir::create_dir({})", path.as_ref().display())
    }

    /// Recursively create a directory and all of its parent components if they are missing.
    ///
    /// This corresponds to [`std::fs::create_dir_all`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::create_dir_all`]: https://doc.rust-lang.org/std/fs/fn.create_dir_all.html
    pub fn create_dir_all<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        unimplemented!("Dir::create_dir_all({})", path.as_ref().display())
    }

    /// Opens a file in write-only mode.
    ///
    /// This corresponds to [`std::fs::File::create`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::File::create`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
    pub fn create_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<File> {
        let path = path.as_ref();

        #[cfg(unix)]
        {
            use yanix::file::{openat, Mode, OFlag};
            unsafe {
                let fd = openat(
                    self.file.as_raw_fd(),
                    path,
                    OFlag::WRONLY | OFlag::CREAT | OFlag::TRUNC,
                    Mode::from_bits(0o666).ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "unrecognized mode flags")
                    })?,
                )?;
                Ok(File::from_raw_fd(fd))
            }
        }
    }

    /// Returns the canonical form of a path with all intermediate components normalized
    /// and symbolic links resolved.
    ///
    /// This corresponds to [`std::fs::canonicalize`], but instead of returning an
    /// absolute path, returns a path relative to the directory represented by `self`.
    ///
    /// [`std::fs::canonicalize`]: https://doc.rust-lang.org/std/fs/fn.canonicalize.html
    pub fn canonicalize<P: AsRef<Path>>(&mut self, path: P) -> io::Result<PathBuf> {
        // TODO Implement canoncalize without returning an absolute path.
        unimplemented!("Dir::canonicalize({})", path.as_ref().display())
    }

    /// Copies the contents of one file to another. This function will also copy the permission
    /// bits of the original file to the destination file.
    ///
    /// This corresponds to [`std::fs::copy`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::copy`]: https://doc.rust-lang.org/std/fs/fn.copy.html
    pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
        unimplemented!(
            "Dir::copy({}, {})",
            from.as_ref().display(),
            to.as_ref().display()
        )
    }

    /// Creates a new hard link on the filesystem.
    ///
    /// This corresponds to [`std::fs::hard_link`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::hard_link`]: https://doc.rust-lang.org/std/fs/fn.hard_link.html
    pub fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
        unimplemented!(
            "Dir::hard_link({}, {})",
            src.as_ref().display(),
            dst.as_ref().display()
        )
    }

    /// Given a path, query the file system to get information about a file, directory, etc.
    ///
    /// This corresponds to [`std::fs::metadata`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::metadata`]: https://doc.rust-lang.org/std/fs/fn.metadata.html
    pub fn metadata<P: AsRef<Path>>(&mut self, path: P) -> io::Result<Metadata> {
        unimplemented!("Dir::metadata({})", path.as_ref().display())
    }

    /// Returns an iterator over the entries within a directory.
    ///
    /// This corresponds to [`std::fs::read_dir`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::read_dir`]: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    pub fn read_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<ReadDir> {
        fs::read_dir(path).map(ReadDir::from_ambient)
    }

    /// Read the entire contents of a file into a bytes vector.
    ///
    /// This corresponds to [`std::fs::read`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::read`]: https://doc.rust-lang.org/std/fs/fn.read.html
    pub fn read_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<Vec<u8>> {
        use io::Read;
        let mut file = self.open_file(path)?;
        let mut bytes = Vec::with_capacity(initial_buffer_size(&file));
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// This corresponds to [`std::fs::read_link`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::read_link`]: https://doc.rust-lang.org/std/fs/fn.read_link.html
    pub fn read_link<P: AsRef<Path>>(&mut self, path: P) -> io::Result<PathBuf> {
        unimplemented!("Dir::read_link({})", path.as_ref().display())
    }

    /// Read the entire contents of a file into a string.
    ///
    /// This corresponds to [`std::fs::read_to_string`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::read_to_string`]: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
    pub fn read_to_string<P: AsRef<Path>>(&mut self, path: P) -> io::Result<String> {
        unimplemented!("Dir::read_to_string({})", path.as_ref().display())
    }

    /// Removes an existing, empty directory.
    ///
    /// This corresponds to [`std::fs::remove_dir`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::remove_dir`]: https://doc.rust-lang.org/std/fs/fn.remove_dir.html
    pub fn remove_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        unimplemented!("Dir::remove_dir({})", path.as_ref().display())
    }

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    ///
    /// This corresponds to [`std::fs::remove_dir_all`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::remove_dir_all`]: https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html
    pub fn remove_dir_all<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        unimplemented!("Dir::remove_dir_all({})", path.as_ref().display())
    }

    /// Removes a file from the filesystem.
    ///
    /// This corresponds to [`std::fs::remove_file`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::remove_file`]: https://doc.rust-lang.org/std/fs/fn.remove_file.html
    pub fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        unimplemented!("Dir::remove_file({})", path.as_ref().display())
    }

    /// Rename a file or directory to a new name, replacing the original file if to already exists.
    ///
    /// This corresponds to [`std::fs::rename`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::rename`]: https://doc.rust-lang.org/std/fs/fn.rename.html
    pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
        unimplemented!(
            "Dir::rename({}, {})",
            from.as_ref().display(),
            to.as_ref().display()
        )
    }

    /// Changes the permissions found on a file or a directory.
    ///
    /// This corresponds to [`std::fs::set_permissions`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::set_permissions`]: https://doc.rust-lang.org/std/fs/fn.set_permissions.html
    pub fn set_permissions<P: AsRef<Path>>(
        &mut self,
        path: P,
        perm: Permissions,
    ) -> io::Result<()> {
        unimplemented!(
            "Dir::set_permissions({}, {:?})",
            path.as_ref().display(),
            perm
        )
    }

    /// Query the metadata about a file without following symlinks.
    ///
    /// This corresponds to [`std::fs::symlink_metadata`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::symlink_metadata`]: https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html
    pub fn symlink_metadata<P: AsRef<Path>>(&mut self, path: P) -> io::Result<Metadata> {
        unimplemented!("Dir::symlink_metadata({:?}", path.as_ref().display())
    }

    /// Write a slice as the entire contents of a file.
    ///
    /// This corresponds to [`std::fs::write`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::write`]: https://doc.rust-lang.org/std/fs/fn.write.html
    pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(
        &mut self,
        path: P,
        contents: C,
    ) -> io::Result<()> {
        use io::Write;
        let mut file = self.create_file(path)?;
        file.write_all(contents.as_ref())
    }

    /// Creates the specified directory with the options configured in this builder.
    ///
    /// This corresponds to [`std::fs::DirBuilder::create`].
    ///
    /// [`std::fs::DirBuilder::create`]: https://doc.rust-lang.org/std/fs/struct.DirBuilder.html#method.create
    #[inline]
    pub fn create_with_dir_builder<P: AsRef<Path>>(
        &self,
        dir_builder: &DirBuilder,
        path: P,
    ) -> io::Result<()> {
        unimplemented!("Dir::create_with_dir_builder({})", path.as_ref().display())
    }

    /// Creates a new symbolic link on the filesystem.
    ///
    /// This corresponds to [`std::os::unix::fs::symlink`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::os::unix::fs::symlink`]: https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html
    #[cfg(unix)]
    pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, src: P, dst: Q) -> io::Result<()> {
        unimplemented!(
            "Dir::symlink({}, {})",
            src.as_ref().display(),
            dst.as_ref().display()
        )
    }

    /// Creates a new `UnixListener` bound to the specified socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixListener::bind`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`std::os::unix::net::UnixListener::bind`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.bind
    #[cfg(unix)]
    pub fn bind_unix_listener<P: AsRef<Path>>(&mut self, path: P) -> io::Result<UnixListener> {
        unimplemented!("Dir::bind_unix_listener({})", path.as_ref().display())
    }

    /// Connects to the socket named by path.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::connect`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`std::os::unix::net::UnixStream::connect`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.connect
    #[cfg(unix)]
    pub fn connect_unix_stream<P: AsRef<Path>>(&mut self, path: P) -> io::Result<UnixStream> {
        unimplemented!("Dir::connect_unix_stream({})", path.as_ref().display())
    }

    /// Creates a Unix datagram socket bound to the given path.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::bind`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`std::os::unix::net::UnixDatagram::bind`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.bind
    #[cfg(unix)]
    pub fn bind_unix_datagram<P: AsRef<Path>>(&mut self, path: P) -> io::Result<UnixDatagram> {
        unimplemented!("Dir::bind_unix_datagram({})", path.as_ref().display())
    }

    /// Connects the socket to the specified address.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::connect`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`std::os::unix::net::UnixDatagram::connect`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.connect
    #[cfg(unix)]
    pub fn connect_unix_datagram<P: AsRef<Path>>(
        &mut self,
        unix_datagram: &UnixDatagram,
        path: P,
    ) -> io::Result<()> {
        unimplemented!("Dir::connect_unix_datagram({})", path.as_ref().display())
    }

    /// Sends data on the socket to the specified address.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::send_to`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`std::os::unix::net::UnixDatagram::send_to`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.send_to
    #[cfg(unix)]
    pub fn send_to_unix_datagram_addr<P: AsRef<Path>>(
        &mut self,
        unix_datagram: &UnixDatagram,
        buf: &[u8],
        path: P,
    ) -> io::Result<usize> {
        unimplemented!(
            "Dir::send_to_unix_datagram_addr({:?}, {})",
            buf,
            path.as_ref().display()
        )
    }
}

#[cfg(unix)]
impl FromRawFd for Dir {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_ambient(fs::File::from_raw_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawHandle for Dir {
    unsafe fn from_raw_fd(handle: RawHandle) -> Self {
        Self::from_ambient(fs::File::from_raw_handle(handle))
    }
}

#[cfg(unix)]
impl AsRawFd for Dir {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawHandle for Dir {
    fn as_raw_handle(&self) -> RawHandle {
        self.file.as_raw_handle()
    }
}

#[cfg(unix)]
impl IntoRawFd for Dir {
    fn into_raw_fd(self) -> RawFd {
        self.file.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawHandle for Dir {
    fn into_raw_handle(self) -> RawHandle {
        self.file.into_raw_handle()
    }
}

/// Indicates how large a buffer to pre-allocate before reading the entire file.
///
/// Derived from the function of the same name in libstd.
fn initial_buffer_size(file: &File) -> usize {
    // Allocate one extra byte so the buffer doesn't need to grow before the
    // final `read` call at the end of the file.  Don't worry about `usize`
    // overflow because reading will fail regardless in that case.
    file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0)
}

// TODO: impl Debug for Dir? But don't expose Dir's path...
