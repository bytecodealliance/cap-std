//! This module consists of helper types and functions for dealing
//! with setting the file times.

use crate::fs::{
    open, set_file_times_syscall, set_times_nofollow, to_timespec, FileTimeSpec, FollowSymlinks,
    OpenOptions,
};
use std::{fs, io, path::Path};

pub(crate) fn set_times_impl(
    start: &fs::File,
    path: &Path,
    atime: Option<FileTimeSpec>,
    mtime: Option<FileTimeSpec>,
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
    atime: Option<FileTimeSpec>,
    mtime: Option<FileTimeSpec>,
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
