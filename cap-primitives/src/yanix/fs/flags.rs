use crate::fs::{FollowSymlinks, OpenOptions};
use std::io;
use yanix::file::OFlags;

pub(crate) fn compute_oflags(options: &OpenOptions) -> io::Result<OFlags> {
    let mut oflags = OFlags::CLOEXEC;
    oflags |= get_access_mode(options)?;
    oflags |= get_creation_mode(options)?;
    if options.follow == FollowSymlinks::No {
        oflags |= OFlags::NOFOLLOW;
    }
    oflags |= OFlags::from_bits(options.ext.custom_flags).expect("unrecognized OFlag bits")
        & !OFlags::ACCMODE;
    Ok(oflags)
}

// `OpenOptions` translation code derived from Rust's src/libstd/sys/unix/fs.rs
// at revision 7e11379f3b4c376fbb9a6c4d44f3286ccc28d149.

fn get_access_mode(options: &OpenOptions) -> io::Result<OFlags> {
    match (options.read, options.write, options.append) {
        (true, false, false) => Ok(OFlags::RDONLY),
        (false, true, false) => Ok(OFlags::WRONLY),
        (true, true, false) => Ok(OFlags::RDWR),
        (false, _, true) => Ok(OFlags::WRONLY | OFlags::APPEND),
        (true, _, true) => Ok(OFlags::RDWR | OFlags::APPEND),
        (false, false, false) => Err(io::Error::from_raw_os_error(libc::EINVAL)),
    }
}

fn get_creation_mode(options: &OpenOptions) -> io::Result<OFlags> {
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
            (false, false, false) => OFlags::empty(),
            (true, false, false) => OFlags::CREAT,
            (false, true, false) => OFlags::TRUNC,
            (true, true, false) => OFlags::CREAT | OFlags::TRUNC,
            (_, _, true) => OFlags::CREAT | OFlags::EXCL,
        },
    )
}
