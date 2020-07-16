use std::{fs, io, os::unix::io::AsRawFd, path::Path};
use yanix::file::{mkdirat, Mode};

/// *Unsandboxed* function similar to `mkdir`, but which does not perform sandboxing.
pub(crate) fn mkdir_unchecked(start: &fs::File, path: &Path) -> io::Result<()> {
    unsafe { mkdirat(start.as_raw_fd(), path, Mode::from_bits(0o777).unwrap()) }
}
