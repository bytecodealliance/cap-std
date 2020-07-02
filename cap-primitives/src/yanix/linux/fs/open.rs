//! Linux 5.6 and later have a syscall `openat2`, with flags that allow it to
//! enforce the sandboxing property we want. See [the LWN article] for details.
//!
//! [the LWN article]: https://lwn.net/Articles/796868/
//!
//! On older Linux, fall back to `open_manually`.

#[cfg(debug_assertions)]
use crate::fs::is_same_file;
use crate::{
    fs::OpenOptions,
    fs::{compute_oflags, open_manually_wrapper},
};
use std::{
    ffi::CString,
    fs, io,
    os::unix::{
        ffi::OsStrExt,
        io::{AsRawFd, FromRawFd, RawFd},
    },
    path::Path,
    ptr,
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

/// Call the `openat2` system call.
fn open_openat2(start: &fs::File, path: &Path, options: &OpenOptions) -> io::Result<fs::File> {
    let oflags = compute_oflags(options);
    let mode = options.ext.mode;

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
                    libc::EXDEV => return escape_attempt(),
                    errno => return other_error(errno),
                },
                ret => {
                    let fd = ret as RawFd;

                    #[cfg(debug_assertions)]
                    {
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
                        debug_assert!(
                            is_same_file(start, &check)?,
                            "open_manually should open the same inode as open_openat2"
                        );
                    }

                    return Ok(fs::File::from_raw_fd(fd));
                }
            }
        }
    }

    // Fall back to the manual-resolution path.
    open_manually_wrapper(start, path, options)
}

lazy_static! {
    static ref OPEN: fn(&fs::File, &Path, &OpenOptions) -> io::Result<fs::File> = {
        // Test if `openat2` is supported. If so, we can use `open_openat2`.
        // Otherwise, fall back to `open_manually`.
        unsafe {
            if let -1 = libc::syscall(
                SYS_OPENAT2,
                -1,
                ptr::null::<libc::c_void>(),
                ptr::null::<libc::c_void>(),
                0,
            ) {
                if let libc::ENOSYS = io::Error::last_os_error().raw_os_error().unwrap() {
                    // We're on an older Linux.
                } else {
                    return open_openat2;
                }
            }
        }

        open_manually_wrapper
    };
}

#[inline]
pub(crate) fn open_impl(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    OPEN(start, path, options)
}

#[cold]
fn escape_attempt() -> io::Result<fs::File> {
    Err(io::Error::new(
        io::ErrorKind::PermissionDenied,
        "a path led outside of the filesystem",
    ))
}

#[cold]
fn other_error(errno: i32) -> io::Result<fs::File> {
    Err(io::Error::from_raw_os_error(errno))
}
