use crate::{
    fs::{DirBuilder, File, Metadata, OpenOptions, Permissions, ReadDir},
    os::unix::net::{UnixDatagram, UnixListener, UnixStream},
};
use std::{
    fs, io,
    os::unix::io::{AsRawFd, FromRawFd, IntoRawFd},
    path::{Path, PathBuf},
};
use yanix::file::{openat, Mode, OFlag};

pub(crate) struct Dir {
    std_file: fs::File,
}

impl Dir {
    #[inline]
    pub(crate) fn from_std_file(std_file: fs::File) -> Self {
        Self { std_file }
    }

    #[inline]
    pub(crate) fn as_raw_fd(&self) -> i32 {
        self.std_file.as_raw_fd()
    }

    #[inline]
    pub(crate) fn into_raw_fd(self) -> i32 {
        self.std_file.into_raw_fd()
    }

    pub(crate) fn open_file(&self, path: &Path) -> io::Result<File> {
        unsafe {
            let fd = openat(
                self.std_file.as_raw_fd(),
                path,
                OFlag::RDONLY,
                Mode::empty(),
            )?;
            Ok(File::from_raw_fd(fd))
        }
    }

    pub(crate) fn open_file_with(&self, path: &Path, options: &OpenOptions) -> io::Result<File> {
        unimplemented!(
            "Dir::open_file_with({:?}, {}, {:?})",
            self.std_file,
            path.display(),
            options
        );
    }

    pub(crate) fn open_dir(&self, path: &Path) -> io::Result<crate::fs::Dir> {
        unsafe {
            let fd = openat(
                self.std_file.as_raw_fd(),
                path,
                OFlag::RDONLY | OFlag::DIRECTORY,
                Mode::empty(),
            )?;
            Ok(crate::fs::Dir::from_raw_fd(fd))
        }
    }

    pub(crate) fn create_dir(&self, path: &Path) -> io::Result<()> {
        unimplemented!("Dir::create_dir({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        unimplemented!(
            "Dir::create_dir_all({:?}, {})",
            self.std_file,
            path.display()
        )
    }

    pub(crate) fn create_file(&self, path: &Path) -> io::Result<File> {
        unsafe {
            let fd = openat(
                self.std_file.as_raw_fd(),
                path,
                OFlag::WRONLY | OFlag::CREAT | OFlag::TRUNC,
                Mode::from_bits(0o666).unwrap(),
            )?;
            Ok(File::from_raw_fd(fd))
        }
    }

    pub(crate) fn canonicalize(&self, path: &Path) -> io::Result<PathBuf> {
        // TODO Implement canoncalize without returning an absolute path.
        unimplemented!("Dir::canonicalize({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn copy(&self, from: &Path, to: &Path) -> io::Result<u64> {
        unimplemented!(
            "Dir::copy({:?}, {}, {})",
            self.std_file,
            from.display(),
            to.display()
        )
    }

    pub(crate) fn hard_link(&self, src: &Path, dst: &Path) -> io::Result<()> {
        unimplemented!(
            "Dir::hard_link({:?}, {}, {})",
            self.std_file,
            src.display(),
            dst.display()
        )
    }

    pub(crate) fn metadata(&self, path: &Path) -> io::Result<Metadata> {
        unimplemented!("Dir::metadata({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn read_dir(&self, path: &Path) -> io::Result<ReadDir> {
        unimplemented!("Dir::read_dir({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn read_link(&self, path: &Path) -> io::Result<PathBuf> {
        unimplemented!("Dir::read_link({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn read_to_string(&self, path: &Path) -> io::Result<String> {
        unimplemented!(
            "Dir::read_to_string({:?}, {})",
            self.std_file,
            path.display()
        )
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
        unimplemented!("Dir::remove_file({:?}, {})", self.std_file, path.display())
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
        unimplemented!(
            "Dir::symlink_metadata({:?}, {:?})",
            self.std_file,
            path.display()
        )
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

    pub(crate) fn try_clone(&self) -> io::Result<Dir> {
        Ok(Self::from_std_file(self.std_file.try_clone()?))
    }
}
