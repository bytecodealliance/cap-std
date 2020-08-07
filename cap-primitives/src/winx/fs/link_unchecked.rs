use super::get_path::concatenate_or_return_absolute;
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
    let old_full_path = concatenate_or_return_absolute(old_start, old_path)?;
    let new_full_path = concatenate_or_return_absolute(new_start, new_path)?;
    fs::hard_link(old_full_path, new_full_path)
}
