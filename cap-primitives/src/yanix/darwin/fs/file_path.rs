//! `get_path` translation code for macOS derived from Rust's
//! library/std/src/sys/unix/fs.rs at revision
//! 108e90ca78f052c0c1c49c42a22c85620be19712.

use std::{fs, path::PathBuf};

pub(crate) fn file_path(file: &fs::File) -> Option<PathBuf> {
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
