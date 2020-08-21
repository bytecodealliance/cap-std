#![allow(clippy::useless_conversion)]

use crate::fs::{FileTypeExt, Metadata, PermissionsExt};
use std::{
    convert::TryFrom,
    fs,
    time::{Duration, SystemTime},
};

#[derive(Debug, Clone)]
pub(crate) struct MetadataExt {
    dev: u64,
    ino: u64,
    mode: u32,
    nlink: u64,
    uid: u32,
    gid: u32,
    rdev: u64,
    size: u64,
    atime: i64,
    atime_nsec: i64,
    mtime: i64,
    mtime_nsec: i64,
    ctime: i64,
    ctime_nsec: i64,
    blksize: u64,
    blocks: u64,
}

impl MetadataExt {
    /// Constructs a new instance of `Self` from the given `std::fs::Metadata`.
    #[inline]
    pub(crate) fn from_std(std: fs::Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;
        Self {
            dev: std.dev(),
            ino: std.ino(),
            mode: std.mode(),
            nlink: std.nlink(),
            uid: std.uid(),
            gid: std.gid(),
            rdev: std.rdev(),
            size: std.size(),
            atime: std.atime(),
            atime_nsec: std.atime_nsec(),
            mtime: std.mtime(),
            mtime_nsec: std.mtime_nsec(),
            ctime: std.ctime(),
            ctime_nsec: std.ctime_nsec(),
            blksize: std.blksize(),
            blocks: std.blocks(),
        }
    }

    /// Constructs a new instance of `Metadata` from the given `libc::stat`.
    #[inline]
    pub(crate) fn from_libc(stat: libc::stat) -> Metadata {
        Metadata {
            file_type: FileTypeExt::from_libc(stat.st_mode),
            len: u64::try_from(stat.st_size).unwrap(),
            permissions: PermissionsExt::from_libc(stat.st_mode),
            modified: system_time_from_libc(stat.st_mtime, stat.st_mtime_nsec),
            accessed: system_time_from_libc(stat.st_atime, stat.st_atime_nsec),

            #[cfg(any(
                target_os = "freebsd",
                target_os = "openbsd",
                target_os = "macos",
                target_os = "ios"
            ))]
            created: system_time_from_libc(stat.st_birthtime, stat.st_birthtime_nsec),

            #[cfg(target_os = "netbsd")]
            created: system_time_from_libc(stat.st_birthtime, stat.st_birthtimensec),

            // `stat.st_ctime` is the latest status change; we want the creation.
            #[cfg(not(any(
                target_os = "freebsd",
                target_os = "openbsd",
                target_os = "macos",
                target_os = "ios",
                target_os = "netbsd"
            )))]
            created: None,

            ext: Self {
                dev: u64::try_from(stat.st_dev).unwrap(),
                ino: stat.st_ino.into(),
                mode: u32::from(stat.st_mode),
                nlink: u64::from(stat.st_nlink),
                uid: stat.st_uid,
                gid: stat.st_gid,
                rdev: u64::try_from(stat.st_rdev).unwrap(),
                size: u64::try_from(stat.st_size).unwrap(),
                atime: i64::from(stat.st_atime),
                atime_nsec: i64::from(stat.st_atime_nsec),
                mtime: i64::from(stat.st_mtime),
                mtime_nsec: i64::from(stat.st_mtime_nsec),
                ctime: i64::from(stat.st_ctime),
                ctime_nsec: i64::from(stat.st_ctime_nsec),
                blksize: u64::try_from(stat.st_blksize).unwrap(),
                blocks: u64::try_from(stat.st_blocks).unwrap(),
            },
        }
    }

    /// Constructs a new instance of `Metadata` from the given `libc::statx`.
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    #[inline]
    #[allow(dead_code)] // TODO: Add `statx` to yanix and use this when possible.
    pub(crate) fn from_libc_statx(statx: libc::statx) -> Metadata {
        Metadata {
            file_type: FileTypeExt::from_libc(libc::mode_t::from(statx.stx_mode)),
            len: u64::try_from(statx.stx_size).unwrap(),
            permissions: PermissionsExt::from_libc(libc::mode_t::from(statx.stx_mode)),
            modified: system_time_from_libc(
                statx.stx_mtime.tv_sec,
                i64::from(statx.stx_mtime.tv_nsec),
            ),
            accessed: system_time_from_libc(
                statx.stx_atime.tv_sec,
                i64::from(statx.stx_atime.tv_nsec),
            ),
            created: system_time_from_libc(
                statx.stx_btime.tv_sec,
                i64::from(statx.stx_btime.tv_nsec),
            ),

            ext: Self {
                dev: unsafe { libc::makedev(statx.stx_dev_major, statx.stx_dev_minor) },
                ino: statx.stx_ino.into(),
                mode: u32::from(statx.stx_mode),
                nlink: u64::from(statx.stx_nlink),
                uid: statx.stx_uid,
                gid: statx.stx_gid,
                rdev: unsafe { libc::makedev(statx.stx_rdev_major, statx.stx_rdev_minor) },
                size: statx.stx_size,
                atime: i64::from(statx.stx_atime.tv_sec),
                atime_nsec: i64::from(statx.stx_atime.tv_nsec),
                mtime: i64::from(statx.stx_mtime.tv_sec),
                mtime_nsec: i64::from(statx.stx_mtime.tv_nsec),
                ctime: i64::from(statx.stx_ctime.tv_sec),
                ctime_nsec: i64::from(statx.stx_ctime.tv_nsec),
                blksize: u64::from(statx.stx_blksize),
                blocks: statx.stx_blocks,
            },
        }
    }

    /// Determine if `self` and `other` refer to the same inode on the same device.
    pub(crate) fn is_same_file(&self, other: &Self) -> bool {
        self.dev == other.dev && self.ino == other.ino
    }
}

#[allow(clippy::similar_names)]
fn system_time_from_libc(sec: libc::time_t, nsec: libc::time_t) -> Option<SystemTime> {
    SystemTime::UNIX_EPOCH.checked_add(Duration::new(
        u64::try_from(sec).unwrap(),
        u32::try_from(nsec).unwrap(),
    ))
}

impl std::os::unix::fs::MetadataExt for MetadataExt {
    #[inline]
    fn dev(&self) -> u64 {
        self.dev
    }

    #[inline]
    fn ino(&self) -> u64 {
        self.ino
    }

    #[inline]
    fn mode(&self) -> u32 {
        self.mode
    }

    #[inline]
    fn nlink(&self) -> u64 {
        self.nlink
    }

    #[inline]
    fn uid(&self) -> u32 {
        self.uid
    }

    #[inline]
    fn gid(&self) -> u32 {
        self.gid
    }

    #[inline]
    fn rdev(&self) -> u64 {
        self.rdev
    }

    #[inline]
    fn size(&self) -> u64 {
        self.size
    }

    #[inline]
    fn atime(&self) -> i64 {
        self.atime
    }

    #[inline]
    fn atime_nsec(&self) -> i64 {
        self.atime_nsec
    }

    #[inline]
    fn mtime(&self) -> i64 {
        self.mtime
    }

    #[inline]
    fn mtime_nsec(&self) -> i64 {
        self.mtime_nsec
    }

    #[inline]
    fn ctime(&self) -> i64 {
        self.ctime
    }

    #[inline]
    fn ctime_nsec(&self) -> i64 {
        self.ctime_nsec
    }

    #[inline]
    fn blksize(&self) -> u64 {
        self.blksize
    }

    #[inline]
    fn blocks(&self) -> u64 {
        self.blocks
    }
}
