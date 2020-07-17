use crate::fs::{FollowSymlinks, Metadata, MetadataExt};
use std::{fs, io, os::unix::io::AsRawFd, path::Path};
use yanix::file::{fstatat, AtFlags};

/// *Unsandboxed* function similar to `stat`, but which does not perform sandboxing.
pub(crate) fn stat_unchecked(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    let atflags = match follow {
        FollowSymlinks::Yes => AtFlags::empty(),
        FollowSymlinks::No => AtFlags::SYMLINK_NOFOLLOW,
    };

    unsafe { fstatat(start.as_raw_fd(), path, atflags) }.map(MetadataExt::from_libc)
}
