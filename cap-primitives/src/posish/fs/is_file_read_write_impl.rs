use posish::fs::is_file_read_write;
use std::{fs, io};

#[inline]
pub(crate) fn is_file_read_write_impl(file: &fs::File) -> io::Result<(bool, bool)> {
    Ok(is_file_read_write(file)?)
}
