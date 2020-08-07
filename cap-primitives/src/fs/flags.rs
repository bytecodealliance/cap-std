use crate::fs::flags_impl;
use std::{fs, io};

/// Return a pair of booleans indicating whether the given file is opened
/// for reading and writing, respectively.
#[inline]
pub fn flags(file: &fs::File) -> io::Result<(bool, bool)> {
    flags_impl(file)
}
