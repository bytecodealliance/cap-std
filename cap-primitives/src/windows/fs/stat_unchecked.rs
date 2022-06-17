#[cfg(windows_by_handle)]
use super::get_path::concatenate;
use crate::fs::{FollowSymlinks, Metadata};
use std::path::Path;
use std::{fs, io};
#[cfg(not(windows_by_handle))]
use windows_sys::Win32::Storage::FileSystem::{
    FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT,
};
#[cfg(not(windows_by_handle))]
use {
    crate::fs::{open_unchecked, OpenOptions},
    std::os::windows::fs::OpenOptionsExt,
};

/// *Unsandboxed* function similar to `stat`, but which does not perform
/// sandboxing.
pub(crate) fn stat_unchecked(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    // When we have `windows_by_handle`, we just call `fs::metadata` etc. and it
    // has everything.
    #[cfg(windows_by_handle)]
    {
        let full_path = concatenate(start, path)?;
        match follow {
            FollowSymlinks::Yes => fs::metadata(full_path),
            FollowSymlinks::No => fs::symlink_metadata(full_path),
        }
        .map(Metadata::from_just_metadata)
    }

    // Otherwise, attempt to open the file to get the metadata that way, as
    // that gives us all the info.
    #[cfg(not(windows_by_handle))]
    {
        let mut opts = OpenOptions::new();
        opts.access_mode(0);
        match follow {
            FollowSymlinks::Yes => {
                opts.custom_flags(FILE_FLAG_BACKUP_SEMANTICS);
                opts.follow(FollowSymlinks::Yes);
            }
            FollowSymlinks::No => {
                opts.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS);
                opts.follow(FollowSymlinks::No);
            }
        }
        let file = open_unchecked(start, path, &opts)?;
        Metadata::from_file(&file)
    }
}
