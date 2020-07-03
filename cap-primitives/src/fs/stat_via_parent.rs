//! `stat` by resolving the parent directory and calling `fstatat`.

use crate::fs::{
    open_manually, open_parent, resolve_symlink_at, stat_unchecked, FollowSymlinks, MaybeOwnedFile,
    Metadata, OpenOptions,
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
        let basename = match open_parent(&mut start, &path, &mut symlink_count)? {
            None => {
                // `None` means the last component was `..` so open the full path and `fstat` it.
                let file = open_manually(
                    start.as_file(),
                    &path,
                    OpenOptions::new().read(true),
                    &mut symlink_count,
                    None,
                )?;

                let result = file.metadata().map(Metadata::from_std);

                // Check that we're still within the containing path.
                #[cfg(debug_assertions)]
                start.descend_to(file);

                return result;
            }
            // Otherwise we have a normal path.
            Some(basename) => basename,
        };

        // Do the stat.
        let metadata = stat_unchecked(start.as_file(), basename, FollowSymlinks::No)?;

        // If the user didn't want us to follow a symlink in the last component, or we didn't
        // find a symlink, we're done.
        if !metadata.file_type().is_symlink() || follow == FollowSymlinks::No {
            return Ok(metadata);
        }

        // Resolve the symlink and iterate.
        path = Cow::Owned(resolve_symlink_at(
            start.as_file(),
            basename.as_ref(),
            &mut symlink_count,
        )?);
    }
}
