use crate::fs::DirOptions;
use posish::fs::{mkdirat, Mode};
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `mkdir`, but which does not perform sandboxing.
pub(crate) fn mkdir_unchecked(
    start: &fs::File,
    path: &Path,
    options: &DirOptions,
) -> io::Result<()> {
    mkdirat(
        start,
        path,
        Mode::from_bits(options.ext.mode as libc::mode_t).unwrap(),
    )
}
