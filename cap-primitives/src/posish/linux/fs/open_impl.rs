//! Linux 5.6 and later have a syscall `openat2`, with flags that allow it to
//! enforce the sandboxing property we want. See the [LWN article] for an
//! overview and the [`openat2` documentation] for details.
//!
//! [LWN article]: https://lwn.net/Articles/796868/
//! [`openat2` documentation]: https://man7.org/linux/man-pages/man2/openat2.2.html
//!
//! On older Linux, fall back to `manually::open`.

use super::super::super::fs::{c_str, compute_oflags};
#[cfg(racy_asserts)]
use crate::fs::is_same_file;
use crate::fs::{errors, manually, OpenOptions};
use posish::fs::{openat2, Mode, OFlags, ResolveFlags};
use std::{
    fs, io,
    path::Path,
    sync::atomic::{AtomicBool, Ordering::Relaxed},
};

/// Call the `openat2` system call, or use a fallback if that's unavailable.
pub(crate) fn open_impl(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    let result = open_beneath(start, path, options);

    // If that returned `ENOSYS`, use a fallback strategy.
    if let Err(e) = &result {
        if let Some(libc::ENOSYS) = e.raw_os_error() {
            return manually::open(start, path, options);
        }
    }

    result
}

/// Call the `openat2` system call with `RESOLVE_BENEATH`. If the syscall is
/// unavailable, mark it so for future calls. If `openat2` is unavailable
/// either permanently or temporarily, return `ENOSYS`.
pub(crate) fn open_beneath(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    static INVALID: AtomicBool = AtomicBool::new(false);
    if !INVALID.load(Relaxed) {
        let oflags = compute_oflags(options)?;

        // Do two `contains` checks because `TMPFILE` may be represented with
        // multiple flags and we need to ensure they're all set.
        let mode = if oflags.contains(OFlags::CREATE) || oflags.contains(OFlags::TMPFILE) {
            Mode::from_bits(options.ext.mode & 0o7777).unwrap()
        } else {
            Mode::empty()
        };

        // We know `openat2` needs a `&CStr` internally; to avoid allocating on
        // each iteration of the loop below, allocate the `CString` now.
        let path_c_str = c_str(path)?;

        // `openat2` fails with `EAGAIN` if a rename happens anywhere on the host
        // while it's running, so use a loop to retry it a few times. But not too many
        // times, because there's no limit on how often this can happen. The actual
        // number here is currently an arbitrarily chosen guess.
        for _ in 0..4 {
            match openat2(
                start,
                path_c_str.as_c_str(),
                oflags,
                mode,
                ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS,
            ) {
                Ok(file) => {
                    // Note that we don't bother with `ensure_cloexec` here
                    // because Linux has supported `O_CLOEXEC` since 2.6.18,
                    // and `openat2` was introduced in 5.6.

                    #[cfg(racy_asserts)]
                    check_open(start, path, options, &file);

                    return Ok(file);
                }
                Err(err) => match err.raw_os_error() {
                    Some(libc::EAGAIN) => continue,
                    Some(libc::EXDEV) => return Err(errors::escape_attempt()),
                    Some(libc::ENOSYS) => {
                        // `openat2` is permanently unavailable; mark it so and
                        // exit the loop.
                        INVALID.store(true, Relaxed);
                        break;
                    }
                    _ => return Err(err),
                },
            }
        }
    }

    // `openat2` is unavailable, either temporarily or permanently.
    Err(io::Error::from_raw_os_error(libc::ENOSYS))
}

#[cfg(racy_asserts)]
fn check_open(start: &fs::File, path: &Path, options: &OpenOptions, file: &fs::File) {
    let check = manually::open(
        start,
        path,
        options
            .clone()
            .create(false)
            .create_new(false)
            .truncate(false),
    )
    .expect("manually::open failed when open_openat2 succeeded");
    assert!(
        is_same_file(file, &check).unwrap(),
        "manually::open should open the same inode as open_openat2"
    );
}
