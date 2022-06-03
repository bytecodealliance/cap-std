use crate::fs::{FollowSymlinks, Metadata, MetadataExt};
use rustix::fs::{statat, AtFlags};
use std::path::Path;
use std::{fs, io};

// TODO: update all these to
// #[cfg(any(target_os = "android", target_os = "linux"))]
// once we're on restix >= v0.34.3.
#[cfg(all(target_os = "linux", target_env = "gnu"))]
use rustix::fs::{statx, StatxFlags};
#[cfg(all(target_os = "linux", target_env = "gnu"))]
use std::sync::atomic::{AtomicBool, Ordering};

/// *Unsandboxed* function similar to `stat`, but which does not perform
/// sandboxing.
pub(crate) fn stat_unchecked(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    let atflags = match follow {
        FollowSymlinks::Yes => AtFlags::empty(),
        FollowSymlinks::No => AtFlags::SYMLINK_NOFOLLOW,
    };

    // `statx` is preferred on Linux because it can return creation times.
    // Linux kernels prior to 4.11 don't have `statx` and return `ENOSYS`. We
    // store the availability in a global to avoid unnecessary syscalls.
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    static HAS_STATX: AtomicBool = AtomicBool::new(true);

    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    if HAS_STATX.load(Ordering::Relaxed) {
        let statx_result = statx(
            start,
            path,
            atflags,
            StatxFlags::BASIC_STATS | StatxFlags::BTIME,
        );
        if let Err(rustix::io::Error::NOSYS) = statx_result {
            HAS_STATX.store(false, Ordering::Relaxed);
        } else {
            return Ok(statx_result.map(MetadataExt::from_rustix_statx)?);
        }
    }

    Ok(statat(start, path, atflags).map(MetadataExt::from_rustix)?)
}
