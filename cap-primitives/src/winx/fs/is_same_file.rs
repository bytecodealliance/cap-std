use std::{fs, io, os::windows::fs::MetadataExt};

/// Determine if `a` and `b` refer to the same inode on the same device.
pub(crate) fn is_same_file(a: &fs::File, b: &fs::File) -> io::Result<bool> {
    let a_metadata = a.metadata()?;
    let b_metadata = b.metadata()?;
    Ok(
        a_metadata.volume_serial_number() == b_metadata.volume_serial_number()
            && a_metadata.file_index() == b_metadata.file_index(),
    )
}
