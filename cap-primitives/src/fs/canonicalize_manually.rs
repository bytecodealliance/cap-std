//! Manual path canonicalization, one component at a time, with manual symlink
//! resolution, in order to enforce sandboxing.

use crate::fs::{canonicalize_options, open_manually_impl, FollowSymlinks, MaybeOwnedFile};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Implement `canonicalize` by breaking up the path into components and resolving
/// each component individually, and resolving symbolic links manually.
pub(crate) fn canonicalize_manually_and_follow(
    start: &fs::File,
    path: &Path,
) -> io::Result<PathBuf> {
    canonicalize_manually(start, path, FollowSymlinks::Yes)
}

/// The main body of `canonicalize_manually`, which takes an extra `follow`
/// flag allowing the caller to disable following symlinks in the last
/// component.
pub(crate) fn canonicalize_manually(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<PathBuf> {
    let mut symlink_count = 0;
    let mut canonical_path = PathBuf::new();
    let start = MaybeOwnedFile::borrowed(start);

    if let Err(e) = open_manually_impl(
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
