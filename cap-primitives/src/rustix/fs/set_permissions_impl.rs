use crate::fs::{errors, open, OpenOptions, Permissions};
use rustix::fs::{fchmod, Mode};
use rustix::io::Error;
use std::convert::TryInto;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{fs, io};

/// This sounds like it should be a job for `fchmodat`, however `fchmodat`
/// handles symlinks in an incompatible way. It either follows symlinks
/// without guaranteeing to stay in the sandbox, or with `AT_SYMLINK_NOFOLLOW`
/// it attempts to change the permissions of symlinks themselves. What we'd
/// need is for it to fail if it encounters a symlink, like `O_NOFOLLOW` does.
pub(crate) fn set_permissions_impl(
    start: &fs::File,
    path: &Path,
    perm: Permissions,
) -> io::Result<()> {
    let std_perm = perm.into_std(start)?;

    // Try `fchmod` with a normal handle. Normal handles need some kind of
    // access, so first try read.
    match open(start, path, OpenOptions::new().read(true)) {
        Ok(file) => return set_file_permissions(&file, std_perm),
        Err(err) => match Error::from_io_error(&err) {
            Some(Error::ACCESS) => (),
            _ => return Err(err),
        },
    }

    // Next try write.
    match open(start, path, OpenOptions::new().write(true)) {
        Ok(file) => return set_file_permissions(&file, std_perm),
        Err(err) => match Error::from_io_error(&err) {
            Some(Error::ACCESS) | Some(Error::ISDIR) => (),
            _ => return Err(err),
        },
    }

    // If neither of those worked, we're out of luck.
    Err(Error::NOTSUP.into())
}

pub(crate) fn set_file_permissions(file: &fs::File, perm: fs::Permissions) -> io::Result<()> {
    #[allow(clippy::useless_conversion)]
    let mode =
        Mode::from_bits(perm.mode().try_into().unwrap()).ok_or_else(errors::invalid_flags)?;
    Ok(fchmod(file, mode)?)
}
