use crate::fs::Permissions;
use std::{convert::TryInto, ffi::CString, fs, io, path::Path};
#[cfg(unix)]
use {std::os::unix::ffi::OsStrExt, std::os::unix::fs::PermissionsExt, std::os::unix::io::AsRawFd};

fn cvt_i32(t: i32) -> io::Result<i32> {
    if t == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

/// *Unsandboxed* function similar to `set_permissions`, but which does not
/// perform sandboxing.
///
/// Note that this function does not work reliably on Linux, since `fchmodat`
/// with `AT_SYMLINK_NOFOLLOW` is emulated by libc using `/proc` routines that
/// assume `/proc` is trustworthy.
#[inline]
pub(crate) fn set_permissions_unchecked(
    start: &fs::File,
    path: &Path,
    perm: Permissions,
) -> io::Result<()> {
    cvt_i32(unsafe {
        libc::fchmodat(
            start.as_raw_fd(),
            CString::new(path.as_os_str().as_bytes())?.as_ptr(),
            perm.mode().try_into().unwrap(),
            libc::AT_SYMLINK_NOFOLLOW,
        )
    })?;
    Ok(())
}
