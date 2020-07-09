use crate::fs::{FollowSymlinks, Metadata};
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `stat`, but which does not perform sandboxing.
pub(crate) fn stat_unchecked(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    todo!("stat_unchecked")
}
