use crate::fs::{open, FollowSymlinks, OpenOptions, SystemTimeSpec};
use fs_set_times::SetTimes;
use std::{fs, io, os::windows::fs::OpenOptionsExt, path::Path};
use winapi::{
    shared::winerror::ERROR_NOT_SUPPORTED,
    um::winbase::{FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT},
};

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

    match open(
        start,
        path,
        OpenOptions::new().write(true).custom_flags(custom_flags),
    ) {
        Ok(file) => {
            return file.set_times(
                atime.map(SystemTimeSpec::into_std),
                mtime.map(SystemTimeSpec::into_std),
            )
        }
        Err(err) => match err.kind() {
            io::ErrorKind::PermissionDenied => (),
            _ => return Err(err),
        },
    }

    match open(
        start,
        path,
        OpenOptions::new().read(true).custom_flags(custom_flags),
    ) {
        Ok(file) => {
            return file.set_times(
                atime.map(SystemTimeSpec::into_std),
                mtime.map(SystemTimeSpec::into_std),
            )
        }
        Err(err) => match err.kind() {
            io::ErrorKind::PermissionDenied => (),
            _ => return Err(err),
        },
    }

    Err(io::Error::from_raw_os_error(ERROR_NOT_SUPPORTED as i32))
}
