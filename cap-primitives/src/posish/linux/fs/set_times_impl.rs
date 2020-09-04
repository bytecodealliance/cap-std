//! This module consists of helper types and functions for dealing
//! with setting the file times specific to Linux.

use super::procfs::set_times_through_proc_self_fd;
use crate::fs::{
    open, set_file_times_syscall, set_times_nofollow, to_timespec, FollowSymlinks, OpenOptions,
    SystemTimeSpec,
};
use std::{fs, io, path::Path};

pub(crate) fn set_times_impl(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
    follow: FollowSymlinks,
) -> io::Result<()> {
    match follow {
        FollowSymlinks::Yes => set_path_times(start, path, atime, mtime),
        FollowSymlinks::No => set_times_nofollow(start, path, atime, mtime),
    }
}

fn set_path_times(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    let times = [to_timespec(atime)?, to_timespec(mtime)?];

    // Try `futimens` with a normal handle. Normal handles need some kind of
    // access, so first try write.
    match open(start, path, OpenOptions::new().write(true)) {
        Ok(file) => return set_file_times_syscall(&file, &times),
        Err(err) => match err.raw_os_error() {
            Some(libc::EACCES) | Some(libc::EISDIR) => (),
            _ => return Err(err),
        },
    }

    // Next try read.
    match open(start, path, OpenOptions::new().read(true)) {
        Ok(file) => return set_file_times_syscall(&file, &times),
        Err(err) => match err.raw_os_error() {
            Some(libc::EACCES) => (),
            _ => return Err(err),
        },
    }

    // If neither of those worked, turn to `/proc`.
    set_times_through_proc_self_fd(start, path, &times)
}
