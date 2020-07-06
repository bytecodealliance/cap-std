use crate::{
    fs::{DirBuilder, File, Metadata, OpenOptions, Permissions, ReadDir},
    os::unix::net::{UnixDatagram, UnixListener, UnixStream},
};
use async_std::{
    fs, io,
    os::unix::{
        fs::OpenOptionsExt,
        io::{AsRawFd, FromRawFd, IntoRawFd},
    },
};
use cap_primitives::fs::{canonicalize, link, mkdir, open, stat, unlink, FollowSymlinks};
use std::{
    fmt,
    mem::ManuallyDrop,
    path::{Path, PathBuf},
};
use yanix::file::OFlag;

pub(crate) struct Dir {
    std_file: fs::File,
}

impl Dir {
    #[inline]
    pub(crate) fn from_std_file(std_file: fs::File) -> Self {
        Self { std_file }
    }

    #[inline]
    pub(crate) fn into_std_file(self) -> fs::File {
        self.std_file
    }

    #[inline]
    pub(crate) fn as_raw_fd(&self) -> i32 {
        self.std_file.as_raw_fd()
    }

    #[inline]
    pub(crate) fn into_raw_fd(self) -> i32 {
        self.std_file.into_raw_fd()
    }

    pub(crate) fn open_file_with(&self, path: &Path, options: &OpenOptions) -> io::Result<File> {
        let file = unsafe { self.as_sync_file() };
        open(&file, path, options).map(|file| unsafe { File::from_raw_fd(file.into_raw_fd()) })
    }

    pub(crate) fn open_dir(&self, path: &Path) -> io::Result<crate::fs::Dir> {
        self.open_file_with(
            path,
            OpenOptions::new()
                .read(true)
                .custom_flags(OFlag::DIRECTORY.bits()),
        )
        .map(|file| crate::fs::Dir::from_std_file(file.std))
    }

    pub(crate) fn create_dir(&self, path: &Path) -> io::Result<()> {
        let file = unsafe { self.as_sync_file() };
        mkdir(&file, path)
    }

    pub(crate) fn canonicalize(&self, path: &Path) -> io::Result<PathBuf> {
        use std::os::unix::io::FromRawFd;
        let file =
            ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(self.std_file.as_raw_fd()) });
        canonicalize(&file, path)
    }

    pub(crate) fn hard_link(&self, src: &Path, dst_dir: &Dir, dst: &Path) -> io::Result<()> {
        let src_file = unsafe { self.as_sync_file() };
        let dst_file = unsafe { dst_dir.as_sync_file() };
        link(&src_file, src, &dst_file, dst)
    }

    pub(crate) fn metadata(&self, path: &Path) -> io::Result<Metadata> {
        let file = unsafe { self.as_sync_file() };
        stat(&file, path, FollowSymlinks::Yes)
    }

    pub(crate) fn read_dir(&self, path: &Path) -> io::Result<ReadDir> {
        unimplemented!("Dir::read_dir({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn read_link(&self, path: &Path) -> io::Result<PathBuf> {
        unimplemented!("Dir::read_link({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn remove_dir(&self, path: &Path) -> io::Result<()> {
        unimplemented!("Dir::remove_dir({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn remove_dir_all(&self, path: &Path) -> io::Result<()> {
        unimplemented!(
            "Dir::remove_dir_all({:?}, {})",
            self.std_file,
            path.display()
        )
    }

    pub(crate) fn remove_file(&self, path: &Path) -> io::Result<()> {
        let file = unsafe { self.as_sync_file() };
        unlink(&file, path)
    }

    pub(crate) fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        unimplemented!(
            "Dir::rename({:?}, {}, {})",
            self.std_file,
            from.display(),
            to.display()
        )
    }

    pub(crate) fn set_permissions(&self, path: &Path, perm: Permissions) -> io::Result<()> {
        unimplemented!(
            "Dir::set_permissions({:?}, {}, {:?})",
            self.std_file,
            path.display(),
            perm
        )
    }

    pub(crate) fn symlink_metadata(&self, path: &Path) -> io::Result<Metadata> {
        use std::os::unix::io::FromRawFd;
        let file =
            ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(self.std_file.as_raw_fd()) });
        stat(&file, path, FollowSymlinks::No)
    }

    pub(crate) fn create_with_dir_builder(
        &self,
        dir_builder: &DirBuilder,
        path: &Path,
    ) -> io::Result<()> {
        unimplemented!(
            "Dir::create_with_dir_builder({:?}, {})",
            self.std_file,
            path.display()
        )
    }

    pub(crate) fn symlink(&self, src: &Path, dst: &Path) -> io::Result<()> {
        unimplemented!(
            "Dir::symlink({:?}, {}, {})",
            self.std_file,
            src.display(),
            dst.display()
        )
    }

    pub(crate) fn bind_unix_listener(&self, path: &Path) -> io::Result<UnixListener> {
        unimplemented!(
            "Dir::bind_unix_listener({:?}, {})",
            self.std_file,
            path.display()
        )
    }

    pub(crate) fn connect_unix_stream(&self, path: &Path) -> io::Result<UnixStream> {
        unimplemented!(
            "Dir::connect_unix_stream({:?}, {})",
            self.std_file,
            path.display()
        )
    }

    pub(crate) fn bind_unix_datagram(&self, path: &Path) -> io::Result<UnixDatagram> {
        unimplemented!(
            "Dir::bind_unix_datagram({:?}, {})",
            self.std_file,
            path.display()
        )
    }

    pub(crate) fn connect_unix_datagram(
        &self,
        unix_datagram: &UnixDatagram,
        path: &Path,
    ) -> io::Result<()> {
        unimplemented!(
            "Dir::connect_unix_datagram({:?}, {})",
            self.std_file,
            path.display()
        )
    }

    pub(crate) fn send_to_unix_datagram_addr(
        &self,
        unix_datagram: &UnixDatagram,
        buf: &[u8],
        path: &Path,
    ) -> io::Result<usize> {
        unimplemented!(
            "Dir::send_to_unix_datagram_addr({:?}, {:?}, {})",
            self.std_file,
            buf,
            path.display()
        )
    }

    // async_std doesn't have `try_clone`.

    /// Utility for returning `self`'s `async_std::fs::File` member as a
    /// `std::fs::File` for synchronous operations.
    ///
    /// # Safety
    ///
    /// Callers must avoid using `self`'s `async_std::fs::File` while the
    /// resulting `std::fs::File` is live, and must ensure that the resulting
    /// `std::fs::File` outlives `self`'s `async_std::fs::File`.
    #[inline]
    unsafe fn as_sync_file(&self) -> ManuallyDrop<std::fs::File> {
        ManuallyDrop::new(std::fs::File::from_raw_fd(self.std_file.as_raw_fd()))
    }
}

impl fmt::Debug for Dir {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b = f.debug_struct("Dir");

        if cfg!(any(unix, target_os = "fuchsia")) {
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

            let fd = self.std_file.as_raw_fd();
            b.field("fd", &fd);
            if let Some((read, write)) = unsafe { get_mode(fd) } {
                b.field("read", &read).field("write", &write);
            }
        }

        b.finish()
    }
}
