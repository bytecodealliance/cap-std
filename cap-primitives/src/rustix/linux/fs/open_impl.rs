//! Linux 5.6 and later have a syscall `openat2`, with flags that allow it to
//! enforce the sandboxing property we want. See the [LWN article] for an
//! overview and the [`openat2` documentation] for details.
//!
//! [LWN article]: https://lwn.net/Articles/796868/
//! [`openat2` documentation]: https://man7.org/linux/man-pages/man2/openat2.2.html
//!
//! On older Linux, fall back to `manually::open`.

use super::super::super::fs::compute_oflags;
#[cfg(racy_asserts)]
use crate::fs::is_same_file;
use crate::fs::{errors, manually, OpenOptions};
use io_lifetimes::FromFd;
use rustix::fs::{openat2, Mode, OFlags, RawMode, ResolveFlags};
use rustix::path::Arg;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::{fs, io};

/// Call the `openat2` system call, or use a fallback if that's unavailable.
pub(crate) fn open_impl(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    let result = open_beneath(start, path, options);

    // If that returned `ENOSYS`, use a fallback strategy.
    if let Err(err) = &result {
        if Some(rustix::io::Error::NOSYS.raw_os_error()) == err.raw_os_error() {
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
    if INVALID.load(Relaxed) {
        // `openat2` is permanently unavailable.
        return Err(rustix::io::Error::NOSYS.into());
    }

    let oflags = compute_oflags(options)?;

    // Do two `contains` checks because `TMPFILE` may be represented with
    // multiple flags and we need to ensure they're all set.
    let mode = if oflags.contains(OFlags::CREATE) || oflags.contains(OFlags::TMPFILE) {
        Mode::from_bits((options.ext.mode & 0o7777) as RawMode).unwrap()
    } else {
        Mode::empty()
    };

    // On Android, seccomp kills processes that execute unrecognized system
    // calls, so we do an explicit version check rather than relying on
    // getting an `ENOSYS`.
    #[cfg(target_os = "android")]
    {
        static CHECKED: AtomicBool = AtomicBool::new(false);

        if !CHECKED.load(Relaxed) {
            if !openat2_supported() {
                INVALID.store(true, Relaxed);
                return Err(rustix::io::Error::NOSYS.into());
            }

            CHECKED.store(true, Relaxed);
        }
    }

    // We know `openat2` needs a `&CStr` internally; to avoid allocating on
    // each iteration of the loop below, allocate the `CString` now.
    path.into_with_c_str(|path_c_str| {
        // `openat2` fails with `EAGAIN` if a rename happens anywhere on the host
        // while it's running, so use a loop to retry it a few times. But not too many
        // times, because there's no limit on how often this can happen. The actual
        // number here is currently an arbitrarily chosen guess.
        for _ in 0..4 {
            match openat2(
                start,
                path_c_str,
                oflags,
                mode,
                ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS,
            ) {
                Ok(file) => {
                    let file = fs::File::from_into_fd(file);

                    #[cfg(racy_asserts)]
                    check_open(start, path, options, &file);

                    return Ok(file);
                }
                Err(err) => match err {
                    // A rename or similar happened. Try again.
                    rustix::io::Error::AGAIN => continue,

                    // `EPERM` is used by some `seccomp` sandboxes to indicate
                    // that `openat2` is unimplemented:
                    // <https://github.com/systemd/systemd/blob/e2357b1c8a87b610066b8b2a59517bcfb20b832e/src/shared/seccomp-util.c#L2066>
                    //
                    // However, `EPERM` may also indicate a failed `O_NOATIME`
                    // or a file seal prevented the operation, and it's complex
                    // to detect those cases, so exit the loop and use the
                    // fallback.
                    rustix::io::Error::PERM => break,

                    // `ENOSYS` means `openat2` is permanently unavailable;
                    // mark it so and exit the loop.
                    rustix::io::Error::NOSYS => {
                        INVALID.store(true, Relaxed);
                        break;
                    }

                    _ => return Err(err.into()),
                },
            }
        }

        Err(rustix::io::Error::NOSYS.into())
    })
    .map_err(|err| match err {
        rustix::io::Error::XDEV => errors::escape_attempt(),
        err => err.into(),
    })
}

/// Test whether `openat2` is supported on the currently running OS.
#[cfg(target_os = "android")]
fn openat2_supported() -> bool {
    // `openat2` is supported in Linux 5.6 and later. Parse the current
    // Linux version from the `release` field from `uname` to detect this.
    let uname = rustix::process::uname();
    let release = uname.release().to_bytes();
    if let Some((major, minor)) = linux_major_minor(release) {
        if major >= 6 || (major == 5 && minor >= 6) {
            return true;
        }
    }

    false
}

/// Extract the major and minor values from a Linux `release` string.
#[cfg(target_os = "android")]
fn linux_major_minor(release: &[u8]) -> Option<(u32, u32)> {
    let mut parts = release.split(|b| *b == b'.');
    if let Some(major) = parts.next() {
        if let Ok(major) = std::str::from_utf8(major) {
            if let Ok(major) = major.parse::<u32>() {
                if let Some(minor) = parts.next() {
                    if let Ok(minor) = std::str::from_utf8(minor) {
                        if let Ok(minor) = minor.parse::<u32>() {
                            return Some((major, minor));
                        }
                    }
                }
            }
        }
    }

    None
}

#[cfg(target_os = "android")]
#[test]
fn test_linux_major_minor() {
    assert_eq!(linux_major_minor(b"5.11.0-5489-something"), Some((5, 11)));
    assert_eq!(linux_major_minor(b"5.10.0-9-whatever"), Some((5, 10)));
    assert_eq!(linux_major_minor(b"5.6.0"), Some((5, 6)));
    assert_eq!(linux_major_minor(b"2.6.34"), Some((2, 6)));
    assert_eq!(linux_major_minor(b""), None);
    assert_eq!(linux_major_minor(b"linux-2.6.32"), None);
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
