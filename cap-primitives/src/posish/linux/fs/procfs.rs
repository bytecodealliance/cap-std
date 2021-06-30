//! Utilities for working with `/proc`, where Linux's `procfs` is typically
//! mounted. `/proc` serves as an adjunct to Linux's main syscall surface area,
//! providing additional features with an awkward interface.
//!
//! This module does a considerable amount of work to determine whether `/proc`
//! is mounted, with actual `procfs`, and without any additional mount points on
//! top of the paths we open.

use crate::fs::{
    errors, open, read_link_unchecked, set_times_follow_unchecked, OpenOptions, SystemTimeSpec,
};
use io_lifetimes::{AsFd, AsFilelike};
use posish::{
    fs::{chmodat, Mode, OFlags, RawMode},
    io::proc_self_fd,
    path::DecInt,
};
use std::{
    fs, io,
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
};

pub(crate) fn get_path_from_proc_self_fd(file: &fs::File) -> io::Result<PathBuf> {
    read_link_unchecked(
        &proc_self_fd()?.0.as_filelike_view::<fs::File>(),
        &DecInt::from_fd(file),
        PathBuf::new(),
    )
}

pub(crate) fn set_permissions_through_proc_self_fd(
    start: &fs::File,
    path: &Path,
    perm: fs::Permissions,
) -> io::Result<()> {
    let opath = open(
        start,
        path,
        OpenOptions::new()
            .read(true)
            .custom_flags(OFlags::PATH.bits() as i32),
    )?;

    let dirfd = proc_self_fd()?.0;
    let mode = Mode::from_bits(perm.mode() as RawMode).ok_or_else(errors::invalid_flags)?;
    Ok(chmodat(&dirfd, DecInt::from_fd(&opath), mode)?)
}

pub(crate) fn set_times_through_proc_self_fd(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    let opath = open(
        start,
        path,
        OpenOptions::new()
            .read(true)
            .custom_flags(OFlags::PATH.bits() as i32),
    )?;

    // Don't pass `AT_SYMLINK_NOFOLLOW`, because we do actually want to follow
    // the first symlink. We don't want to follow any subsequent symlinks, but
    // omitting `O_NOFOLLOW` above ensures that the destination of the link
    // isn't a symlink.
    set_times_follow_unchecked(
        proc_self_fd()?.0.as_fd(),
        &DecInt::from_fd(&opath),
        atime,
        mtime,
    )
}
