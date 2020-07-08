use crate::fs::{OpenUncheckedError, OpenOptions};
use std::{
    path::Path,
    fs,
};

/// *Unsandboxed* function similar to `open`, but which does not perform sandboxing.
pub(crate) fn open_unchecked(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> Result<fs::File, OpenUncheckedError> {
    unimplemented!("open_unchecked")
}
