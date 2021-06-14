use crate::fs::{target_o_path, FollowSymlinks, OpenOptions};
use posish::{fs::OFlags, io::Errno};
use std::io;

pub(in super::super) fn compute_oflags(options: &OpenOptions) -> io::Result<OFlags> {
    let mut oflags = OFlags::CLOEXEC;
    oflags |= get_access_mode(options)?;
    oflags |= get_creation_mode(options)?;
    if options.follow == FollowSymlinks::No {
        oflags |= OFlags::NOFOLLOW;
    }
    if options.dir_required {
        oflags |= OFlags::DIRECTORY;

        // If the target has `O_PATH`, we don't need to read the directory
        // entries, and we're not requesting write access (which need to
        // fail on a directory), use it.
        if !options.readdir_required && !options.write && !options.append {
            oflags |= target_o_path();
        }
    }
    // Use `RWMODE` here instead of `ACCMODE` so that we preserve the `O_PATH` flag.
    oflags |= OFlags::from_bits(options.ext.custom_flags as _).expect("unrecognized OFlags")
        & !OFlags::RWMODE;
    Ok(oflags)
}

// `OpenOptions` translation code derived from Rust's
// library/std/src/sys/unix/fs.rs at revision
// 108e90ca78f052c0c1c49c42a22c85620be19712.

pub(crate) fn get_access_mode(options: &OpenOptions) -> io::Result<OFlags> {
    match (options.read, options.write, options.append) {
        (true, false, false) => Ok(OFlags::RDONLY),
        (false, true, false) => Ok(OFlags::WRONLY),
        (true, true, false) => Ok(OFlags::RDWR),
        (false, _, true) => Ok(OFlags::WRONLY | OFlags::APPEND),
        (true, _, true) => Ok(OFlags::RDWR | OFlags::APPEND),
        (false, false, false) => Err(Errno::INVAL.io_error()),
    }
}

pub(crate) fn get_creation_mode(options: &OpenOptions) -> io::Result<OFlags> {
    match (options.write, options.append) {
        (true, false) => {}
        (false, false) => {
            if options.truncate || options.create || options.create_new {
                return Err(Errno::INVAL.io_error());
            }
        }
        (_, true) => {
            if options.truncate && !options.create_new {
                return Err(Errno::INVAL.io_error());
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
