//! This defines `set_permissions`, the primary entrypoint to sandboxed
//! filesystem permissions modification.

use crate::fs::{set_permissions_impl, Permissions};
use std::{fs, io, path::Path};

/// Perform a `chmodat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[inline]
pub fn set_permissions(start: &fs::File, path: &Path, perm: Permissions) -> io::Result<()> {
    set_permissions_impl(start, path, perm)
}
