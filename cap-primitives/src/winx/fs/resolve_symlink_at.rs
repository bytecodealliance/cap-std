use std::{ffi::OsStr, fs, io, path::PathBuf};

const MAX_SYMLINK_EXPANSIONS: u8 = 40;

/// This is a wrapper around `readlinkat` which helps enforce the symlink
/// expansion limit, and which handles the low-level details.
pub(crate) fn resolve_symlink_at(
    base: &fs::File,
    name: &OsStr,
    symlink_count: &mut u8,
) -> io::Result<PathBuf> {
    todo!("resolve_symlink_at")
}
