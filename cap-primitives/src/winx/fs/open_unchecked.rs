use crate::fs::OpenOptions;
use std::{
    path::Path,
    fs, io,
};

/// *Unsandboxed* function similar to `open`, but which does not perform sandboxing.
pub(crate) fn open_unchecked(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    unimplemented!("open_unchecked")
}
