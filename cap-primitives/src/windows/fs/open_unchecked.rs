use super::{get_path::concatenate_or_return_absolute, open_options_to_std};
use crate::fs::{errors, FollowSymlinks, OpenOptions, OpenUncheckedError, SymlinkKind};
use std::{fs, io, os::windows::fs::MetadataExt, path::Path};
use winapi::{
    shared::winerror,
    um::{winbase, winnt::FILE_ATTRIBUTE_DIRECTORY},
};

/// *Unsandboxed* function similar to `open`, but which does not perform sandboxing.
pub(crate) fn open_unchecked(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> Result<fs::File, OpenUncheckedError> {
    let full_path =
        concatenate_or_return_absolute(start, path).map_err(OpenUncheckedError::Other)?;
    let (opts, manually_trunc) = open_options_to_std(options);
    match opts.open(full_path) {
        Ok(f) => {
            let enforce_dir = options.dir_required;
            let enforce_nofollow = options.follow == FollowSymlinks::No
                && (options.ext.custom_flags & winbase::FILE_FLAG_OPEN_REPARSE_POINT) == 0;

            if enforce_dir || enforce_nofollow {
                let metadata = f.metadata().map_err(OpenUncheckedError::Other)?;

                if enforce_dir {
                    // Require a directory. It may seem possible to eliminate
                    // this `metadata()` call by appending a slash to the path
                    // before opening it so that the OS requires a directory
                    // for us, however on Windows in some circumstances this
                    // leads to "The filename, directory name, or volume label
                    // syntax is incorrect." errors.
                    //
                    // We check `file_attributes()` instead of using `is_dir()`
                    // since the latter returns false if we're looking at a
                    // directory symlink.
                    if metadata.file_attributes() & FILE_ATTRIBUTE_DIRECTORY == 0 {
                        return Err(OpenUncheckedError::Other(errors::is_not_directory()));
                    }
                }

                if enforce_nofollow {
                    // Windows doesn't have a way to return errors like
                    // `O_NOFOLLOW`, so if we're not following symlinks and
                    // we're not using `FILE_FLAG_OPEN_REPARSE_POINT` manually
                    // to open a symlink itself, check for symlinks and report
                    // them as a distinct error.
                    if metadata.file_type().is_symlink() {
                        return Err(OpenUncheckedError::Symlink(
                            io::Error::from_raw_os_error(winerror::ERROR_STOPPED_ON_SYMLINK as i32),
                            if metadata.file_attributes() & FILE_ATTRIBUTE_DIRECTORY
                                == FILE_ATTRIBUTE_DIRECTORY
                            {
                                SymlinkKind::Dir
                            } else {
                                SymlinkKind::File
                            },
                        ));
                    }
                }
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
