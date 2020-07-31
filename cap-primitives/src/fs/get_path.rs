//! `get_path` translation code for Linux and macOS derived from Rust's
//! src/libstd/sys/unix/fs.rs at revision
//! 7e11379f3b4c376fbb9a6c4d44f3286ccc28d149.

use std::{fs, path::PathBuf};

#[cfg(target_os = "linux")]
pub(crate) fn get_path(file: &fs::File) -> Option<PathBuf> {
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

#[cfg(target_os = "macos")]
pub(crate) fn get_path(file: &fs::File) -> Option<PathBuf> {
    use std::os::unix::{ffi::OsStringExt, io::AsRawFd};

    // The use of PATH_MAX is generally not encouraged, but it
    // is inevitable in this case because macOS defines `fcntl` with
    // `F_GETPATH` in terms of `MAXPATHLEN`, and there are no
    // alternatives. If a better method is invented, it should be used
    // instead.
    let mut buf = vec![0; libc::PATH_MAX as usize];
    let n = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETPATH, buf.as_ptr()) };
    if n == -1 {
        return None;
    }
    let l = buf.iter().position(|&c| c == 0).unwrap();
    buf.truncate(l as usize);
    buf.shrink_to_fit();
    Some(PathBuf::from(std::ffi::OsString::from_vec(buf)))
}

#[cfg(target_os = "windows")]
pub(crate) fn get_path(file: &fs::File) -> Option<PathBuf> {
    crate::winx::fs::get_path_impl(file).ok()
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub(crate) fn get_path(_file: &fs::File) -> Option<PathBuf> {
    None
}
