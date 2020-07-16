use crate::fs::{FollowSymlinks, OpenOptions};
use std::io;
use yanix::file::OFlag;

pub(crate) fn compute_oflags(options: &OpenOptions) -> io::Result<OFlag> {
    // TODO: Add `CLOEXEC` when yanix is updated.
    let mut oflags = OFlag::empty();
    oflags |= get_access_mode(options)?;
    oflags |= get_creation_mode(options)?;
    if options.follow == FollowSymlinks::No {
        oflags |= OFlag::NOFOLLOW;
    }
    oflags |= OFlag::from_bits(options.ext.custom_flags).expect("unrecognized OFlag bits")
        & !OFlag::ACCMODE;
    Ok(oflags)
}

// `OpenOptions` translation code derived from Rust's src/libstd/sys/unix/fs.rs
// at revision 7e11379f3b4c376fbb9a6c4d44f3286ccc28d149.

fn get_access_mode(options: &OpenOptions) -> io::Result<OFlag> {
    match (options.read, options.write, options.append) {
        (true, false, false) => Ok(OFlag::RDONLY),
        (false, true, false) => Ok(OFlag::WRONLY),
        (true, true, false) => Ok(OFlag::RDWR),
        (false, _, true) => Ok(OFlag::WRONLY | OFlag::APPEND),
        (true, _, true) => Ok(OFlag::RDWR | OFlag::APPEND),
        (false, false, false) => Err(io::Error::from_raw_os_error(libc::EINVAL)),
    }
}

fn get_creation_mode(options: &OpenOptions) -> io::Result<OFlag> {
    match (options.write, options.append) {
        (true, false) => {}
        (false, false) => {
            if options.truncate || options.create || options.create_new {
                return Err(io::Error::from_raw_os_error(libc::EINVAL));
            }
        }
        (_, true) => {
            if options.truncate && !options.create_new {
                return Err(io::Error::from_raw_os_error(libc::EINVAL));
            }
        }
    }

    Ok(
        match (options.create, options.truncate, options.create_new) {
            (false, false, false) => OFlag::empty(),
            (true, false, false) => OFlag::CREAT,
            (false, true, false) => OFlag::TRUNC,
            (true, true, false) => OFlag::CREAT | OFlag::TRUNC,
            (_, _, true) => OFlag::CREAT | OFlag::EXCL,
        },
    )
}
