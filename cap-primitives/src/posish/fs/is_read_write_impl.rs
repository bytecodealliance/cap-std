use posish::io::is_read_write;
use std::{fs, io};

#[inline]
pub(crate) fn is_read_write_impl(file: &fs::File) -> io::Result<(bool, bool)> {
    is_read_write(file)
}
