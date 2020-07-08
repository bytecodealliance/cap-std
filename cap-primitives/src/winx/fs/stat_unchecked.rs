use crate::fs::{FollowSymlinks, Metadata};
use std::{
    path::Path,
    fs, io,
};

/// *Unsandboxed* function similar to `stat`, but which does not perform sandboxing.
pub(crate) fn stat_unchecked(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    unimplemented!("stat_unchecked")
}
