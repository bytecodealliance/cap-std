//! `stat` by resolving the parent directory and calling `fstatat`.

use crate::fs::{
    open_parent, readlink_one, stat_unchecked, FollowSymlinks, MaybeOwnedFile, Metadata,
};
use std::{borrow::Cow, fs, io, path::Path};

/// Implement `stat` by `open`ing up the parent component of the path and then
/// calling `fstatat` on the last component. If it's a symlink, repeat this
/// process.
pub(crate) fn stat_via_parent(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    let mut symlink_count = 0;
    let mut start = MaybeOwnedFile::borrowed(start);
    let mut path = Cow::Borrowed(path);

    loop {
        // Split `path` into parent and basename and open the parent.
        let basename = open_parent(&mut start, &path, &mut symlink_count)?;

        // Do the stat.
        let metadata = stat_unchecked(&start, basename.as_ref(), FollowSymlinks::No)?;

        // If the user didn't want us to follow a symlink in the last component, or we didn't
        // find a symlink, we're done.
        if !metadata.file_type().is_symlink() || follow == FollowSymlinks::No {
            return Ok(metadata);
        }

        // Dereference the symlink and iterate.
        path = Cow::Owned(readlink_one(&start, basename, &mut symlink_count)?);
    }
}
