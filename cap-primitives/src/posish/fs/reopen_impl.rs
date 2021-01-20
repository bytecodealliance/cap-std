use crate::{
    fs::{open_unchecked, OpenOptions},
    posish::fs::file_path,
};
use posish::fs::cwd;
use std::{fs, io};

/// Implementation of `reopen`.
pub(crate) fn reopen_impl(file: &fs::File, options: &OpenOptions) -> io::Result<fs::File> {
    if let Some(path) = file_path(file) {
        Ok(open_unchecked(&cwd(), &path, options)?)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Couldn't reopen file"))
    }
}
