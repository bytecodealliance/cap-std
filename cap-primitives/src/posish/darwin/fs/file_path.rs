//! `get_path` translation code for macOS derived from Rust's
//! library/std/src/sys/unix/fs.rs at revision
//! 108e90ca78f052c0c1c49c42a22c85620be19712.

use crate::posish::fs::file_path_by_ttyname_or_seaching;
use posish::fs::getpath;
use std::fs;
use std::path::PathBuf;

pub(crate) fn file_path(file: &fs::File) -> Option<PathBuf> {
    if let Ok(path) = getpath(file) {
        return Some(path);
    }

    file_path_by_ttyname_or_seaching(file)
}
