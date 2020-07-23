use super::{get_path::concatenate_or_return_absolute, open_options_to_std};
use crate::fs::{OpenOptions, OpenUncheckedError};
use std::{fs, io, path::Path};
use winapi::shared::winerror;

/// *Unsandboxed* function similar to `open`, but which does not perform sandboxing.
pub(crate) fn open_unchecked(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> Result<fs::File, OpenUncheckedError> {
    let full_path =
        concatenate_or_return_absolute(start, path).map_err(OpenUncheckedError::Other)?;
    let opts = open_options_to_std(options);
    match opts.open(full_path) {
        Ok(f) => Ok(f),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Err(OpenUncheckedError::NotFound(e)),
        Err(e) => match e.raw_os_error() {
            Some(code) => match code as u32 {
                winerror::ERROR_FILE_NOT_FOUND | winerror::ERROR_PATH_NOT_FOUND => {
                    Err(OpenUncheckedError::NotFound(e))
                }
                winerror::ERROR_REPARSE | winerror::ERROR_REPARSE_OBJECT => {
                    Err(OpenUncheckedError::Symlink(e))
                }
                _ => Err(OpenUncheckedError::Other(e)),
            },
            None => Err(OpenUncheckedError::Other(e)),
        },
    }
}
