use crate::fs::file_path_by_searching;
#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
use posish::io::ttyname;
#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
use std::ffi::OsString;
use std::{fs, path::PathBuf};

pub(crate) fn file_path_by_ttyname_or_seaching(file: &fs::File) -> Option<PathBuf> {
    // If it happens to be a tty, we can look up its name.
    #[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
    if let Ok(name) = ttyname(file, OsString::new()) {
        return Some(name.into());
    }

    file_path_by_searching(file)
}
