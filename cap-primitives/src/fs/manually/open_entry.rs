use super::{internal_open, readlink_one};
use crate::fs::{open_unchecked, FollowSymlinks, MaybeOwnedFile, OpenOptions, OpenUncheckedError};
use std::{ffi::OsStr, fs, io};

pub(crate) fn open_entry(
    start: &fs::File,
    path: &OsStr,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    match open_unchecked(
        start,
        path.as_ref(),
        options.clone().follow(FollowSymlinks::No),
    ) {
        Ok(file) => Ok(file),
        Err(OpenUncheckedError::Symlink(_, _)) if options.follow == FollowSymlinks::Yes => {
            let mut symlink_count = 0;
            let destination = readlink_one(start, path, &mut symlink_count)?;
            let maybe = MaybeOwnedFile::borrowed(start);
            internal_open(maybe, &destination, options, &mut symlink_count, None)
                .map(MaybeOwnedFile::unwrap_owned)
        }
        Err(OpenUncheckedError::NotFound(err))
        | Err(OpenUncheckedError::Other(err))
        | Err(OpenUncheckedError::Symlink(err, _)) => Err(err),
    }
}
