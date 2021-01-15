use crate::fs::{DirBuilder, File, Metadata, OpenOptions, ReadDir};
#[cfg(target_os = "wasi")]
use async_std::os::wasi::{
    fs::OpenOptionsExt,
    io::{AsRawFd, IntoRawFd},
};
use async_std::path::{Path, PathBuf};
use async_std::task::spawn_blocking;
use async_std::{fs, io};
use cap_primitives::fs::{
    canonicalize, copy, create_dir, hard_link, open, open_ambient_dir, open_dir, read_base_dir,
    read_dir, read_link, remove_dir, remove_dir_all, remove_file, remove_open_dir,
    remove_open_dir_all, rename, set_permissions, stat, DirOptions, FollowSymlinks, Permissions,
};
use cap_primitives::{ambient_authority, AmbientAuthority};
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, FromFd, IntoFd, OwnedFd};
use io_lifetimes::{AsFilelike, FromFilelike};
#[cfg(windows)]
use io_lifetimes::{AsHandle, BorrowedHandle, FromHandle, IntoHandle, OwnedHandle};
use std::fmt;
use unsafe_io::OwnsRaw;
#[cfg(unix)]
use {
    crate::os::unix::net::{UnixDatagram, UnixListener, UnixStream},
    async_std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd},
    cap_primitives::fs::symlink,
};
#[cfg(windows)]
use {
    async_std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle},
    cap_primitives::fs::{symlink_dir, symlink_file},
    unsafe_io::os::windows::{AsRawHandleOrSocket, IntoRawHandleOrSocket, RawHandleOrSocket},
};

/// A reference to an open directory on a filesystem.
///
/// This does not directly correspond to anything in `async_std`, however its
/// methods correspond to the [functions in `async_std::fs`] and the
/// constructor methods for [`async_std::fs::File`].
///
/// Unlike `async_std::fs`, this API's `canonicalize` returns a relative path
/// since absolute paths don't interoperate well with the capability model.
///
/// [functions in `async_std::fs`]: https://docs.rs/async-std/latest/async_std/fs/index.html#functions
#[derive(Clone)]
pub struct Dir {
    std_file: fs::File,
}

impl Dir {
    /// Constructs a new instance of `Self` from the given
    /// `async_std::fs::File`.
    ///
    /// To prevent race conditions on Windows, the file must be opened without
    /// `FILE_SHARE_DELETE`.
    ///
    /// # Ambient Authority
    ///
    /// `async_std::fs::File` is not sandboxed and may access any path that the
    /// host process has access to.
    #[inline]
    pub fn from_std_file(std_file: fs::File, _: AmbientAuthority) -> Self {
        Self { std_file }
    }

    /// Consumes `self` and returns an `async_std::fs::File`.
    #[inline]
    pub fn into_std_file(self) -> fs::File {
        self.std_file
    }

    /// Attempts to open a file in read-only mode.
    ///
    /// This corresponds to [`async_std::fs::File::open`], but only accesses
    /// paths relative to `self`.
    #[inline]
    pub async fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
        self.open_with(path, OpenOptions::new().read(true)).await
    }

    /// Opens a file at `path` with the options specified by `options`.
    ///
    /// This corresponds to [`async_std::fs::OpenOptions::open`].
    ///
    /// Instead of being a method on `OpenOptions`, this is a method on `Dir`,
    /// and it only accesses paths relative to `self`.
    #[inline]
    pub async fn open_with<P: AsRef<Path>>(
        &self,
        path: P,
        options: &OpenOptions,
    ) -> io::Result<File> {
        self._open_with(path.as_ref(), options).await
    }

    #[cfg(not(target_os = "wasi"))]
    async fn _open_with(&self, path: &Path, options: &OpenOptions) -> io::Result<File> {
        let path = path.to_path_buf();
        let clone = self.clone();
        let options = options.clone();
        let file = spawn_blocking(move || {
            open(
                &*clone.as_filelike_view::<std::fs::File>(),
                path.as_ref(),
                &options,
            )
        })
        .await?
        .into();
        Ok(File::from_std(file, ambient_authority()))
    }

    #[cfg(target_os = "wasi")]
    async fn _open_with(
        file: &std::fs::File,
        path: &Path,
        options: &OpenOptions,
    ) -> io::Result<File> {
        let file = options.open_at(&self.std_file, path)?.into();
        Ok(File::from_std(file, ambient_authority()))
    }

    /// Attempts to open a directory.
    #[inline]
    pub async fn open_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        let dir = spawn_blocking(move || {
            open_dir(&clone.as_filelike_view::<std::fs::File>(), path.as_ref())
        })
        .await?
        .into();
        Ok(Self::from_std_file(dir, ambient_authority()))
    }

    /// Creates a new, empty directory at the provided path.
    ///
    /// This corresponds to [`async_std::fs::create_dir`], but only accesses
    /// paths relative to `self`.
    ///
    /// TODO: async: fix this when we fix https://github.com/bytecodealliance/cap-std/issues/51
    #[inline]
    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        self._create_dir_one(path.as_ref(), &DirOptions::new())
    }

    /// Recursively create a directory and all of its parent components if they
    /// are missing.
    ///
    /// This corresponds to [`async_std::fs::create_dir_all`], but only
    /// accesses paths relative to `self`.
    ///
    /// TODO: async: fix this when we fix https://github.com/bytecodealliance/cap-std/issues/51
    #[inline]
    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        self._create_dir_all(path.as_ref(), &DirOptions::new())
    }

    /// Creates the specified directory with the options configured in this
    /// builder.
    ///
    /// This corresponds to [`async_std::fs::DirBuilder::create`].
    ///
    /// TODO: async: fix this when we fix https://github.com/bytecodealliance/cap-std/issues/51
    #[inline]
    pub fn create_dir_with<P: AsRef<Path>>(
        &self,
        path: P,
        dir_builder: &DirBuilder,
    ) -> io::Result<()> {
        let options = dir_builder.options();
        if dir_builder.is_recursive() {
            self._create_dir_all(path.as_ref(), options)
        } else {
            self._create_dir_one(path.as_ref(), options)
        }
    }

    #[inline]
    fn _create_dir_one(&self, path: &Path, dir_options: &DirOptions) -> io::Result<()> {
        create_dir(
            &self.as_filelike_view::<std::fs::File>(),
            path.as_ref(),
            dir_options,
        )
    }

    fn _create_dir_all(&self, path: &Path, dir_options: &DirOptions) -> io::Result<()> {
        if path == Path::new("") {
            return Ok(());
        }

        match self._create_dir_one(path, dir_options) {
            Ok(()) => return Ok(()),
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(_) if self.is_dir_blocking(path) => return Ok(()),
            Err(e) => return Err(e),
        }
        match path.parent() {
            Some(p) => self._create_dir_all(p, dir_options)?,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "failed to create whole tree",
                ))
            }
        }
        match self._create_dir_one(path, dir_options) {
            Ok(()) => Ok(()),
            Err(_) if self.is_dir_blocking(path) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Opens a file in write-only mode.
    ///
    /// This corresponds to [`async_std::fs::File::create`], but only accesses
    /// paths relative to `self`.
    #[inline]
    pub async fn create<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
        self.open_with(
            path,
            OpenOptions::new().write(true).create(true).truncate(true),
        )
        .await
    }

    /// Returns the canonical form of a path with all intermediate components
    /// normalized and symbolic links resolved.
    ///
    /// This corresponds to [`async_std::fs::canonicalize`], but instead of
    /// returning an absolute path, returns a path relative to the
    /// directory represented by `self`.
    #[inline]
    pub async fn canonicalize<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            canonicalize(&clone.as_filelike_view::<std::fs::File>(), path.as_ref())
        })
        .await
        .map(PathBuf::from)
    }

    /// Copies the contents of one file to another. This function will also
    /// copy the permission bits of the original file to the destination
    /// file.
    ///
    /// This corresponds to [`async_std::fs::copy`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub async fn copy<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        from: P,
        to_dir: &Self,
        to: Q,
    ) -> io::Result<u64> {
        let from = from.as_ref().to_path_buf();
        let to = to.as_ref().to_path_buf();
        let from_clone = self.clone();
        let to_clone = to_dir.clone();
        spawn_blocking(move || {
            copy(
                &from_clone.as_filelike_view::<std::fs::File>(),
                from.as_ref(),
                &to_clone.as_filelike_view::<std::fs::File>(),
                to.as_ref(),
            )
        })
        .await
    }

    /// Creates a new hard link on a filesystem.
    ///
    /// This corresponds to [`async_std::fs::hard_link`], but only accesses
    /// paths relative to `self`.
    #[inline]
    pub async fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        src: P,
        dst_dir: &Self,
        dst: Q,
    ) -> io::Result<()> {
        let dst = dst.as_ref().to_path_buf();
        let src = src.as_ref().to_path_buf();
        let src_clone = self.clone();
        let dst_clone = dst_dir.clone();
        spawn_blocking(move || {
            hard_link(
                &src_clone.as_filelike_view::<std::fs::File>(),
                src.as_ref(),
                &dst_clone.as_filelike_view::<std::fs::File>(),
                dst.as_ref(),
            )
        })
        .await
    }

    /// Given a path, query the file system to get information about a file,
    /// directory, etc.
    ///
    /// This corresponds to [`async_std::fs::metadata`], but only accesses
    /// paths relative to `self`.
    #[inline]
    pub async fn metadata<P: AsRef<Path>>(&self, path: P) -> io::Result<Metadata> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            stat(
                &clone.as_filelike_view::<std::fs::File>(),
                path.as_ref(),
                FollowSymlinks::Yes,
            )
        })
        .await
    }

    /// TODO: Remove this once `create_dir` and friends are async.
    #[inline]
    fn metadata_blocking<P: AsRef<Path>>(&self, path: P) -> io::Result<Metadata> {
        let path = path.as_ref().to_path_buf();
        stat(
            &self.as_filelike_view::<std::fs::File>(),
            path.as_ref(),
            FollowSymlinks::Yes,
        )
    }

    /// Returns an iterator over the entries within `self`.
    #[inline]
    pub async fn entries(&self) -> io::Result<ReadDir> {
        let clone = self.clone();
        spawn_blocking(move || read_base_dir(&clone.as_filelike_view::<std::fs::File>()))
            .await
            .map(|inner| ReadDir { inner })
    }

    /// Returns an iterator over the entries within a directory.
    ///
    /// This corresponds to [`async_std::fs::read_dir`], but only accesses
    /// paths relative to `self`.
    #[inline]
    pub async fn read_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<ReadDir> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || read_dir(&clone.as_filelike_view::<std::fs::File>(), path.as_ref()))
            .await
            .map(|inner| ReadDir { inner })
    }

    /// Read the entire contents of a file into a bytes vector.
    ///
    /// This corresponds to [`async_std::fs::read`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub async fn read<P: AsRef<Path>>(&self, path: P) -> io::Result<Vec<u8>> {
        use async_std::prelude::*;
        let mut file = self.open(path).await?;
        let mut bytes = Vec::with_capacity(initial_buffer_size(&file).await);
        file.read_to_end(&mut bytes).await?;
        Ok(bytes)
    }

    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// This corresponds to [`async_std::fs::read_link`], but only accesses
    /// paths relative to `self`.
    #[inline]
    pub async fn read_link<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || read_link(&clone.as_filelike_view::<std::fs::File>(), path.as_ref()))
            .await
            .map(PathBuf::from)
    }

    /// Read the entire contents of a file into a string.
    ///
    /// This corresponds to [`async_std::fs::read_to_string`], but only
    /// accesses paths relative to `self`.
    #[inline]
    pub async fn read_to_string<P: AsRef<Path>>(&self, path: P) -> io::Result<String> {
        use async_std::prelude::*;
        let mut s = String::new();
        self.open(path).await?.read_to_string(&mut s).await?;
        Ok(s)
    }

    /// Removes an empty directory.
    ///
    /// This corresponds to [`async_std::fs::remove_dir`], but only accesses
    /// paths relative to `self`.
    #[inline]
    pub async fn remove_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            remove_dir(&clone.as_filelike_view::<std::fs::File>(), path.as_ref())
        })
        .await
    }

    /// Removes a directory at this path, after removing all its contents. Use
    /// carefully!
    ///
    /// This corresponds to [`async_std::fs::remove_dir_all`], but only
    /// accesses paths relative to `self`.
    #[inline]
    pub async fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            remove_dir_all(&clone.as_filelike_view::<std::fs::File>(), path.as_ref())
        })
        .await
    }

    /// Remove the directory referenced by `self` and consume `self`.
    ///
    /// Note that even though this implementation works in terms of handles
    /// as much as possible, removal is not guaranteed to be atomic with
    /// respect to a concurrent rename of the directory.
    #[inline]
    pub async fn remove_open_dir(self) -> io::Result<()> {
        let file = std::fs::File::from_into_filelike(self.std_file);
        spawn_blocking(move || remove_open_dir(file)).await
    }

    /// Removes the directory referenced by `self`, after removing all its
    /// contents, and consume `self`. Use carefully!
    ///
    /// Note that even though this implementation works in terms of handles
    /// as much as possible, removal is not guaranteed to be atomic with
    /// respect to a concurrent rename of the directory.
    #[inline]
    pub async fn remove_open_dir_all(self) -> io::Result<()> {
        let file = std::fs::File::from_into_filelike(self.std_file);
        spawn_blocking(move || remove_open_dir_all(file)).await
    }

    /// Removes a file from a filesystem.
    ///
    /// This corresponds to [`async_std::fs::remove_file`], but only accesses
    /// paths relative to `self`.
    #[inline]
    pub async fn remove_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            remove_file(&clone.as_filelike_view::<std::fs::File>(), path.as_ref())
        })
        .await
    }

    /// Rename a file or directory to a new name, replacing the original file
    /// if to already exists.
    ///
    /// This corresponds to [`async_std::fs::rename`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub async fn rename<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        from: P,
        to_dir: &Self,
        to: Q,
    ) -> io::Result<()> {
        let from = from.as_ref().to_path_buf();
        let to = to.as_ref().to_path_buf();
        let clone = self.clone();
        let to_clone = to_dir.clone();
        spawn_blocking(move || {
            rename(
                &clone.as_filelike_view::<std::fs::File>(),
                from.as_ref(),
                &to_clone.as_filelike_view::<std::fs::File>(),
                to.as_ref(),
            )
        })
        .await
    }

    /// Changes the permissions found on a file or a directory.
    ///
    /// This corresponds to [`async_std::fs::set_permissions`], but only
    /// accesses paths relative to `self`. Also, on some platforms, this
    /// function may fail if the file or directory cannot be opened for
    /// reading or writing first.
    pub async fn set_permissions<P: AsRef<Path>>(
        &self,
        path: P,
        perm: Permissions,
    ) -> io::Result<()> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            set_permissions(
                &clone.as_filelike_view::<std::fs::File>(),
                path.as_ref(),
                perm,
            )
        })
        .await
    }

    /// Query the metadata about a file without following symlinks.
    ///
    /// This corresponds to [`async_std::fs::symlink_metadata`], but only
    /// accesses paths relative to `self`.
    #[inline]
    pub async fn symlink_metadata<P: AsRef<Path>>(&self, path: P) -> io::Result<Metadata> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            stat(
                &clone.as_filelike_view::<std::fs::File>(),
                path.as_ref(),
                FollowSymlinks::No,
            )
        })
        .await
    }

    /// Write a slice as the entire contents of a file.
    ///
    /// This corresponds to [`async_std::fs::write`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub async fn write<P: AsRef<Path>, C: AsRef<[u8]>>(
        &self,
        path: P,
        contents: C,
    ) -> io::Result<()> {
        use async_std::prelude::*;
        let mut file = self.create(path).await?;
        file.write_all(contents.as_ref()).await
    }

    /// Creates a new symbolic link on a filesystem.
    ///
    /// This corresponds to [`async_std::os::unix::fs::symlink`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`async_std::os::unix::fs::symlink`]: https://docs.rs/async-std/latest/async_std/os/unix/fs/fn.symlink.html
    #[cfg(not(windows))]
    #[inline]
    pub async fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        let src = src.as_ref().to_path_buf();
        let dst = dst.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            symlink(
                src.as_ref(),
                &clone.as_filelike_view::<std::fs::File>(),
                dst.as_ref(),
            )
        })
        .await
    }

    /// Creates a new file symbolic link on a filesystem.
    ///
    /// This corresponds to [`async_std::os::windows::fs::symlink_file`], but
    /// only accesses paths relative to `self`.
    ///
    /// [`async_std::os::windows::fs::symlink_file`]: https://docs.rs/async-std/latest/async_std/os/windows/fs/fn.symlink_file.html
    #[cfg(windows)]
    #[inline]
    pub async fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        let src = src.as_ref().to_path_buf();
        let dst = dst.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            symlink_file(
                src.as_ref(),
                &clone.as_filelike_view::<std::fs::File>(),
                dst.as_ref(),
            )
        })
        .await
    }

    /// Creates a new directory symlink on a filesystem.
    ///
    /// This corresponds to [`async_std::os::windows::fs::symlink_dir`], but
    /// only accesses paths relative to `self`.
    ///
    /// [`async_std::os::windows::fs::symlink_dir`]: https://docs.rs/async-std/latest/async_std/os/windows/fs/fn.symlink_dir.html
    #[cfg(windows)]
    #[inline]
    pub async fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        let src = src.as_ref().to_path_buf();
        let dst = dst.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            symlink_dir(
                src.as_ref(),
                &clone.as_filelike_view::<std::fs::File>(),
                dst.as_ref(),
            )
        })
        .await
    }

    /// Creates a new `UnixListener` bound to the specified socket.
    ///
    /// This corresponds to [`async_std::os::unix::net::UnixListener::bind`],
    /// but only accesses paths relative to `self`.
    ///
    /// XXX: This function is not yet implemented.
    ///
    /// [`async_std::os::unix::net::UnixListener::bind`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixListener.html#method.bind
    #[cfg(unix)]
    #[inline]
    pub async fn bind_unix_listener<P: AsRef<Path>>(&self, path: P) -> io::Result<UnixListener> {
        todo!(
            "Dir::bind_unix_listener({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Connects to the socket named by path.
    ///
    /// This corresponds to [`async_std::os::unix::net::UnixStream::connect`],
    /// but only accesses paths relative to `self`.
    ///
    /// XXX: This function is not yet implemented.
    ///
    /// [`async_std::os::unix::net::UnixStream::connect`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixStream.html#method.connect
    #[cfg(unix)]
    #[inline]
    pub async fn connect_unix_stream<P: AsRef<Path>>(&self, path: P) -> io::Result<UnixStream> {
        todo!(
            "Dir::connect_unix_stream({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Creates a Unix datagram socket bound to the given path.
    ///
    /// This corresponds to [`async_std::os::unix::net::UnixDatagram::bind`],
    /// but only accesses paths relative to `self`.
    ///
    /// XXX: This function is not yet implemented.
    ///
    /// [`async_std::os::unix::net::UnixDatagram::bind`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixDatagram.html#method.bind
    #[cfg(unix)]
    #[inline]
    pub async fn bind_unix_datagram<P: AsRef<Path>>(&self, path: P) -> io::Result<UnixDatagram> {
        todo!(
            "Dir::bind_unix_datagram({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Connects the socket to the specified address.
    ///
    /// This corresponds to
    /// [`async_std::os::unix::net::UnixDatagram::connect`], but only
    /// accesses paths relative to `self`.
    ///
    /// XXX: This function is not yet implemented.
    ///
    /// [`async_std::os::unix::net::UnixDatagram::connect`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixDatagram.html#method.connect
    #[cfg(unix)]
    #[inline]
    pub async fn connect_unix_datagram<P: AsRef<Path>>(
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
    /// This corresponds to
    /// [`async_std::os::unix::net::UnixDatagram::send_to`], but only
    /// accesses paths relative to `self`.
    ///
    /// XXX: This function is not yet implemented.
    ///
    /// [`async_std::os::unix::net::UnixDatagram::send_to`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.UnixDatagram.html#method.send_to
    #[cfg(unix)]
    #[inline]
    pub async fn send_to_unix_datagram_addr<P: AsRef<Path>>(
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

    // async_std doesn't have `try_clone`.

    /// Returns `true` if the path points at an existing entity.
    ///
    /// This corresponds to [`async_std::path::Path::exists`], but only
    /// accesses paths relative to `self`.
    #[inline]
    pub async fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        self.metadata(path).await.is_ok()
    }

    /// Returns `true` if the path exists on disk and is pointing at a regular
    /// file.
    ///
    /// This corresponds to [`async_std::path::Path::is_file`], but only
    /// accesses paths relative to `self`.
    #[inline]
    pub async fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.metadata(path)
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    }

    /// Checks if `path` is a directory.
    ///
    /// This is similar to [`async_std::path::Path::is_dir`] in that it checks
    /// if `path` relative to `Dir` is a directory. This function will traverse
    /// symbolic links to query information about the destination file. In case
    /// of broken symbolic links, this will return `false`.
    #[inline]
    pub async fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool {
        self.metadata(path)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false)
    }

    /// TODO: Remove this once `create_dir` and friends are async.
    #[inline]
    fn is_dir_blocking<P: AsRef<Path>>(&self, path: P) -> bool {
        self.metadata_blocking(path)
            .map(|m| m.is_dir())
            .unwrap_or(false)
    }

    /// Constructs a new instance of `Self` by opening the given path as a
    /// directory using the host process' ambient authority.
    ///
    /// # Ambient Authority
    ///
    /// This function is not sandboxed and may access any path that the host
    /// process has access to.
    #[inline]
    pub async fn open_ambient_dir<P: AsRef<Path>>(
        path: P,
        ambient_authority: AmbientAuthority,
    ) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        spawn_blocking(move || open_ambient_dir(path.as_ref(), ambient_authority))
            .await
            .map(|f| Self::from_std_file(f.into(), ambient_authority))
    }
}

#[cfg(not(windows))]
impl FromRawFd for Dir {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std_file(fs::File::from_raw_fd(fd), ambient_authority())
    }
}

#[cfg(not(windows))]
impl FromFd for Dir {
    #[inline]
    fn from_fd(fd: OwnedFd) -> Self {
        Self::from_std_file(fs::File::from_fd(fd), ambient_authority())
    }
}

#[cfg(windows)]
impl FromRawHandle for Dir {
    /// To prevent race conditions on Windows, the handle must be opened
    /// without `FILE_SHARE_DELETE`.
    #[inline]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self::from_std_file(fs::File::from_raw_handle(handle), ambient_authority())
    }
}

#[cfg(windows)]
impl FromHandle for Dir {
    #[inline]
    fn from_handle(handle: OwnedHandle) -> Self {
        Self::from_std_file(fs::File::from_handle(handle), ambient_authority())
    }
}

#[cfg(not(windows))]
impl AsRawFd for Dir {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std_file.as_raw_fd()
    }
}

#[cfg(not(windows))]
impl AsFd for Dir {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std_file.as_fd()
    }
}

#[cfg(windows)]
impl AsRawHandle for Dir {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.std_file.as_raw_handle()
    }
}

#[cfg(windows)]
impl AsHandle for Dir {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.std_file.as_handle()
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for Dir {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.std_file.as_raw_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl IntoRawFd for Dir {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std_file.into_raw_fd()
    }
}

#[cfg(not(windows))]
impl IntoFd for Dir {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        self.std_file.into_fd()
    }
}

#[cfg(windows)]
impl IntoRawHandle for Dir {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.std_file.into_raw_handle()
    }
}

#[cfg(windows)]
impl IntoHandle for Dir {
    #[inline]
    fn into_handle(self) -> OwnedHandle {
        self.std_file.into_handle()
    }
}

#[cfg(windows)]
impl IntoRawHandleOrSocket for Dir {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.std_file.into_raw_handle_or_socket()
    }
}

// Safety: `Dir` wraps a `fs::File` which owns its handle.
unsafe impl OwnsRaw for Dir {}

/// Indicates how large a buffer to pre-allocate before reading the entire
/// file.
///
/// Derived from the function of the same name in Rust's library/std/src/fs.rs
/// at revision 108e90ca78f052c0c1c49c42a22c85620be19712.
async fn initial_buffer_size(file: &File) -> usize {
    // Allocate one extra byte so the buffer doesn't need to grow before the
    // final `read` call at the end of the file. Don't worry about `usize`
    // overflow because reading will fail regardless in that case.
    file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0)
}

impl fmt::Debug for Dir {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b = f.debug_struct("Dir");
        #[cfg(not(windows))]
        b.field("fd", &self.std_file.as_raw_fd());
        #[cfg(windows)]
        b.field("handle", &self.std_file.as_raw_handle());
        b.finish()
    }
}
