use crate::fs::{FollowSymlinks, Metadata, MetadataExt};
use std::{
    path::Path,
    ffi::OsStr,
    fs, io,
    os::unix::io::AsRawFd,
};
use yanix::file::{fstatat, AtFlag};

/// *Unsandboxed* function similar to `stat`, but which does not perform sandboxing.
pub(crate) fn stat_unchecked(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    // POSIX's `fstatat` with an empty path returns `ENOENT`, so use "." instead.
    let path = if path.as_os_str().is_empty() {
        OsStr::new(".")
    } else {
        path.as_ref()
    };

    let atflags = match follow {
        FollowSymlinks::Yes => AtFlag::empty(),
        FollowSymlinks::No => AtFlag::SYMLINK_NOFOLLOW,
    };

    unsafe { fstatat(start.as_raw_fd(), path, atflags) }.map(MetadataExt::from_libc)
}
