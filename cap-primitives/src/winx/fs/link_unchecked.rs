use crate::fs::FollowSymlinks;
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `link`, but which does not perform sandboxing.
pub(crate) fn link_unchecked(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
    follow: FollowSymlinks,
) -> io::Result<()> {
    unimplemented!("link_unchecked")
}
