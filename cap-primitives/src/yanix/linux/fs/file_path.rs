//! `get_path` translation code for Linux derived from Rust's
//! library/std/src/sys/unix/fs.rs at revision
//! 108e90ca78f052c0c1c49c42a22c85620be19712.

use std::{fs, path::PathBuf};

pub(crate) fn file_path(file: &fs::File) -> Option<PathBuf> {
    use std::os::unix::{fs::MetadataExt, io::AsRawFd};

    // Ignore paths that don't start with '/', which are things like
    // `socket:[3556564]` or similar.
    let mut p = PathBuf::from("/proc/self/fd");
    p.push(&file.as_raw_fd().to_string());
    let path = fs::read_link(p).ok().filter(|path| path.starts_with("/"))?;

    // Linux appends the string " (deleted)" when a file is deleted; avoid
    // treating that as the actual name.
    if file.metadata().ok()?.nlink() == 0 {
        return None;
    }

    Some(path)
}
