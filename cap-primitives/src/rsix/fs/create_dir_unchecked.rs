use crate::fs::DirOptions;
use rsix::fs::{mkdirat, Mode, RawMode};
use std::path::Path;
use std::{fs, io};

/// *Unsandboxed* function similar to `create_dir`, but which does not perform
/// sandboxing.
pub(crate) fn create_dir_unchecked(
    start: &fs::File,
    path: &Path,
    options: &DirOptions,
) -> io::Result<()> {
    Ok(mkdirat(
        start,
        path,
        Mode::from_bits(options.ext.mode as RawMode).unwrap(),
    )?)
}
