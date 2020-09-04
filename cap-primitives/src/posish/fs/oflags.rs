use crate::fs::{FollowSymlinks, OpenOptions};
use posish::fs::OFlags;
use std::io;

/// Return an `OFlags` mask which just includes the bits for reading and
/// writing, like `O_ACCMODE`, but which doesn't include `O_PATH` on systems
/// where that's part of `O_ACCMODE` (eg. musl).
pub(super) fn accmode() -> OFlags {
    #[allow(unused_mut)]
    let mut accmode = OFlags::ACCMODE;

    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "fuchsia",
        target_os = "emscripten"
    ))]
    accmode.remove(OFlags::PATH);

    accmode
}

pub(in super::super) fn compute_oflags(options: &OpenOptions) -> io::Result<OFlags> {
    let mut oflags = OFlags::CLOEXEC;
    oflags |= get_access_mode(options)?;
    oflags |= get_creation_mode(options)?;
    if options.follow == FollowSymlinks::No {
        oflags |= OFlags::NOFOLLOW;
    }
    if options.dir_required {
        oflags |= OFlags::DIRECTORY;
    }
    oflags |=
        OFlags::from_bits(options.ext.custom_flags).expect("unrecognized OFlags") & !accmode();
    Ok(oflags)
}

// `OpenOptions` translation code derived from Rust's
// library/std/src/sys/unix/fs.rs at revision
// 108e90ca78f052c0c1c49c42a22c85620be19712.

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
            (true, false, false) => OFlags::CREATE,
            (false, true, false) => OFlags::TRUNC,
            (true, true, false) => OFlags::CREATE | OFlags::TRUNC,
            (_, _, true) => OFlags::CREATE | OFlags::EXCL,
        },
    )
}
