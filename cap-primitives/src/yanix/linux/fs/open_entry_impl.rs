use crate::fs::{open_with_openat2, OpenOptions, open_entry_manually};
use std::{fs, io, ffi::OsStr};

pub(crate) fn open_entry_impl(
    start: &fs::File,
    path: &OsStr,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    let result = open_with_openat2(
        start,
        path.as_ref(),
        options
    );

    match result {
        Ok(file) => Ok(file),
        Err(err) => match err.raw_os_error() {
            Some(libc::ENOSYS) => open_entry_manually(start, path, options),
            _ => Err(err)
        },
    }
}
