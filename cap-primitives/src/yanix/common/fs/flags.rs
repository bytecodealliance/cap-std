use crate::fs::OpenOptions;
use yanix::file::OFlag;
use std::io;

pub(crate) fn compute_oflags(options: &OpenOptions) -> io::Result<OFlag> {
    // TODO: Add `CLOEXEC` when yanix is updated.
    let mut oflags = OFlag::empty();
    oflags |= OFlag::from_bits(get_access_mode(options)?).unwrap();
    oflags |= OFlag::from_bits(get_creation_mode(options)?).unwrap();
    if options.nofollow {
        oflags |= OFlag::NOFOLLOW;
    }
    oflags |= OFlag::from_bits(options.ext.custom_flags).expect("unrecognized OFlag bits")
        & !OFlag::ACCMODE;
    Ok(oflags)
}

// `OpenOptions` translation code derived from Rust's src/libstd/sys/unix/fs.rs

fn get_access_mode(options: &OpenOptions) -> io::Result<libc::c_int> {
    match (options.read, options.write, options.append) {
        (true, false, false) => Ok(libc::O_RDONLY),
        (false, true, false) => Ok(libc::O_WRONLY),
        (true, true, false) => Ok(libc::O_RDWR),
        (false, _, true) => Ok(libc::O_WRONLY | libc::O_APPEND),
        (true, _, true) => Ok(libc::O_RDWR | libc::O_APPEND),
        (false, false, false) => Err(io::Error::from_raw_os_error(libc::EINVAL)),
    }
}

fn get_creation_mode(options: &OpenOptions) -> io::Result<libc::c_int> {
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

    Ok(match (options.create, options.truncate, options.create_new) {
        (false, false, false) => 0,
        (true, false, false) => libc::O_CREAT,
        (false, true, false) => libc::O_TRUNC,
        (true, true, false) => libc::O_CREAT | libc::O_TRUNC,
        (_, _, true) => libc::O_CREAT | libc::O_EXCL,
    })
}
