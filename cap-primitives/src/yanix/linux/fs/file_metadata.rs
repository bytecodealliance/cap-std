use crate::fs::{Metadata, MetadataExt};
use std::{
    fs, io,
    os::unix::io::AsRawFd,
    sync::atomic::{AtomicBool, Ordering::Relaxed},
};
use yanix::file::{fstatat, AtFlags};

/// Like `file.metadata()`, but works with `O_PATH` descriptors on old (pre 3.6)
/// versions of Linux too.
pub(super) fn file_metadata(file: &fs::File) -> io::Result<Metadata> {
    // Record whether we've seen an `EBADF` from an `fstat` on an `O_PATH`
    // file descriptor, meaning we're on a Linux that doesn't support it.
    static FSTAT_PATH_BADF: AtomicBool = AtomicBool::new(false);

    if !FSTAT_PATH_BADF.load(Relaxed) {
        match file.metadata() {
            Ok(metadata) => return Ok(Metadata::from_std(metadata)),
            Err(e) => match e.raw_os_error() {
                // Before Linux 3.6, `fstat` with `O_PATH` returned `EBADF`.
                Some(libc::EBADF) => FSTAT_PATH_BADF.store(true, Relaxed),
                _ => return Err(e),
            },
        }
    }

    // If `fstat` with `O_PATH` isn't supported, use `fstatat` with `AT_EMPTY_PATH`.
    unsafe { fstatat(file.as_raw_fd(), "", AtFlags::EMPTY_PATH) }.map(MetadataExt::from_libc)
}
