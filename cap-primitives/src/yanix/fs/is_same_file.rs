#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::{fs, io};

/// Determine if `a` and `b` refer to the same inode on the same device.
pub(crate) fn is_same_file(a: &fs::File, b: &fs::File) -> io::Result<bool> {
    let a_metadata = a.metadata()?;
    let b_metadata = b.metadata()?;
    Ok(a_metadata.dev() == b_metadata.dev() && a_metadata.ino() == b_metadata.ino())
}
