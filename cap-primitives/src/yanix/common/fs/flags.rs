use crate::fs::OpenOptions;
use yanix::file::OFlag;

pub(crate) fn compute_oflags(options: &OpenOptions) -> OFlag {
    // TODO: Diagnose invalid combinations.
    let mut oflags = OFlag::empty();
    if options.read && options.write {
        oflags |= OFlag::RDWR;
    } else if options.read {
        oflags |= OFlag::RDONLY;
    } else if options.write {
        oflags |= OFlag::WRONLY;
    }
    if options.append {
        oflags |= OFlag::APPEND;
    }
    if options.create_new {
        oflags |= OFlag::EXCL | OFlag::CREAT;
    } else {
        if options.truncate {
            oflags |= OFlag::TRUNC;
        }
        if options.create {
            oflags |= OFlag::CREAT;
        }
    }
    if options.nofollow {
        oflags |= OFlag::NOFOLLOW;
    }
    oflags |= OFlag::from_bits(options.ext.custom_flags).expect("unrecognized OFlag bits")
        & !OFlag::ACCMODE;
    oflags
}
