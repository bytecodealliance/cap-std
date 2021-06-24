#![allow(clippy::useless_conversion)]

use crate::{
    fs::{FileTypeExt, Metadata, PermissionsExt},
    time::{Duration, SystemClock, SystemTime},
};
#[cfg(all(target_os = "linux", target_env = "gnu"))]
use posish::fs::{makedev, Statx};
use posish::fs::{RawMode, Stat};
use std::{
    convert::{TryFrom, TryInto},
    fs, io,
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
    /// Constructs a new instance of `Self` from the given [`std::fs::File`]
    /// and [`std::fs::Metadata`].
    #[inline]
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn from(_file: &fs::File, std: &fs::Metadata) -> io::Result<Self> {
        // On Posish-style platforms, the `Metadata` has everything we need.
        Ok(Self::from_just_metadata(std))
    }

    /// Constructs a new instance of `Self` from the given
    /// [`std::fs::Metadata`].
    #[inline]
    pub(crate) fn from_just_metadata(std: &fs::Metadata) -> Self {
        use posish::fs::MetadataExt;
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

    /// Constructs a new instance of `Metadata` from the given `Stat`.
    #[inline]
    pub(crate) fn from_posish(stat: Stat) -> Metadata {
        Metadata {
            file_type: FileTypeExt::from_raw_mode(stat.st_mode as RawMode),
            len: u64::try_from(stat.st_size).unwrap(),
            permissions: PermissionsExt::from_raw_mode(stat.st_mode as RawMode),

            #[cfg(not(target_os = "netbsd"))]
            modified: system_time_from_posish(
                stat.st_mtime.try_into().unwrap(),
                stat.st_mtime_nsec as _,
            ),
            #[cfg(not(target_os = "netbsd"))]
            accessed: system_time_from_posish(
                stat.st_atime.try_into().unwrap(),
                stat.st_atime_nsec as _,
            ),

            #[cfg(target_os = "netbsd")]
            modified: system_time_from_posish(
                stat.st_mtime.try_into().unwrap(),
                stat.st_mtimensec as _,
            ),
            #[cfg(target_os = "netbsd")]
            accessed: system_time_from_posish(
                stat.st_atime.try_into().unwrap(),
                stat.st_atimensec as _,
            ),

            #[cfg(any(
                target_os = "freebsd",
                target_os = "openbsd",
                target_os = "macos",
                target_os = "ios"
            ))]
            created: system_time_from_posish(
                stat.st_birthtime.try_into().unwrap(),
                stat.st_birthtime_nsec as _,
            ),

            #[cfg(target_os = "netbsd")]
            created: system_time_from_posish(
                stat.st_birthtime.try_into().unwrap(),
                stat.st_birthtimensec as _,
            ),

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
                atime: i64::try_from(stat.st_atime).unwrap(),
                #[cfg(not(target_os = "netbsd"))]
                atime_nsec: stat.st_atime_nsec as _,
                #[cfg(target_os = "netbsd")]
                atime_nsec: stat.st_atimensec as _,
                mtime: i64::try_from(stat.st_mtime).unwrap(),
                #[cfg(not(target_os = "netbsd"))]
                mtime_nsec: stat.st_mtime_nsec as _,
                #[cfg(target_os = "netbsd")]
                mtime_nsec: stat.st_mtimensec as _,
                ctime: i64::try_from(stat.st_ctime).unwrap(),
                #[cfg(not(target_os = "netbsd"))]
                ctime_nsec: stat.st_ctime_nsec as _,
                #[cfg(target_os = "netbsd")]
                ctime_nsec: stat.st_ctimensec as _,
                blksize: u64::try_from(stat.st_blksize).unwrap(),
                blocks: u64::try_from(stat.st_blocks).unwrap(),
            },
        }
    }

    /// Constructs a new instance of `Metadata` from the given `Statx`.
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    #[inline]
    #[allow(dead_code)] // TODO: use `statx` when possible.
    pub(crate) fn from_posish_statx(statx: Statx) -> Metadata {
        Metadata {
            file_type: FileTypeExt::from_raw_mode(RawMode::from(statx.stx_mode)),
            len: u64::try_from(statx.stx_size).unwrap(),
            permissions: PermissionsExt::from_raw_mode(RawMode::from(statx.stx_mode)),
            modified: system_time_from_posish(statx.stx_mtime.tv_sec, statx.stx_mtime.tv_nsec as _),
            accessed: system_time_from_posish(statx.stx_atime.tv_sec, statx.stx_atime.tv_nsec as _),
            created: system_time_from_posish(statx.stx_btime.tv_sec, statx.stx_btime.tv_nsec as _),

            ext: Self {
                dev: makedev(statx.stx_dev_major, statx.stx_dev_minor),
                ino: statx.stx_ino.into(),
                mode: u32::from(statx.stx_mode),
                nlink: u64::from(statx.stx_nlink),
                uid: statx.stx_uid,
                gid: statx.stx_gid,
                rdev: makedev(statx.stx_rdev_major, statx.stx_rdev_minor),
                size: statx.stx_size,
                atime: i64::from(statx.stx_atime.tv_sec),
                atime_nsec: statx.stx_atime.tv_nsec as _,
                mtime: i64::from(statx.stx_mtime.tv_sec),
                mtime_nsec: statx.stx_mtime.tv_nsec as _,
                ctime: i64::from(statx.stx_ctime.tv_sec),
                ctime_nsec: statx.stx_ctime.tv_nsec as _,
                blksize: u64::from(statx.stx_blksize),
                blocks: statx.stx_blocks,
            },
        }
    }

    /// Determine if `self` and `other` refer to the same inode on the same device.
    pub(crate) const fn is_same_file(&self, other: &Self) -> bool {
        self.dev == other.dev && self.ino == other.ino
    }
}

#[allow(clippy::similar_names)]
fn system_time_from_posish(sec: i64, nsec: u64) -> Option<SystemTime> {
    SystemClock::UNIX_EPOCH.checked_add(Duration::new(u64::try_from(sec).unwrap(), nsec as _))
}

impl posish::fs::MetadataExt for MetadataExt {
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
