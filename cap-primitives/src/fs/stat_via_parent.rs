//! `stat` by resolving the parent directory and calling `fstatat`.

use crate::fs::{
    open_parent_manually, readlink_one, stat_unchecked, FollowSymlinks, MaybeOwnedFile, Metadata,
};
use std::{borrow::Cow, fs, io, path::Path};

/// Implement `stat` by `open`ing up the parent component of the path and then
/// calling `stat_unchecked` on the last component. If it's a symlink, repeat this
/// process.
pub(crate) fn stat_via_parent(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    let mut symlink_count = 0;
    let mut dir = MaybeOwnedFile::borrowed(start);
    let mut path = Cow::Borrowed(path);

    loop {
        // Split `path` into parent and basename and open the parent.
        let (opened_dir, basename) = open_parent_manually(dir, &path, &mut symlink_count)?;
        dir = opened_dir;

        // Do the stat.
        let metadata = stat_unchecked(&dir, basename.as_ref(), FollowSymlinks::No)?;

        // If the user didn't want us to follow a symlink in the last component, or we didn't
        // find a symlink, we're done.
        if !metadata.file_type().is_symlink() || follow == FollowSymlinks::No {
            return Ok(metadata);
        }

        // Dereference the symlink and iterate.
        path = Cow::Owned(readlink_one(&dir, basename, &mut symlink_count)?);
    }
}
