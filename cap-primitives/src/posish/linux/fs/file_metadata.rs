use crate::fs::{Metadata, MetadataExt};
use posish::fs::{statat, AtFlags};
use std::{
    fs, io,
    sync::atomic::{AtomicBool, Ordering::Relaxed},
};

/// Like `file.metadata()`, but works with `O_PATH` descriptors on old (pre 3.6)
/// versions of Linux too.
pub(super) fn file_metadata(file: &fs::File) -> io::Result<Metadata> {
    // Record whether we've seen an `EBADF` from an `fstat` on an `O_PATH`
    // file descriptor, meaning we're on a Linux that doesn't support it.
    static FSTAT_PATH_BADF: AtomicBool = AtomicBool::new(false);

    if !FSTAT_PATH_BADF.load(Relaxed) {
        match Metadata::from_file(file) {
            Ok(metadata) => return Ok(metadata),
            Err(e) => match e.raw_os_error() {
                // Before Linux 3.6, `fstat` with `O_PATH` returned `EBADF`.
                Some(libc::EBADF) => FSTAT_PATH_BADF.store(true, Relaxed),
                _ => return Err(e),
            },
        }
    }

    // If `fstat` with `O_PATH` isn't supported, use `statat` with `AT_EMPTY_PATH`.
    statat(file, "", AtFlags::EMPTY_PATH).map(MetadataExt::from_libc)
}
