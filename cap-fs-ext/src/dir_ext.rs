#[cfg(not(windows))]
use cap_primitives::fs::symlink;
use cap_primitives::fs::{open_dir_nofollow, set_times, set_times_nofollow};
#[cfg(windows)]
use cap_primitives::fs::{symlink_dir, symlink_file};
use std::{io, path::Path};
use unsafe_io::AsUnsafeFile;

pub use cap_primitives::fs::SystemTimeSpec;

/// Extension trait for `Dir`.
pub trait DirExt {
    /// Set the last access time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_atime`].
    ///
    /// [`filetime::set_file_atime`]: https://docs.rs/filetime/latest/filetime/fn.set_file_atime.html
    fn set_atime<P: AsRef<Path>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()>;

    /// Set the last modification time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_mtime`].
    ///
    /// [`filetime::set_file_mtime`]: https://docs.rs/filetime/latest/filetime/fn.set_file_mtime.html
    fn set_mtime<P: AsRef<Path>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()>;

    /// Set the last access and modification times for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_times`].
    ///
    /// [`filetime::set_file_times`]: https://docs.rs/filetime/latest/filetime/fn.set_file_times.html
    fn set_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()>;

    /// Set the last access and modification times for a file on a filesystem.
    /// This function does not follow symlink.
    ///
    /// This corresponds to [`filetime::set_symlink_file_times`].
    ///
    /// [`filetime::set_symlink_file_times`]: https://docs.rs/filetime/latest/filetime/fn.set_symlink_file_times.html
    fn set_symlink_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()>;

    /// Creates a new symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::unix::fs::symlink`], except that
    /// it's supported on non-Unix platforms as well, and it's not guaranteed
    /// to be atomic.
    ///
    /// [`std::os::unix::fs::symlink`]: https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()>;

    /// Creates a new file symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_file`], except that
    /// it's supported on non-Windows platforms as well, and it's not guaranteed
    /// to fail if the target is not a file.
    ///
    /// [`std::os::windows::fs::symlink_file`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_file.html
    fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()>;

    /// Creates a new directory symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_dir`], except that
    /// it's supported on non-Windows platforms as well, and it's not guaranteed
    /// to fail if the target is not a directory.
    ///
    /// [`std::os::windows::fs::symlink_dir`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_dir.html
    fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()>;

    /// Similar to `cap_std::fs::Dir::open_dir`, but fails if the path names a
    /// symlink.
    fn open_dir_nofollow<P: AsRef<Path>>(&self, path: P) -> io::Result<Self>
    where
        Self: Sized;

    /// Removes a file or symlink from a filesystem.
    ///
    /// Removal of symlinks has different behavior under Windows - if a symlink points
    /// to a directory, it cannot be removed with the remove_file operation. This
    /// method will remove files and all symlinks.
    ///
    /// On Windows, if a file or symlink does not exist at this path, but an empty directory does
    /// exist, this function will remove the directory.
    fn remove_file_or_symlink<P: AsRef<Path>>(&self, path: P) -> io::Result<()>;
}

/// `fs_utf8` version of `DirExt`.
#[cfg(all(any(feature = "std", feature = "async_std"), feature = "fs_utf8"))]
pub trait DirExtUtf8 {
    /// Set the last access time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_atime`].
    ///
    /// [`filetime::set_file_atime`]: https://docs.rs/filetime/latest/filetime/fn.set_file_atime.html
    fn set_atime<P: AsRef<str>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()>;

    /// Set the last modification time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_mtime`].
    ///
    /// [`filetime::set_file_mtime`]: https://docs.rs/filetime/latest/filetime/fn.set_file_mtime.html
    fn set_mtime<P: AsRef<str>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()>;

    /// Set the last access and modification times for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_times`].
    ///
    /// [`filetime::set_file_times`]: https://docs.rs/filetime/latest/filetime/fn.set_file_times.html
    fn set_times<P: AsRef<str>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()>;

    /// Set the last access and modification times for a file on a filesystem.
    /// This function does not follow symlink.
    ///
    /// This corresponds to [`filetime::set_symlink_file_times`].
    ///
    /// [`filetime::set_symlink_file_times`]: https://docs.rs/filetime/latest/filetime/fn.set_symlink_file_times.html
    fn set_symlink_times<P: AsRef<str>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()>;

    /// Creates a new symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::unix::fs::symlink`], except that
    /// it's supported on non-Unix platforms as well, and it's not guaranteed
    /// to be atomic.
    ///
    /// [`std::os::unix::fs::symlink`]: https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html
    fn symlink<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()>;

    /// Creates a new file symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_file`], except that
    /// it's supported on non-Windows platforms as well, and it's not guaranteed
    /// to fail if the target is not a file.
    ///
    /// [`std::os::windows::fs::symlink_file`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_file.html
    fn symlink_file<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()>;

    /// Creates a new directory symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_dir`], except that
    /// it's supported on non-Windows platforms as well, and it's not guaranteed
    /// to fail if the target is not a directory.
    ///
    /// [`std::os::windows::fs::symlink_dir`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_dir.html
    fn symlink_dir<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()>;

    /// Similar to `cap_std::fs::Dir::open_dir`, but fails if the path names a
    /// symlink.
    fn open_dir_nofollow<P: AsRef<str>>(&self, path: P) -> io::Result<Self>
    where
        Self: Sized;

    /// Removes a file or symlink from a filesystem.
    ///
    /// Removal of symlinks has different behavior under Windows - if a symlink points
    /// to a directory, it cannot be removed with the remove_file operation. This
    /// method will remove files and all symlinks.
    ///
    /// On Windows, ff a file or symlink does not exist at this path, but an empty directory does
    /// exist, this function will remove the directory.
    fn remove_file_or_symlink<P: AsRef<str>>(&self, path: P) -> io::Result<()>;
}

#[cfg(feature = "std")]
impl DirExt for cap_std::fs::Dir {
    #[inline]
    fn set_atime<P: AsRef<Path>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()> {
        set_times(&self.as_file_view(), path.as_ref(), Some(atime), None)
    }

    #[inline]
    fn set_mtime<P: AsRef<Path>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()> {
        set_times(&self.as_file_view(), path.as_ref(), None, Some(mtime))
    }

    #[inline]
    fn set_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        set_times(&self.as_file_view(), path.as_ref(), atime, mtime)
    }

    #[inline]
    fn set_symlink_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        set_times_nofollow(&self.as_file_view(), path.as_ref(), atime, mtime)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        symlink(src.as_ref(), &self.as_file_view(), dst.as_ref())
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        self.symlink(src, dst)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        self.symlink(src, dst)
    }

    #[cfg(windows)]
    #[inline]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        if self.metadata(src.as_ref())?.is_dir() {
            self.symlink_dir(src, dst)
        } else {
            self.symlink_file(src, dst)
        }
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        symlink_file(src.as_ref(), &self.as_file_view(), dst.as_ref())
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        symlink_dir(src.as_ref(), &self.as_file_view(), dst.as_ref())
    }

    #[inline]
    fn open_dir_nofollow<P: AsRef<Path>>(&self, path: P) -> io::Result<Self> {
        match open_dir_nofollow(&self.as_file_view(), path.as_ref()) {
            Ok(file) => Ok(unsafe { Self::from_std_file(file) }),
            Err(e) => Err(e),
        }
    }

    #[cfg(not(windows))]
    #[inline]
    fn remove_file_or_symlink<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        self.remove_file(path.as_ref())
    }

    #[cfg(windows)]
    #[inline]
    fn remove_file_or_symlink<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        self.remove_file(path.as_ref())
            .or_else(|e| self.remove_dir(path.as_ref()).map_err(|_| e))
    }
}

#[cfg(feature = "async_std")]
impl DirExt for cap_async_std::fs::Dir {
    #[inline]
    fn set_atime<P: AsRef<Path>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()> {
        set_times(&self.as_file_view(), path.as_ref(), Some(atime), None)
    }

    #[inline]
    fn set_mtime<P: AsRef<Path>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()> {
        set_times(&self.as_file_view(), path.as_ref(), None, Some(mtime))
    }

    #[inline]
    fn set_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        set_times(&self.as_file_view(), path.as_ref(), atime, mtime)
    }

    #[inline]
    fn set_symlink_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        set_times_nofollow(&self.as_file_view(), path.as_ref(), atime, mtime)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        symlink(src.as_ref(), &self.as_file_view(), dst.as_ref())
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        self.symlink(src.as_ref(), dst.as_ref())
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        self.symlink(src.as_ref(), dst.as_ref())
    }

    #[cfg(windows)]
    #[inline]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        if self.metadata(src.as_ref())?.is_dir() {
            self.symlink_dir(src.as_ref(), dst.as_ref())
        } else {
            self.symlink_file(src.as_ref(), dst.as_ref())
        }
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        symlink_file(src.as_ref(), &self.as_file_view(), dst.as_ref())
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        symlink_dir(src.as_ref(), &self.as_file_view(), dst.as_ref())
    }

    #[inline]
    fn open_dir_nofollow<P: AsRef<Path>>(&self, path: P) -> io::Result<Self> {
        match open_dir_nofollow(&self.as_file_view(), path.as_ref()) {
            Ok(file) => Ok(unsafe { Self::from_std_file(file.into()) }),
            Err(e) => Err(e),
        }
    }

    #[cfg(not(windows))]
    #[inline]
    fn remove_file_or_symlink<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        self.remove_file(path.as_ref())
    }

    #[cfg(windows)]
    #[inline]
    fn remove_file_or_symlink<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        self.remove_file(path.as_ref())
            .or_else(|e| self.remove_dir(path.as_ref()).map_err(|_| e))
    }
}

#[cfg(all(feature = "std", feature = "fs_utf8"))]
impl DirExtUtf8 for cap_std::fs_utf8::Dir {
    #[inline]
    fn set_atime<P: AsRef<str>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()> {
        let path = from_utf8(path)?;
        set_times(&self.as_file_view(), &path, Some(atime), None)
    }

    #[inline]
    fn set_mtime<P: AsRef<str>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()> {
        let path = from_utf8(path)?;
        set_times(&self.as_file_view(), &path, None, Some(mtime))
    }

    #[inline]
    fn set_times<P: AsRef<str>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        let path = from_utf8(path)?;
        set_times(&self.as_file_view(), &path, atime, mtime)
    }

    #[inline]
    fn set_symlink_times<P: AsRef<str>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        let path = from_utf8(path)?;
        set_times_nofollow(&self.as_file_view(), &path, atime, mtime)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink(self, src, dst)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_file<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink(self, src, dst)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_dir<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink(self, src, dst)
    }

    #[cfg(windows)]
    #[inline]
    fn symlink<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()> {
        if self.metadata(src.as_ref())?.is_dir() {
            Self::symlink_dir(self, src, dst)
        } else {
            Self::symlink_file(self, src, dst)
        }
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_file<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink_file(self, src, dst)
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_dir<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink_dir(self, src, dst)
    }

    #[inline]
    fn open_dir_nofollow<P: AsRef<str>>(&self, path: P) -> io::Result<Self> {
        match open_dir_nofollow(&self.as_file_view(), path.as_ref().as_ref()) {
            Ok(file) => Ok(unsafe { Self::from_std_file(file.into()) }),
            Err(e) => Err(e),
        }
    }

    #[cfg(not(windows))]
    #[inline]
    fn remove_file_or_symlink<P: AsRef<str>>(&self, path: P) -> io::Result<()> {
        self.remove_file(path.as_ref())
    }

    #[cfg(windows)]
    #[inline]
    fn remove_file_or_symlink<P: AsRef<str>>(&self, path: P) -> io::Result<()> {
        self.remove_file(path.as_ref())
            .or_else(|e| self.remove_dir(path.as_ref()).map_err(|_| e))
    }
}

#[cfg(all(feature = "async_std", feature = "fs_utf8"))]
impl DirExtUtf8 for cap_async_std::fs_utf8::Dir {
    #[inline]
    fn set_atime<P: AsRef<str>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()> {
        let path = from_utf8(path)?;
        set_times(&self.as_file_view(), &path, Some(atime), None)
    }

    #[inline]
    fn set_mtime<P: AsRef<str>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()> {
        let path = from_utf8(path)?;
        set_times(&self.as_file_view(), &path, None, Some(mtime))
    }

    #[inline]
    fn set_times<P: AsRef<str>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        let path = from_utf8(path)?;
        set_times(&self.as_file_view(), &path, atime, mtime)
    }

    #[inline]
    fn set_symlink_times<P: AsRef<str>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        let path = from_utf8(path)?;
        set_times_nofollow(&self.as_file_view(), &path, atime, mtime)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()> {
        let src = from_utf8(src)?;
        let dst = from_utf8(dst)?;
        symlink(&src, &self.as_file_view(), &dst)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_file<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()> {
        self.symlink(src, dst)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_dir<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()> {
        self.symlink(src, dst)
    }

    #[cfg(windows)]
    #[inline]
    fn symlink<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()> {
        if self.metadata(src.as_ref())?.is_dir() {
            self.symlink_dir(src, dst)
        } else {
            self.symlink_file(src, dst)
        }
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_file<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()> {
        let src = from_utf8(src)?;
        let dst = from_utf8(dst)?;
        symlink_file(&src, &self.as_file_view(), &dst)
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_dir<P: AsRef<str>, Q: AsRef<str>>(&self, src: P, dst: Q) -> io::Result<()> {
        let src = from_utf8(src)?;
        let dst = from_utf8(dst)?;
        symlink_dir(&src, &self.as_file_view(), &dst)
    }

    #[inline]
    fn open_dir_nofollow<P: AsRef<str>>(&self, path: P) -> io::Result<Self> {
        match open_dir_nofollow(&self.as_file_view(), path.as_ref().as_ref()) {
            Ok(file) => Ok(unsafe { Self::from_std_file(file.into()) }),
            Err(e) => Err(e),
        }
    }

    #[cfg(not(windows))]
    #[inline]
    fn remove_file_or_symlink<P: AsRef<str>>(&self, path: P) -> io::Result<()> {
        self.remove_file(path.as_ref())
    }

    #[cfg(windows)]
    #[inline]
    fn remove_file_or_symlink<P: AsRef<str>>(&self, path: P) -> io::Result<()> {
        self.remove_file(path.as_ref())
            .or_else(|e| self.remove_dir(path.as_ref()).map_err(|_| e))
    }
}

#[cfg(all(any(feature = "std", feature = "async_std"), feature = "fs_utf8"))]
fn from_utf8<P: AsRef<str>>(path: P) -> std::io::Result<std::path::PathBuf> {
    #[cfg(not(windows))]
    let path = {
        #[cfg(unix)]
        use std::{ffi::OsString, os::unix::ffi::OsStringExt};
        #[cfg(target_os = "wasi")]
        use std::{ffi::OsString, os::wasi::ffi::OsStringExt};

        let string = arf_strings::str_to_host(path.as_ref())?;
        OsString::from_vec(string.into_bytes())
    };

    #[cfg(windows)]
    let path = arf_strings::str_to_host(path.as_ref())?;

    Ok(path.into())
}
