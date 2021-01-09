//! This module consists of helper types and functions for dealing
//! with setting the file times.

use crate::fs::{open, OpenOptions, SystemTimeSpec};
use fs_set_times::SetTimes;
use std::{fs, io, path::Path};

pub(crate) fn set_times_impl(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    // Try `futimens` with a normal handle. Normal handles need some kind of
    // access, so first try write.
    match open(start, path, OpenOptions::new().write(true)) {
        Ok(file) => {
            return file.set_times(
                atime.map(SystemTimeSpec::into_std),
                mtime.map(SystemTimeSpec::into_std),
            )
        }
        Err(err) => match err.raw_os_error() {
            Some(libc::EACCES) | Some(libc::EISDIR) => (),
            _ => return Err(err),
        },
    }

    // Next try read.
    match open(start, path, OpenOptions::new().read(true)) {
        Ok(file) => {
            return file.set_times(
                atime.map(SystemTimeSpec::into_std),
                mtime.map(SystemTimeSpec::into_std),
            )
        }
        Err(err) => match err.raw_os_error() {
            Some(libc::EACCES) => (),
            _ => return Err(err),
        },
    }

    // It's not possible to do anything else with generic POSIX. Plain
    // `utimensat` has two options:
    //  - Follow symlinks, which would open up a race in which a concurrent
    //    modification of the symlink could point outside the sandbox and we
    //    wouldn't be able to detect it, or
    //  - Don't follow symlinks, which would modify the timestamp of the
    //    symlink instead of the file we're trying to get to.
    //
    // So neither does what we need.
    Err(io::Error::from_raw_os_error(libc::ENOTSUP))
}
