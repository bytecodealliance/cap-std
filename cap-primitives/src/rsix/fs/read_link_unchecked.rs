use rsix::fs::readlinkat;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// *Unsandboxed* function similar to `read_link`, but which does not perform
/// sandboxing.
pub(crate) fn read_link_unchecked(
    start: &fs::File,
    path: &Path,
    reuse: PathBuf,
) -> io::Result<PathBuf> {
    Ok(readlinkat(start, path, reuse.into()).map(Into::into)?)
}
