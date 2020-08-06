//! Linux 5.6 and later have a syscall `openat2`, with flags that allow it to
//! enforce the sandboxing property we want. See the [LWN article] for an
//! overview and the [openat2 documentation] for details.
//!
//! [LWN article]: https://lwn.net/Articles/796868/
//! [openat2 documentation]: https://man7.org/linux/man-pages/man2/openat2.2.html
//!
//! On older Linux, fall back to `open_manually`.

use super::super::super::fs::compute_oflags;
#[cfg(not(feature = "no_racy_asserts"))]
use crate::fs::is_same_file;
use crate::fs::{errors, open_manually_wrapper, OpenOptions};
use std::{
    ffi::CString,
    fs, io,
    os::unix::{
        ffi::OsStrExt,
        io::{AsRawFd, FromRawFd, RawFd},
    },
    path::Path,
    sync::atomic::{AtomicBool, Ordering::Relaxed},
};

const SYS_OPENAT2: i64 = 437;
const RESOLVE_NO_MAGICLINKS: u64 = 0x02;
const RESOLVE_BENEATH: u64 = 0x08;

#[repr(C)]
#[derive(Debug, Default)]
struct OpenHow {
    oflag: u64,
    mode: u64,
    resolve: u64,
}
const SIZEOF_OPEN_HOW: usize = std::mem::size_of::<OpenHow>();

/// Call the `openat2` system call, or use a fallback if that's unavailable.
pub(crate) fn open_impl(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    let result = open_with_openat2(start, path, options);

    // If that returned `ENOSYS`, use a fallback strategy.
    if let Err(e) = &result {
        if let Some(libc::ENOSYS) = e.raw_os_error() {
            return open_manually_wrapper(start, path, options);
        }
    }

    result
}

/// Call the `openat2` system call. If the syscall is unavailable, mark it so
/// for future calls. If `openat2` is unavailable either permanently or
/// temporarily, return `ENOSYS`.
pub(crate) fn open_with_openat2(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    static INVALID: AtomicBool = AtomicBool::new(false);
    if !INVALID.load(Relaxed) {
        let oflags = compute_oflags(options)?;

        // TODO use `yanix::file::OFlags` when `TMPFILE` is introduced
        // Until then, since `O_TMPFILE := 0x20000000 | libc::O_DIRECTORY`,
        // we need to compare for bit equality.
        let mode = if (oflags.bits() & libc::O_CREAT == libc::O_CREAT)
            || (oflags.bits() & libc::O_TMPFILE == libc::O_TMPFILE)
        {
            options.ext.mode
        } else {
            0
        };

        let path_cstr = CString::new(path.as_os_str().as_bytes())?;
        let open_how = OpenHow {
            oflag: u64::from(oflags.bits() as u32),
            mode: u64::from(mode),
            resolve: RESOLVE_BENEATH | RESOLVE_NO_MAGICLINKS,
        };

        // `openat2` fails with `EAGAIN` if a rename happens anywhere on the host
        // while it's running, so use a loop to retry it a few times. But not too many
        // times, because there's no limit on how often this can happen.
        for _ in 0..4 {
            unsafe {
                match libc::syscall(
                    SYS_OPENAT2,
                    start.as_raw_fd(),
                    path_cstr.as_ptr(),
                    &open_how,
                    SIZEOF_OPEN_HOW,
                ) {
                    -1 => match io::Error::last_os_error().raw_os_error().unwrap() {
                        libc::EAGAIN => continue,
                        libc::EXDEV => return Err(errors::escape_attempt()),
                        libc::ENOSYS => {
                            // `openat2` is permanently unavailable; mark it so and
                            // exit the loop.
                            INVALID.store(true, Relaxed);
                            break;
                        }
                        errno => return other_error(errno),
                    },
                    ret => {
                        // Note that we don't bother with `ensure_cloexec` here
                        // because Linux has supported `O_CLOEXEC` since 2.6.18,
                        // and `openat2` was introduced in 5.6.
                        let file = fs::File::from_raw_fd(ret as RawFd);

                        #[cfg(not(feature = "no_racy_asserts"))]
                        check_open(start, path, options, &file);

                        return Ok(file);
                    }
                }
            }
        }
    }

    // `openat2` is unavailable, either temporarily or permanently.
    other_error(libc::ENOSYS)
}

fn other_error(errno: i32) -> io::Result<fs::File> {
    Err(io::Error::from_raw_os_error(errno))
}

#[cfg(not(feature = "no_racy_asserts"))]
fn check_open(start: &fs::File, path: &Path, options: &OpenOptions, file: &fs::File) {
    let check = open_manually_wrapper(
        start,
        path,
        options
            .clone()
            .create(false)
            .create_new(false)
            .truncate(false),
    )
    .expect("open_manually failed when open_openat2 succeeded");
    assert!(
        is_same_file(file, &check).expect("open_manually should be able to stat the result"),
        "open_manually should open the same inode as open_openat2"
    );
}
