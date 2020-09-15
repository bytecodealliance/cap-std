//! Utilities for working with `/proc`, where Linux's `procfs` is typically
//! mounted. `/proc` serves as an adjunct to Linux's main syscall surface area,
//! providing additional features with an awkward interface.
//!
//! This module does a considerable amount of work to determine whether `/proc`
//! is mounted, with actual `procfs`, and without any additional mount points on
//! top of the paths we open.

use super::file_metadata;
use crate::fs::{
    errors, open, open_unchecked, readlink_unchecked, set_times_follow_unchecked, FollowSymlinks,
    Metadata, OpenOptions, SystemTimeSpec,
};
use posish::fs::{chmodat, fstatfs, renameat, Mode};
use std::{
    fs, io,
    os::unix::{
        fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
        io::AsRawFd,
    },
    path::{Path, PathBuf},
};

/// Linux's procfs always uses inode 1 for its root directory.
const PROC_ROOT_INO: u64 = 1;

/// The filesystem magic number for procfs.
///
/// This is defined in the `libc` crate for linux-gnu but not for
/// linux-musl, so we define it ourselves.
#[cfg(not(target_env = "musl"))]
const PROC_SUPER_MAGIC: libc::__fsword_t = 0x0000_9fa0;
#[cfg(target_env = "musl")]
const PROC_SUPER_MAGIC: libc::c_ulong = 0x0000_9fa0;

lazy_static! {
    static ref PROC_SELF_FD: io::Result<fs::File> = init_proc_self_fd();
}

// Identify a subdirectory of "/proc", to determine which anomalies to
// check for.
enum Subdir {
    Proc,
    Pid,
    Fd,
}

/// Open a handle for "/proc/self/fd".
#[allow(clippy::useless_conversion)]
fn init_proc_self_fd() -> io::Result<fs::File> {
    // When libc does have this constant, check that our copy has the same value.
    #[cfg(not(target_env = "musl"))]
    assert_eq!(
        PROC_SUPER_MAGIC,
        libc::__fsword_t::from(libc::PROC_SUPER_MAGIC)
    );

    // Open "/proc". Here and below, use `read(true)` even though we don't need
    // read permissions, because Rust's libstd requires an access mode, and
    // Linux ignores `O_RDONLY` with `O_PATH`.
    let proc = fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_PATH | libc::O_DIRECTORY | libc::O_NOFOLLOW)
        .open("/proc")?;
    let proc_metadata = check_proc_dir(Subdir::Proc, &proc, None, 0, 0)?;

    let (uid, gid, pid) = unsafe { (libc::getuid(), libc::getgid(), libc::getpid()) };
    let mut options = OpenOptions::new();
    let options = options
        .read(true)
        .follow(FollowSymlinks::No)
        .custom_flags(libc::O_PATH | libc::O_DIRECTORY);

    // Open "/proc/self". Use our pid to compute the name rather than literally
    // using "self", as "self" is a symlink.
    let proc_self = open_unchecked(&proc, Path::new(&pid.to_string()), options)?;
    drop(proc);
    check_proc_dir(Subdir::Pid, &proc_self, Some(&proc_metadata), uid, gid)?;

    // Open "/proc/self/fd".
    let proc_self_fd = open_unchecked(&proc_self, Path::new("fd"), options)?;
    drop(proc_self);
    check_proc_dir(Subdir::Fd, &proc_self_fd, Some(&proc_metadata), uid, gid)?;

    Ok(proc_self_fd)
}

/// Check a subdirectory of "/proc" for anomalies.
fn check_proc_dir(
    kind: Subdir,
    dir: &fs::File,
    proc_metadata: Option<&Metadata>,
    uid: libc::uid_t,
    gid: libc::gid_t,
) -> io::Result<Metadata> {
    // Check the filesystem magic.
    check_procfs(dir)?;

    let dir_metadata = file_metadata(dir)?;

    // Check the root inode number.
    if let Subdir::Proc = kind {
        if dir_metadata.ino() != PROC_ROOT_INO {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "unexpected root inode in /proc",
            ));
        }

        // Check that "/proc" is a mountpoint.
        if !is_mountpoint(dir)? {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "/proc isn't a mount point",
            ));
        }
    } else {
        // Check that we haven't been linked back to the root of "/proc".
        if dir_metadata.ino() == PROC_ROOT_INO {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "unexpected non-root inode in /proc subdirectory",
            ));
        }

        // Check that we're still in procfs.
        if dir_metadata.dev() != proc_metadata.unwrap().dev() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "/proc subdirectory is on a different filesystem from /proc",
            ));
        }

        // Check that subdirectories of "/proc" are not mount points.
        if is_mountpoint(dir)? {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "/proc subdirectory is a mount point",
            ));
        }
    }

    let mode = if let Subdir::Fd = kind {
        0o40500
    } else {
        0o40555
    };

    // Check that our process owns the directory.
    if dir_metadata.uid() != uid || dir_metadata.gid() != gid || dir_metadata.mode() != mode {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "/proc pid subdirectory isn't owned by the process' owner",
        ));
    }

    if let Subdir::Fd = kind {
        // Check that the "/proc/self/fd" directory doesn't have any extraneous
        // links into it (which would include unexpected subdirectories).
        if dir_metadata.nlink() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "/proc/self/fd has unexpected subdirectories or links",
            ));
        }
    } else {
        // Check that the "/proc" directory isn't empty.
        if dir_metadata.nlink() <= 2 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "/proc subdirectory is unexpectedly empty",
            ));
        }
    }

    Ok(dir_metadata)
}

/// Check that `file` is opened on a `procfs` filesystem.
fn check_procfs(file: &fs::File) -> io::Result<()> {
    let statfs = fstatfs(file)?;
    let f_type = statfs.f_type;
    if f_type != PROC_SUPER_MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("unexpected filesystem type in /proc ({:#x})", f_type),
        ));
    }

    Ok(())
}

/// Check whether the given directory handle is a mount point. We use
/// a `rename` call that would otherwise fail, but which fails with `EXDEV`
/// first if it would cross a mount point.
fn is_mountpoint(file: &fs::File) -> io::Result<bool> {
    let e = renameat(file, "../.", file, ".").unwrap_err();
    match e.raw_os_error() {
        Some(libc::EXDEV) => Ok(true), // the rename failed due to crossing a mount point
        Some(libc::EBUSY) => Ok(false), // the rename failed normally
        _ => panic!("Unexpected error from `renameat`: {:?}", e),
    }
}

fn proc_self_fd() -> io::Result<&'static fs::File> {
    PROC_SELF_FD
        .as_ref()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("error opening /proc: {}", e)))
}

pub(crate) fn get_path_from_proc_self_fd(file: &fs::File) -> io::Result<PathBuf> {
    readlink_unchecked(proc_self_fd()?, Path::new(&file.as_raw_fd().to_string()))
}

pub(crate) fn set_permissions_through_proc_self_fd(
    start: &fs::File,
    path: &Path,
    perm: fs::Permissions,
) -> io::Result<()> {
    let opath = open(
        start,
        path,
        OpenOptions::new().read(true).custom_flags(libc::O_PATH),
    )?;

    let dirfd = proc_self_fd()?;
    let mode = Mode::from_bits(perm.mode()).ok_or_else(errors::invalid_flags)?;
    chmodat(dirfd, opath.as_raw_fd().to_string(), mode)
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
        OpenOptions::new().read(true).custom_flags(libc::O_PATH),
    )?;

    // Don't pass `AT_SYMLINK_NOFOLLOW`, because we do actually want to follow
    // the first symlink. We don't want to follow any subsequent symlinks, but
    // omitting `O_NOFOLLOW` above ensures that the destination of the link
    // isn't a symlink.
    let dirfd = proc_self_fd()?;
    set_times_follow_unchecked(
        dirfd,
        Path::new(&opath.as_raw_fd().to_string()),
        atime,
        mtime,
    )
}
