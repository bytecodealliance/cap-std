//! Capability-oriented filesystem timestamps.
//!
//! This corresponds to [`filetime`], except:
//!  - The `set_*` functions take a `FileTimeSpec`, which may have the special
//!    value `SymbolicNow`.
//!  - Functions which take paths require the process to have either read or write
//!    access to a filesystem object in order to change its time on some platforms.
//!  - Setting a filesystem object's time to `FileTime::zero()` returns not-supported
//!    error on some platforms.
//!
//! [`filetime`]: https://docs.rs/filetime/

#![deny(missing_docs)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/sunfishcode/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sunfishcode/cap-std/main/media/cap-std.ico"
)]

use cap_primitives::fs::set_times;
use cap_std::fs::Dir;
#[cfg(not(windows))]
use std::os::unix::io::{AsRawFd, FromRawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use std::{io, path::Path};

pub use cap_primitives::fs::{FileTime, FileTimeSpec, FollowSymlinks};

/// Set the last access and modification times for a file handle.
///
/// This corresponds to [`filetime::set_file_handle_times`].
///
/// [`filetime::set_file_handle_times`]: https://docs.rs/filetime/current/filetime/fn.set_file_handle_times.html
pub use cap_primitives::fs::set_file_times;

/// An extension for `Dir` which adds methods for setting file timestamps.
pub trait DirExt {
    /// Set the last access time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_atime`].
    ///
    /// [`filetime::set_file_atime`]: https://docs.rs/filetime/current/filetime/fn.set_file_atime.html
    fn set_atime<P: AsRef<Path>>(&self, path: P, atime: FileTimeSpec) -> io::Result<()>;

    /// Set the last modification time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_mtime`].
    ///
    /// [`filetime::set_file_mtime`]: https://docs.rs/filetime/current/filetime/fn.set_file_mtime.html
    fn set_mtime<P: AsRef<Path>>(&self, path: P, mtime: FileTimeSpec) -> io::Result<()>;

    /// Set the last access and modification times for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_times`].
    ///
    /// [`filetime::set_file_times`]: https://docs.rs/filetime/current/filetime/fn.set_file_times.html
    fn set_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<FileTimeSpec>,
        mtime: Option<FileTimeSpec>,
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
        atime: Option<FileTimeSpec>,
        mtime: Option<FileTimeSpec>,
    ) -> io::Result<()>;
}

#[cfg(not(windows))]
unsafe fn as_file(file: &impl AsRawFd) -> std::mem::ManuallyDrop<std::fs::File> {
    std::mem::ManuallyDrop::new(std::fs::File::from_raw_fd(file.as_raw_fd()))
}

#[cfg(windows)]
unsafe fn as_file(file: &impl AsRawHandle) -> std::mem::ManuallyDrop<std::fs::File> {
    std::mem::ManuallyDrop::new(std::fs::File::from_raw_handle(file.as_raw_handle()))
}

impl DirExt for Dir {
    fn set_atime<P: AsRef<Path>>(&self, path: P, atime: FileTimeSpec) -> io::Result<()> {
        unsafe {
            set_times(
                &as_file(self),
                path.as_ref(),
                Some(atime),
                None,
                FollowSymlinks::Yes,
            )
        }
    }

    fn set_mtime<P: AsRef<Path>>(&self, path: P, mtime: FileTimeSpec) -> io::Result<()> {
        unsafe {
            set_times(
                &as_file(self),
                path.as_ref(),
                None,
                Some(mtime),
                FollowSymlinks::Yes,
            )
        }
    }

    fn set_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<FileTimeSpec>,
        mtime: Option<FileTimeSpec>,
    ) -> io::Result<()> {
        unsafe {
            set_times(
                &as_file(self),
                path.as_ref(),
                atime,
                mtime,
                FollowSymlinks::Yes,
            )
        }
    }

    fn set_symlink_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<FileTimeSpec>,
        mtime: Option<FileTimeSpec>,
    ) -> io::Result<()> {
        unsafe {
            set_times(
                &as_file(self),
                path.as_ref(),
                atime,
                mtime,
                FollowSymlinks::No,
            )
        }
    }
}

/// An extension for `File` which adds methods for setting file timestamps.
pub trait FileExt {
    /// Set the last access and modification times for a file handle.
    ///
    /// This corresponds to [`filetime::set_file_handle_times`].
    ///
    /// [`filetime::set_file_handle_times`]: https://docs.rs/filetime/current/filetime/fn.set_file_handle_times.html
    fn set_times(&self, atime: Option<FileTimeSpec>, mtime: Option<FileTimeSpec>)
        -> io::Result<()>;
}

impl FileExt for std::fs::File {
    fn set_times(
        &self,
        atime: Option<FileTimeSpec>,
        mtime: Option<FileTimeSpec>,
    ) -> io::Result<()> {
        unsafe { set_file_times(&as_file(self), atime, mtime) }
    }
}
