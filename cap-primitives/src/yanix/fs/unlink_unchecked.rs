use std::{fs, io, os::unix::io::AsRawFd, path::Path};
use yanix::file::{unlinkat, AtFlags};

/// *Unsandboxed* function similar to `unlink`, but which does not perform sandboxing.
pub(crate) fn unlink_unchecked(start: &fs::File, path: &Path) -> io::Result<()> {
    unsafe { unlinkat(start.as_raw_fd(), path, AtFlags::empty()) }
}
