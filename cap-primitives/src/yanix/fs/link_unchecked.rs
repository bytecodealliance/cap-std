use crate::fs::FollowSymlinks;
use std::{fs, io, os::unix::io::AsRawFd, path::Path};
use yanix::file::{linkat, AtFlags};

/// *Unsandboxed* function similar to `link`, but which does not perform sandboxing.
pub(crate) fn link_unchecked(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
    follow: FollowSymlinks,
) -> io::Result<()> {
    let flags = match follow {
        FollowSymlinks::Yes => AtFlags::SYMLINK_FOLLOW,
        FollowSymlinks::No => AtFlags::empty(),
    };

    unsafe {
        linkat(
            old_start.as_raw_fd(),
            old_path,
            new_start.as_raw_fd(),
            new_path,
            flags,
        )
    }
}
