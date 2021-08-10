//! This module consists of helper types and functions for dealing
//! with setting the file times specific to Linux.

use super::procfs::set_times_through_proc_self_fd;
use crate::fs::{open, OpenOptions, SystemTimeSpec};
use fs_set_times::SetTimes;
use std::path::Path;
use std::{fs, io};

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
        Err(err) => match posish::io::Error::from_io_error(&err) {
            Some(posish::io::Error::ACCES) | Some(posish::io::Error::ISDIR) => (),
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
        Err(err) => match posish::io::Error::from_io_error(&err) {
            Some(posish::io::Error::ACCES) => (),
            _ => return Err(err),
        },
    }

    // If neither of those worked, turn to `/proc`.
    set_times_through_proc_self_fd(start, path, atime, mtime)
}
