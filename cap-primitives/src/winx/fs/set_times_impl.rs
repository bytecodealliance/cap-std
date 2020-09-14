use crate::fs::{open, FollowSymlinks, OpenOptions, SystemTimeSpec};
use fs_set_times::SetTimes;
use std::{fs, io, os::windows::fs::OpenOptionsExt, path::Path};
use winapi::um::winbase::{FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT};

pub(crate) fn set_times_impl(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
    follow: FollowSymlinks,
) -> io::Result<()> {
    let mut custom_flags = FILE_FLAG_BACKUP_SEMANTICS;
    match follow {
        FollowSymlinks::Yes => (),
        FollowSymlinks::No => custom_flags |= FILE_FLAG_OPEN_REPARSE_POINT,
    };

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
