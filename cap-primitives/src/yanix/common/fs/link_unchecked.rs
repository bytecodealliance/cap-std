use crate::fs::FollowSymlinks;
use std::{
    path::Path,
    ffi::OsStr,
    fs, io,
    os::unix::io::AsRawFd,
};
use yanix::file::{linkat, AtFlag};

/// *Unsandboxed* function similar to `link`, but which does not perform sandboxing.
pub(crate) fn link_unchecked(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
    follow: FollowSymlinks,
) -> io::Result<()> {
    // POSIX's `linkat` with an empty path returns `ENOENT`, so use "." instead.
    // TODO: Use `AT_EMPTY_PATH` instead, on platforms that support it?
    let new_path = if new_path.components().next().is_none() {
        OsStr::new(".")
    } else {
        new_path.as_ref()
    };
    let old_path = if old_path.components().next().is_none() {
        OsStr::new(".")
    } else {
        old_path.as_ref()
    };

    let flags = match follow {
        FollowSymlinks::Yes => AtFlag::SYMLINK_FOLLOW,
        FollowSymlinks::No => AtFlag::empty(),
    };

    unsafe { linkat(old_start.as_raw_fd(), old_path, new_start.as_raw_fd(), new_path, flags) }
}
