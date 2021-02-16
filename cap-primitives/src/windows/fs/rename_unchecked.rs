use super::get_path::concatenate_or_return_absolute;
use crate::fs::{errors, open, FollowSymlinks, OpenOptions};
use std::{fs, io, os::windows::fs::OpenOptionsExt, path::Path};
use winapi::um::{
    winbase::{FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT},
    winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE},
};

/// *Unsandboxed* function similar to `rename`, but which does not perform sandboxing.
pub(crate) fn rename_unchecked(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    let (old_full_path, enforce_dir) = concatenate_or_return_absolute(old_start, old_path)?;
    let (new_full_path, _enforce_dir) = concatenate_or_return_absolute(new_start, new_path)?;

    // Windows rejects trailing slashes in old_path but not new_path.
    if enforce_dir {
        // Open the file without `FILE_SHARE_DELETE` so others can't rename
        // the file out from underneath us.
        let mut opts = OpenOptions::new();
        opts.access_mode(0)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
            .access_mode(0)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS)
            .follow(FollowSymlinks::No);
        let opened = open(old_start, old_path, &opts)?;
        if !opened.metadata()?.is_dir() {
            return Err(errors::trailing_slash());
        }
        fs::rename(old_full_path, new_full_path)?;
        drop(opened);
        Ok(())
    } else {
        fs::rename(old_full_path, new_full_path)
    }
}
