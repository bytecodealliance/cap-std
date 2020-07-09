use crate::fs::{OpenOptions, OpenUncheckedError};
use std::{fs, path::Path};

/// *Unsandboxed* function similar to `open`, but which does not perform sandboxing.
pub(crate) fn open_unchecked(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> Result<fs::File, OpenUncheckedError> {
    todo!("open_unchecked")
}
