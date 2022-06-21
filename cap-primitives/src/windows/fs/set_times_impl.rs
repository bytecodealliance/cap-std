use crate::fs::{open, OpenOptions, SystemTimeSpec};
use fs_set_times::SetTimes;
use std::os::windows::fs::OpenOptionsExt;
use std::path::Path;
use std::{fs, io};
use windows_sys::Win32::Storage::FileSystem::{
    FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT,
};

#[inline]
pub(crate) fn set_times_impl(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    set_times_inner(start, path, atime, mtime, 0)
}

#[inline]
pub(crate) fn set_times_nofollow_impl(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    set_times_inner(start, path, atime, mtime, FILE_FLAG_OPEN_REPARSE_POINT)
}

fn set_times_inner(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
    custom_flags: u32,
) -> io::Result<()> {
    let custom_flags = custom_flags | FILE_FLAG_BACKUP_SEMANTICS;

    // On Windows, `set_times` requires write permissions.
    open(
        start,
        path,
        OpenOptions::new().write(true).custom_flags(custom_flags),
    )?
    .set_times(
        atime.map(SystemTimeSpec::into_std),
        mtime.map(SystemTimeSpec::into_std),
    )
}
