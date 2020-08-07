use super::get_path::concatenate_or_return_absolute;
use crate::fs::{FollowSymlinks, Metadata};
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `stat`, but which does not perform sandboxing.
pub(crate) fn stat_unchecked(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    let full_path = concatenate_or_return_absolute(start, path)?;
    match follow {
        FollowSymlinks::Yes => fs::metadata(full_path),
        FollowSymlinks::No => fs::symlink_metadata(full_path),
    }
    .map(Metadata::from_std)
}
