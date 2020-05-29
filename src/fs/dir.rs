use crate::fs::{File, Metadata, OpenOptions, Permissions, ReadDir};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
#[cfg(windows)]
use std::os::unix::io::{AsRawHandle, FromRawHandle, RawHandle};
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
/// Unlike `std::fs`, this API has no `canonicalize`, because absolute paths
/// don't interoperate well with the capability-oriented security model.
pub struct Dir {
    file: fs::File,
}

impl Dir {
    /// Constructs a new instance of `Self` from the given file.
    pub fn from_fs_file(file: fs::File) -> Self {
        Self { file }
    }

    /// Attempts to open a file in read-only mode.
    ///
    /// This corresponds to [`std::fs::File::open`], but only accesses paths
    /// relative to and within `self`.
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
    /// and it only accesses functions relative to and within `self`.
    ///
    /// [`std::fs::OpenOptions::open`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.open
    pub fn open_file_with<P: AsRef<Path>>(
        &mut self,
        path: P,
        options: &OpenOptions,
    ) -> io::Result<File> {
        unimplemented!("Dir::open_file_with");
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
    /// relative to and within `self`.
    ///
    /// [`std::fs::create_dir`]: https://doc.rust-lang.org/std/fs/fn.create_dir.html
    pub fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        unimplemented!("Dir::create_dir")
    }

    /// Recursively create a directory and all of its parent components if they are missing.
    ///
    /// This corresponds to [`std::fs::create_dir_all`], but only accesses paths
    /// relative to and within `self`.
    ///
    /// [`std::fs::create_dir_all`]: https://doc.rust-lang.org/std/fs/fn.create_dir_all.html
    pub fn create_dir_all<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        unimplemented!("Dir::create_dir_all")
    }

    /// Opens a file in write-only mode.
    ///
    /// This corresponds to [`std::fs::File::create`], but only accesses paths
    /// relative to and within `self`.
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

    /// Returns the canonical, absolute form of a path with all intermediate components normalized
    /// and symbolic links resolved.
    ///
    /// This corresponds to [`std::fs::canonicalize`], but only accesses paths
    /// relative to and within `self`.
    ///
    /// [`std::fs::canonicalize`]: https://doc.rust-lang.org/std/fs/fn.canonicalize.html
    pub fn canonicalize<P: AsRef<Path>>(&mut self, path: P) -> io::Result<PathBuf> {
        unimplemented!("Dir::canonicalize")
    }

    /// Copies the contents of one file to another. This function will also copy the permission
    /// bits of the original file to the destination file.
    ///
    /// This corresponds to [`std::fs::copy`], but only accesses paths
    /// relative to and within `self`.
    ///
    /// [`std::fs::copy`]: https://doc.rust-lang.org/std/fs/fn.copy.html
    pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
        unimplemented!("Dir::copy")
    }

    /// Creates a new hard link on the filesystem.
    ///
    /// This corresponds to [`std::fs::hard_link`], but only accesses paths
    /// relative to and within `self`.
    ///
    /// [`std::fs::hard_link`]: https://doc.rust-lang.org/std/fs/fn.hard_link.html
    pub fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
        unimplemented!("Dir::hard_link")
    }

    /// Given a path, query the file system to get information about a file, directory, etc.
    ///
    /// This corresponds to [`std::fs::metadata`], but only accesses paths
    /// relative to and within `self`.
    ///
    /// [`std::fs::metadata`]: https://doc.rust-lang.org/std/fs/fn.metadata.html
    pub fn metadata<P: AsRef<Path>>(&mut self, path: P) -> io::Result<Metadata> {
        unimplemented!("Dir::metadata")
    }

    /// Returns an iterator over the entries within a directory.
    ///
    /// This corresponds to [`std::fs::read_dir`], but reads the directory
    /// represented by `self`.
    ///
    /// [`std::fs::read_dir`]: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    pub fn read(&mut self) -> io::Result<ReadDir> {
        Ok(ReadDir::from_fs_file(self.file.try_clone()?))
    }

    /// Consumes self and returns an iterator over the entries within a directory
    /// in the manner of `read`.
    pub fn into_read(self) -> ReadDir {
        ReadDir::from_fs_file(self.file)
    }

    /// Returns an iterator over the entries within a directory.
    ///
    /// This corresponds to [`std::fs::read_dir`], but only accesses paths
    /// relative to and within `self`.
    ///
    /// [`std::fs::read_dir`]: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    pub fn read_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<ReadDir> {
        self.open_dir(path)?.read()
    }

    /// Read the entire contents of a file into a bytes vector.
    ///
    /// This corresponds to [`std::fs::read`], but only accesses paths
    /// relative to and within `self`.
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
    /// relative to and within `self`.
    ///
    /// [`std::fs::read_link`]: https://doc.rust-lang.org/std/fs/fn.read_link.html
    pub fn read_link<P: AsRef<Path>>(&mut self, path: P) -> io::Result<PathBuf> {
        unimplemented!("Dir::read_link")
    }

    /// Read the entire contents of a file into a string.
    ///
    /// This corresponds to [`std::fs::read_to_string`], but only accesses paths
    /// relative to and within `self`.
    ///
    /// [`std::fs::read_to_string`]: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
    pub fn read_to_string<P: AsRef<Path>>(&mut self, path: P) -> io::Result<String> {
        unimplemented!("Dir::read_to_string")
    }

    /// Removes an existing, empty directory.
    ///
    /// This corresponds to [`std::fs::remove_dir`], but only accesses paths
    /// relative to and within `self`.
    ///
    /// [`std::fs::remove_dir`]: https://doc.rust-lang.org/std/fs/fn.remove_dir.html
    pub fn remove_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        unimplemented!("Dir::remove_dir")
    }

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    ///
    /// This corresponds to [`std::fs::remove_dir_all`], but only accesses paths
    /// relative to and within `self`.
    ///
    /// [`std::fs::remove_dir_all`]: https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html
    pub fn remove_dir_all<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        unimplemented!("Dir::remove_dir_all")
    }

    /// Removes a file from the filesystem.
    ///
    /// This corresponds to [`std::fs::remove_file`], but only accesses paths
    /// relative to and within `self`.
    ///
    /// [`std::fs::remove_file`]: https://doc.rust-lang.org/std/fs/fn.remove_file.html
    pub fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        unimplemented!("Dir::remove_file")
    }

    /// Rename a file or directory to a new name, replacing the original file if to already exists.
    ///
    /// This corresponds to [`std::fs::rename`], but only accesses paths
    /// relative to and within `self`.
    ///
    /// [`std::fs::rename`]: https://doc.rust-lang.org/std/fs/fn.rename.html
    pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
        unimplemented!("Dir::rename")
    }

    /// Changes the permissions found on a file or a directory.
    ///
    /// This corresponds to [`std::fs::set_permissions`], but only accesses paths
    /// relative to and within `self`.
    ///
    /// [`std::fs::set_permissions`]: https://doc.rust-lang.org/std/fs/fn.set_permissions.html
    pub fn set_permissions<P: AsRef<Path>>(
        &mut self,
        path: P,
        perm: Permissions,
    ) -> io::Result<()> {
        unimplemented!("Dir::set_permissions")
    }

    /// Query the metadata about a file without following symlinks.
    ///
    /// This corresponds to [`std::fs::symlink_metadata`], but only accesses paths
    /// relative to and within `self`.
    ///
    /// [`std::fs::symlink_metadata`]: https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html
    pub fn symlink_metadata<P: AsRef<Path>>(&mut self, path: P) -> io::Result<Metadata> {
        unimplemented!("Dir::symplink_metadata")
    }

    /// Write a slice as the entire contents of a file.
    ///
    /// This corresponds to [`std::fs::write`], but only accesses paths
    /// relative to and within `self`.
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
}

#[cfg(unix)]
impl FromRawFd for Dir {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Dir::from_fs_file(fs::File::from_raw_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawHandle for Dir {
    unsafe fn from_raw_fd(handle: RawHandle) -> Self {
        Dir::from_fs_file(fs::File::from_raw_handle(handle))
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
