//! Manual path canonicalization, one component at a time, with manual symlink
//! resolution, in order to enforce sandboxing.

use crate::fs::{open_manually, OpenOptions};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Implement `canonicalize` by breaking up the path into components and resolving
/// each component individually, and resolving symbolic links manually.
pub(crate) fn canonicalize_manually(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    let mut symlink_count = 0;
    let mut canonical_path = PathBuf::new();

    if let Err(e) = open_manually(
        start,
        path,
        OpenOptions::new().read(true),
        &mut symlink_count,
        Some(&mut canonical_path),
    ) {
        if canonical_path.as_os_str().is_empty() {
            return Err(e);
        }
    }

    Ok(canonical_path)
}
