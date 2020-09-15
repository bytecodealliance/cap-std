//! Extension traits for `Dir`

#![deny(missing_docs)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/sunfishcode/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sunfishcode/cap-std/main/media/cap-std.ico"
)]

use cap_primitives::fs::{set_times, FollowSymlinks};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use std::{io, path::Path};

pub use cap_primitives::fs::SystemTimeSpec;

/// Extension trait for `Dir`.
pub trait DirExt {
    /// Set the last access time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_atime`].
    ///
    /// [`filetime::set_file_atime`]: https://docs.rs/filetime/current/filetime/fn.set_file_atime.html
    fn set_atime<P: AsRef<Path>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()>;

    /// Set the last modification time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_mtime`].
    ///
    /// [`filetime::set_file_mtime`]: https://docs.rs/filetime/current/filetime/fn.set_file_mtime.html
    fn set_mtime<P: AsRef<Path>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()>;

    /// Set the last access and modification times for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_times`].
    ///
    /// [`filetime::set_file_times`]: https://docs.rs/filetime/current/filetime/fn.set_file_times.html
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
    /// [`filetime::set_symlink_file_times`]: https://docs.rs/filetime/current/filetime/fn.set_symlink_file_times.html
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
}

#[cfg(feature = "std")]
impl DirExt for cap_std::fs::Dir {
    #[inline]
    fn set_atime<P: AsRef<Path>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            Some(atime),
            None,
            FollowSymlinks::Yes,
        )
    }

    #[inline]
    fn set_mtime<P: AsRef<Path>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            None,
            Some(mtime),
            FollowSymlinks::Yes,
        )
    }

    #[inline]
    fn set_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            atime,
            mtime,
            FollowSymlinks::Yes,
        )
    }

    #[inline]
    fn set_symlink_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            atime,
            mtime,
            FollowSymlinks::No,
        )
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink(self, src, dst)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink(self, src, dst)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink(self, src, dst)
    }

    #[cfg(windows)]
    #[inline]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        if self.metadata(src.as_ref())?.is_dir() {
            Self::symlink_dir(self, src, dst)
        } else {
            Self::symlink_file(self, src, dst)
        }
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink_file(self, src, dst)
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink_dir(self, src, dst)
    }
}

#[cfg(feature = "async_std")]
impl DirExt for cap_async_std::fs::Dir {
    #[inline]
    fn set_atime<P: AsRef<Path>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            Some(atime),
            None,
            FollowSymlinks::Yes,
        )
    }

    #[inline]
    fn set_mtime<P: AsRef<Path>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            None,
            Some(mtime),
            FollowSymlinks::Yes,
        )
    }

    #[inline]
    fn set_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            atime,
            mtime,
            FollowSymlinks::Yes,
        )
    }

    #[inline]
    fn set_symlink_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            atime,
            mtime,
            FollowSymlinks::No,
        )
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink(self, src, dst)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink(self, src, dst)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink(self, src, dst)
    }

    #[cfg(windows)]
    #[inline]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        if self.metadata(src.as_ref())?.is_dir() {
            Self::symlink_dir(self, src, dst)
        } else {
            Self::symlink_file(self, src, dst)
        }
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink_file(self, src, dst)
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink_dir(self, src, dst)
    }
}

#[cfg(all(feature = "std", feature = "fs_utf8"))]
impl DirExt for cap_std::fs_utf8::Dir {
    #[inline]
    fn set_atime<P: AsRef<str>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            Some(atime),
            None,
            FollowSymlinks::Yes,
        )
    }

    #[inline]
    fn set_mtime<P: AsRef<str>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            None,
            Some(mtime),
            FollowSymlinks::Yes,
        )
    }

    #[inline]
    fn set_times<P: AsRef<str>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            atime,
            mtime,
            FollowSymlinks::Yes,
        )
    }

    #[inline]
    fn set_symlink_times<P: AsRef<str>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            atime,
            mtime,
            FollowSymlinks::No,
        )
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
}

#[cfg(all(feature = "async_std", feature = "fs_utf8"))]
impl DirExt for cap_async_std::fs_utf8::Dir {
    #[inline]
    fn set_atime<P: AsRef<str>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            Some(atime),
            None,
            FollowSymlinks::Yes,
        )
    }

    #[inline]
    fn set_mtime<P: AsRef<str>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            None,
            Some(mtime),
            FollowSymlinks::Yes,
        )
    }

    #[inline]
    fn set_times<P: AsRef<str>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            atime,
            mtime,
            FollowSymlinks::Yes,
        )
    }

    #[inline]
    fn set_symlink_times<P: AsRef<str>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        set_times(
            unsafe { &as_file(self) },
            path.as_ref(),
            atime,
            mtime,
            FollowSymlinks::No,
        )
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
}

/// Utility for returning an `async_std::fs::File` as a `std::fs::File`
/// for synchronous operations.
///
/// # Safety
///
/// Callers must avoid using the `async_std::fs::File` while the
/// resulting `std::fs::File` is live, and must ensure that the resulting
/// `std::fs::File` doesn't outlive the `async_std::fs::File`.
#[cfg(not(windows))]
unsafe fn as_file<Fd: AsRawFd>(fd: &Fd) -> std::mem::ManuallyDrop<std::fs::File> {
    std::mem::ManuallyDrop::new(std::fs::File::from_raw_fd(fd.as_raw_fd()))
}

#[cfg(windows)]
unsafe fn as_file<Handle: AsRawHandle>(handle: &Handle) -> std::mem::ManuallyDrop<std::fs::File> {
    std::mem::ManuallyDrop::new(std::fs::File::from_raw_handle(handle.as_raw_handle()))
}
