use super::{get_path::concatenate_or_return_absolute, open_options_to_std};
use crate::fs::{is_dir_options, FollowSymlinks, OpenOptions, OpenUncheckedError};
use std::{
    ffi::OsString,
    fs, io,
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
};
use winapi::shared::winerror;

/// *Unsandboxed* function similar to `open`, but which does not perform sandboxing.
pub(crate) fn open_unchecked(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> Result<fs::File, OpenUncheckedError> {
    let mut full_path =
        concatenate_or_return_absolute(start, path).map_err(OpenUncheckedError::Other)?;

    // If we're expected to open this as a directory, append a trailing separator
    // so that we fail if it's not a directory.
    if is_dir_options(options) {
        let mut wide = full_path.into_os_string().encode_wide().collect::<Vec<_>>();
        wide.push('\\' as u16);
        full_path = PathBuf::from(OsString::from_wide(&wide));
    }

    let (opts, manually_trunc) = open_options_to_std(options);
    match opts.open(full_path) {
        Ok(f) => {
            // Windows doesn't have a way to return errors like `O_NOFOLLOW`,
            // so check for symlinks manualy.
            if options.follow == FollowSymlinks::No
                && f.metadata()
                    .map_err(OpenUncheckedError::Other)?
                    .file_type()
                    .is_symlink()
            {
                return Err(OpenUncheckedError::Symlink(io::Error::new(
                    io::ErrorKind::Other,
                    "symlink encountered",
                )));
            }
            // Windows truncates symlinks into normal files, so truncation
            // may be disabled above; do it manually if needed.
            if manually_trunc {
                // Unwrap is ok because 0 never overflows, and we'll only
                // have `manually_trunc` set when the file is opened for writing.
                f.set_len(0).unwrap();
            }
            Ok(f)
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Err(OpenUncheckedError::NotFound(e)),
        Err(e) => match e.raw_os_error() {
            Some(code) => match code as u32 {
                winerror::ERROR_FILE_NOT_FOUND | winerror::ERROR_PATH_NOT_FOUND => {
                    Err(OpenUncheckedError::NotFound(e))
                }
                _ => Err(OpenUncheckedError::Other(e)),
            },
            None => Err(OpenUncheckedError::Other(e)),
        },
    }
}
