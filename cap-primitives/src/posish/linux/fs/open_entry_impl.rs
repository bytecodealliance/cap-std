use crate::fs::{manually, open_beneath, OpenOptions};
use std::ffi::OsStr;
use std::{fs, io};

pub(crate) fn open_entry_impl(
    start: &fs::File,
    path: &OsStr,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    let result = open_beneath(start, path.as_ref(), options);

    match result {
        Ok(file) => Ok(file),
        Err(err) => match posish::io::Error::from_io_error(&err) {
            Some(posish::io::Error::NOSYS) => manually::open_entry(start, path, options),
            _ => Err(err),
        },
    }
}
