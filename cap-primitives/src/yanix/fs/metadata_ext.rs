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
    pub(super) fn from_libc(mode: libc::stat) -> Metadata {
        Metadata {
            file_type: FileTypeExt::from_libc(mode.st_mode),
            len: u64::try_from(mode.st_size).unwrap(),
            permissions: PermissionsExt::from_libc(mode.st_mode),
            modified: system_time_from_libc(mode.st_mtime, mode.st_mtime_nsec),
            accessed: system_time_from_libc(mode.st_atime, mode.st_atime_nsec),
            created: system_time_from_libc(mode.st_ctime, mode.st_ctime_nsec),

            ext: Self {
                dev: u64::try_from(mode.st_dev).unwrap(),
                ino: mode.st_ino.into(),
                mode: u32::from(mode.st_mode),
                nlink: u64::from(mode.st_nlink),
                uid: mode.st_uid,
                gid: mode.st_gid,
                rdev: u64::try_from(mode.st_rdev).unwrap(),
                size: u64::try_from(mode.st_size).unwrap(),
                atime: i64::from(mode.st_atime),
                atime_nsec: i64::from(mode.st_atime_nsec),
                mtime: i64::from(mode.st_mtime),
                mtime_nsec: i64::from(mode.st_mtime_nsec),
                ctime: i64::from(mode.st_ctime),
                ctime_nsec: i64::from(mode.st_ctime_nsec),
                blksize: u64::try_from(mode.st_blksize).unwrap(),
                blocks: u64::try_from(mode.st_blocks).unwrap(),
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
