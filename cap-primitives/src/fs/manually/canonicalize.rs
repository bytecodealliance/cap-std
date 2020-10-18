//! Manual path canonicalization, one component at a time, with manual symlink
//! resolution, in order to enforce sandboxing.

use super::internal_open;
use crate::fs::{canonicalize_options, FollowSymlinks, MaybeOwnedFile};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Implement `canonicalize` by breaking up the path into components and resolving
/// each component individually, and resolving symbolic links manually.
pub(crate) fn canonicalize(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    canonicalize_with(start, path, FollowSymlinks::Yes)
}

/// The main body of `canonicalize`, which takes an extra `follow` flag allowing
/// the caller to disable following symlinks in the last component.
pub(crate) fn canonicalize_with(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<PathBuf> {
    let mut symlink_count = 0;
    let mut canonical_path = PathBuf::new();
    let start = MaybeOwnedFile::borrowed(start);

    if let Err(e) = internal_open(
        start,
        path,
        canonicalize_options().follow(follow),
        &mut symlink_count,
        Some(&mut canonical_path),
    ) {
        if canonical_path.as_os_str().is_empty() {
            return Err(e);
        }
    }

    Ok(canonical_path)
}
