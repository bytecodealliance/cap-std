use crate::fs::Permissions;
use rustix::fs::{chmodat, AtFlags, Mode};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{fs, io};

pub(crate) fn set_permissions_impl(
    start: &fs::File,
    path: &Path,
    perm: Permissions,
) -> io::Result<()> {
    if !super::beneath_supported(start) {
        return super::super::super::fs::set_permissions_manually(start, path, perm);
    }

    Ok(chmodat(
        start,
        path,
        Mode::from_raw_mode(perm.mode() as _),
        AtFlags::RESOLVE_BENEATH,
    )?)
}
