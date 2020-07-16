use crate::fs::{DirBuilder, File, Metadata, OpenOptions, ReadDir};
use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

#[cfg(any(unix, target_os = "fuchsia"))]
use {
    crate::os::unix::net::{UnixDatagram, UnixListener, UnixStream},
    cap_primitives::fs::{
        canonicalize, link, mkdir, open, readlink, rename, stat, symlink, unlink, FollowSymlinks,
    },
    std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd},
};

#[cfg(windows)]
use {
    cap_primitives::fs::{
        canonicalize, link, mkdir, open, readlink, rename, stat, symlink_dir, symlink_file, unlink,
        FollowSymlinks,
    },
    std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle},
};

#[cfg(target_os = "wasi")]
use std::os::wasi::{
    fs::OpenOptionsExt,
    io::{AsRawFd, IntoRawFd},
};

/// A reference to an open directory on a filesystem.
///
/// TODO: Windows support.
///
/// Unlike `std::fs`, this API's `canonicalize` returns a relative path since
/// absolute paths don't interoperate well with the capability model. And it lacks
/// a `set_permissions` method because popular host platforms don't have a way to
/// perform that operation in a manner compatible with cap-std's sandbox; instead,
/// open the file and call [`File::set_permissions`].
pub struct Dir {
    std_file: fs::File,
}

impl Dir {
    /// Constructs a new instance of `Self` from the given `std::fs::File`.
    #[inline]
    pub fn from_std_file(std_file: fs::File) -> Self {
        Self { std_file }
    }

    /// Consumes `self` and returns a `std::fs::File`.
    #[inline]
    pub fn into_std_file(self) -> fs::File {
        self.std_file
    }

    /// Attempts to open a file in read-only mode.
    ///
    /// This corresponds to [`std::fs::File::open`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::File::open`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
    #[inline]
    pub fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
        self.open_with(path, OpenOptions::new().read(true))
    }

    /// Opens a file at `path` with the options specified by `self`.
    ///
    /// This corresponds to [`std::fs::OpenOptions::open`].
    ///
    /// Instead of being a method on `OpenOptions`, this is a method on `Dir`,
    /// and it only accesses functions relative to `self`.
    ///
    /// [`std::fs::OpenOptions::open`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.open
    #[inline]
    pub fn open_with<P: AsRef<Path>>(&self, path: P, options: &OpenOptions) -> io::Result<File> {
        self._open_with(path.as_ref(), options)
    }

    #[cfg(not(target_os = "wasi"))]
    fn _open_with(&self, path: &Path, options: &OpenOptions) -> io::Result<File> {
        open(&self.std_file, path, options).map(File::from_std)
    }

    #[cfg(target_os = "wasi")]
    fn _open_with(&self, path: &Path, options: &OpenOptions) -> io::Result<File> {
        options.open_at(&self.std_file, path).map(File::from_std)
    }

    /// Attempts to open a directory.
    #[inline]
    pub fn open_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<Self> {
        self._open_dir(path.as_ref())
    }

    #[cfg(any(unix, target_os = "fuchsia"))]
    fn _open_dir(&self, path: &Path) -> io::Result<Self> {
        use std::os::unix::fs::OpenOptionsExt;
        use yanix::file::OFlag;
        self.open_with(
            path,
            OpenOptions::new()
                .read(true)
                .custom_flags(OFlag::DIRECTORY.bits()),
        )
        .map(|file| Self::from_std_file(file.std))
    }

    #[cfg(windows)]
    fn _open_dir(&self, path: &Path) -> io::Result<Self> {
        use std::os::windows::fs::OpenOptionsExt;
        use winapi::um::winbase::FILE_FLAG_BACKUP_SEMANTICS;
        self.open_with(
            path,
            OpenOptions::new()
                .read(true)
                .attributes(FILE_FLAG_BACKUP_SEMANTICS),
        )
        .map(|file| Self::from_std_file(file.std))
    }

    /// Creates a new, empty directory at the provided path.
    ///
    /// This corresponds to [`std::fs::create_dir`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::create_dir`]: https://doc.rust-lang.org/std/fs/fn.create_dir.html
    #[inline]
    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        mkdir(&self.std_file, path.as_ref())
    }

    /// Recursively create a directory and all of its parent components if they are missing.
    ///
    /// This corresponds to [`std::fs::create_dir_all`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::create_dir_all`]: https://doc.rust-lang.org/std/fs/fn.create_dir_all.html
    #[inline]
    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        self._create_dir_all(path.as_ref())
    }

    fn _create_dir_all(&self, path: &Path) -> io::Result<()> {
        if path == Path::new("") {
            return Ok(());
        }

        match self.create_dir(path) {
            Ok(()) => return Ok(()),
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(_) if self.is_dir(path) => return Ok(()),
            Err(e) => return Err(e),
        }
        match path.parent() {
            Some(p) => self._create_dir_all(p)?,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "failed to create whole tree",
                ));
            }
        }
        match self.create_dir(path) {
            Ok(()) => Ok(()),
            Err(_) if self.is_dir(path) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Opens a file in write-only mode.
    ///
    /// This corresponds to [`std::fs::File::create`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::File::create`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
    #[inline]
    pub fn create<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
        self.open_with(
            path,
            OpenOptions::new().write(true).create(true).truncate(true),
        )
    }

    /// Returns the canonical form of a path with all intermediate components normalized
    /// and symbolic links resolved.
    ///
    /// This corresponds to [`std::fs::canonicalize`], but instead of returning an
    /// absolute path, returns a path relative to the directory represented by `self`.
    ///
    /// [`std::fs::canonicalize`]: https://doc.rust-lang.org/std/fs/fn.canonicalize.html
    #[inline]
    pub fn canonicalize<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        canonicalize(&self.std_file, path.as_ref())
    }

    /// Copies the contents of one file to another. This function will also copy the permission
    /// bits of the original file to the destination file.
    ///
    /// This corresponds to [`std::fs::copy`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::copy`]: https://doc.rust-lang.org/std/fs/fn.copy.html
    #[inline]
    pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> io::Result<u64> {
        // Implementation derived from `copy` in Rust's
        // src/libstd/sys_common/fs.rs at revision
        // 7e11379f3b4c376fbb9a6c4d44f3286ccc28d149.
        if !self.is_file(from.as_ref()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "the source path is not an existing regular file",
            ));
        }

        let mut reader = self.open(from)?;
        let mut writer = self.create(to.as_ref())?;
        let perm = reader.metadata()?.permissions();

        let ret = io::copy(&mut reader, &mut writer)?;
        writer.set_permissions(perm)?;
        Ok(ret)
    }

    /// Creates a new hard link on a filesystem.
    ///
    /// This corresponds to [`std::fs::hard_link`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::hard_link`]: https://doc.rust-lang.org/std/fs/fn.hard_link.html
    #[inline]
    pub fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        src: P,
        dst_dir: &Self,
        dst: Q,
    ) -> io::Result<()> {
        link(
            &self.std_file,
            src.as_ref(),
            &dst_dir.std_file,
            dst.as_ref(),
        )
    }

    /// Given a path, query the file system to get information about a file, directory, etc.
    ///
    /// This corresponds to [`std::fs::metadata`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::metadata`]: https://doc.rust-lang.org/std/fs/fn.metadata.html
    #[inline]
    pub fn metadata<P: AsRef<Path>>(&self, path: P) -> io::Result<Metadata> {
        stat(&self.std_file, path.as_ref(), FollowSymlinks::Yes)
    }

    /// Returns an iterator over the entries within a directory.
    ///
    /// This corresponds to [`std::fs::read_dir`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::read_dir`]: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    #[inline]
    pub fn read_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<ReadDir> {
        todo!(
            "Dir::read_dir({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Read the entire contents of a file into a bytes vector.
    ///
    /// This corresponds to [`std::fs::read`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::read`]: https://doc.rust-lang.org/std/fs/fn.read.html
    #[inline]
    pub fn read<P: AsRef<Path>>(&self, path: P) -> io::Result<Vec<u8>> {
        use io::Read;
        let mut file = self.open(path)?;
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
    #[inline]
    pub fn read_link<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        readlink(&self.std_file, path.as_ref())
    }

    /// Read the entire contents of a file into a string.
    ///
    /// This corresponds to [`std::fs::read_to_string`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::read_to_string`]: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
    #[inline]
    pub fn read_to_string<P: AsRef<Path>>(&self, path: P) -> io::Result<String> {
        use std::io::Read;
        let mut s = String::new();
        self.open(path)?.read_to_string(&mut s)?;
        Ok(s)
    }

    /// Removes an existing, empty directory.
    ///
    /// This corresponds to [`std::fs::remove_dir`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::remove_dir`]: https://doc.rust-lang.org/std/fs/fn.remove_dir.html
    #[inline]
    pub fn remove_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        todo!(
            "Dir::remove_dir({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    ///
    /// This corresponds to [`std::fs::remove_dir_all`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::remove_dir_all`]: https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html
    #[inline]
    pub fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        todo!(
            "Dir::remove_dir_all({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Removes a file from a filesystem.
    ///
    /// This corresponds to [`std::fs::remove_file`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::remove_file`]: https://doc.rust-lang.org/std/fs/fn.remove_file.html
    #[inline]
    pub fn remove_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        unlink(&self.std_file, path.as_ref())
    }

    /// Rename a file or directory to a new name, replacing the original file if to already exists.
    ///
    /// This corresponds to [`std::fs::rename`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::rename`]: https://doc.rust-lang.org/std/fs/fn.rename.html
    #[inline]
    pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        from: P,
        to_dir: &Self,
        to: Q,
    ) -> io::Result<()> {
        rename(&self.std_file, from.as_ref(), &to_dir.std_file, to.as_ref())
    }

    /// Query the metadata about a file without following symlinks.
    ///
    /// This corresponds to [`std::fs::symlink_metadata`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::symlink_metadata`]: https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html
    #[inline]
    pub fn symlink_metadata<P: AsRef<Path>>(&self, path: P) -> io::Result<Metadata> {
        stat(&self.std_file, path.as_ref(), FollowSymlinks::No)
    }

    /// Write a slice as the entire contents of a file.
    ///
    /// This corresponds to [`std::fs::write`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::fs::write`]: https://doc.rust-lang.org/std/fs/fn.write.html
    #[inline]
    pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(&self, path: P, contents: C) -> io::Result<()> {
        use io::Write;
        let mut file = self.create(path)?;
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
        _dir_builder: &DirBuilder,
        path: P,
    ) -> io::Result<()> {
        todo!(
            "Dir::create_with_dir_builder({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Creates a new symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::unix::fs::symlink`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::os::unix::fs::symlink`]: https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html
    #[cfg(any(
        unix,
        target_os = "wasi",
        target_os = "redox",
        target_os = "vxwords",
        target_os = "fuchsia"
    ))]
    #[inline]
    pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        symlink(src.as_ref(), &self.std_file, dst.as_ref())
    }

    /// Creates a new file symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_file`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::os::windows::fs::symlink_file`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_file.html
    #[cfg(windows)]
    #[inline]
    pub fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        symlink_file(src.as_ref(), &self.std_file, dst.as_ref())
    }

    /// Creates a new directory symlink on a filesystem.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_dir`], but only accesses paths
    /// relative to `self`.
    ///
    /// [`std::os::windows::fs::symlink_dir`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_dir.html
    #[cfg(windows)]
    #[inline]
    pub fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        symlink_dir(src.as_ref(), &self.std_file, dst.as_ref())
    }

    /// Creates a new `UnixListener` bound to the specified socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixListener::bind`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`std::os::unix::net::UnixListener::bind`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.bind
    #[cfg(unix)]
    #[inline]
    pub fn bind_unix_listener<P: AsRef<Path>>(&self, path: P) -> io::Result<UnixListener> {
        todo!(
            "Dir::bind_unix_listener({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Connects to the socket named by path.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::connect`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`std::os::unix::net::UnixStream::connect`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.connect
    #[cfg(unix)]
    #[inline]
    pub fn connect_unix_stream<P: AsRef<Path>>(&self, path: P) -> io::Result<UnixStream> {
        todo!(
            "Dir::connect_unix_stream({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Creates a Unix datagram socket bound to the given path.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::bind`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`std::os::unix::net::UnixDatagram::bind`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.bind
    #[cfg(unix)]
    #[inline]
    pub fn bind_unix_datagram<P: AsRef<Path>>(&self, path: P) -> io::Result<UnixDatagram> {
        todo!(
            "Dir::bind_unix_datagram({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Connects the socket to the specified address.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::connect`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`std::os::unix::net::UnixDatagram::connect`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.connect
    #[cfg(unix)]
    #[inline]
    pub fn connect_unix_datagram<P: AsRef<Path>>(
        &self,
        _unix_datagram: &UnixDatagram,
        path: P,
    ) -> io::Result<()> {
        todo!(
            "Dir::connect_unix_datagram({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Sends data on the socket to the specified address.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::send_to`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`std::os::unix::net::UnixDatagram::send_to`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.send_to
    #[cfg(unix)]
    #[inline]
    pub fn send_to_unix_datagram_addr<P: AsRef<Path>>(
        &self,
        _unix_datagram: &UnixDatagram,
        buf: &[u8],
        path: P,
    ) -> io::Result<usize> {
        todo!(
            "Dir::send_to_unix_datagram_addr({:?}, {:?}, {})",
            self.std_file,
            buf,
            path.as_ref().display()
        )
    }

    /// Creates a new `Dir` instance that shares the same underlying file handle as the existing
    /// `Dir` instance.
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self::from_std_file(self.std_file.try_clone()?))
    }

    /// Returns `true` if the path points at an existing entity.
    ///
    /// This corresponds to [`std::path::Path::exists`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`std::path::Path::exists`]: https://doc.rust-lang.org/std/path/struct.Path.html#method.exists
    #[inline]
    pub fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        self.metadata(path).is_ok()
    }

    /// Returns `true` if the path exists on disk and is pointing at a regular file.
    ///
    /// This corresponds to [`std::path::Path::is_file`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`std::path::Path::is_file`]: https://doc.rust-lang.org/std/path/struct.Path.html#method.is_file
    #[inline]
    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.metadata(path).map(|m| m.is_file()).unwrap_or(false)
    }

    /// Checks if `path` is a directory.
    ///
    /// This is similar to [`std::path::Path::is_dir`] in that it checks if `path` relative to `Dir`
    /// is a directory. This function will traverse symbolic links to query information about the
    /// destination file. In case of broken symbolic links, this will return `false`.
    ///
    /// [`std::path::Path::is_dir`]: https://doc.rust-lang.org/std/path/struct.Path.html#method.is_dir
    #[inline]
    pub fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool {
        self.metadata(path).map(|m| m.is_dir()).unwrap_or(false)
    }
}

#[cfg(unix)]
impl FromRawFd for Dir {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std_file(fs::File::from_raw_fd(fd))
    }
}

#[cfg(windows)]
impl FromRawHandle for Dir {
    #[inline]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self::from_std_file(fs::File::from_raw_handle(handle))
    }
}

#[cfg(unix)]
impl AsRawFd for Dir {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std_file.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawHandle for Dir {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.std_file.as_raw_handle()
    }
}

#[cfg(unix)]
impl IntoRawFd for Dir {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std_file.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawHandle for Dir {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.std_file.into_raw_handle()
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

impl fmt::Debug for Dir {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b = f.debug_struct("Dir");
        fmt_debug_dir(&self.std_file, &mut b);
        b.finish()
    }
}

#[cfg(any(unix, target_os = "fuchsia"))]
fn fmt_debug_dir(fd: &impl AsRawFd, b: &mut fmt::DebugStruct) {
    unsafe fn get_mode(fd: std::os::unix::io::RawFd) -> Option<(bool, bool)> {
        let mode = yanix::fcntl::get_status_flags(fd);
        if mode.is_err() {
            return None;
        }
        match mode.unwrap() & yanix::file::OFlag::ACCMODE {
            yanix::file::OFlag::RDONLY => Some((true, false)),
            yanix::file::OFlag::RDWR => Some((true, true)),
            yanix::file::OFlag::WRONLY => Some((false, true)),
            _ => None,
        }
    }

    let fd = fd.as_raw_fd();
    b.field("fd", &fd);
    if let Some((read, write)) = unsafe { get_mode(fd) } {
        b.field("read", &read).field("write", &write);
    }
}

#[cfg(target_os = "wasi")]
fn fmt_debug_dir(fd: &impl AsRawFd, b: &mut fmt::DebugStruct) {
    unsafe fn get_mode(fd: std::os::wasi::io::RawFd) -> Option<(bool, bool)> {
        let mode = yanix::fcntl::get_status_flags(fd);
        if mode.is_err() {
            return None;
        }
        match mode.unwrap() & yanix::file::OFlag::ACCMODE {
            yanix::file::OFlag::RDONLY => Some((true, false)),
            yanix::file::OFlag::RDWR => Some((true, true)),
            yanix::file::OFlag::WRONLY => Some((false, true)),
            _ => None,
        }
    }

    let fd = fd.as_raw_fd();
    b.field("fd", &fd);
    if let Some((read, write)) = unsafe { get_mode(fd) } {
        b.field("read", &read).field("write", &write);
    }
}

#[cfg(windows)]
fn fmt_debug_dir(fd: &impl AsRawHandle, b: &mut fmt::DebugStruct) {
    b.field("TODO fill in the blanks", &fd.as_raw_handle());
}
