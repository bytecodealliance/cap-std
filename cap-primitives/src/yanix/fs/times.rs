use super::{cstr, cvt_i32};
use crate::fs::FileTimeSpec;
use std::{convert::TryInto, fs, io, os::unix::io::AsRawFd, path::Path};

pub(crate) fn to_timespec(ft: Option<FileTimeSpec>) -> io::Result<libc::timespec> {
    Ok(match ft {
        None => libc::timespec {
            tv_sec: 0,
            tv_nsec: libc::UTIME_OMIT,
        },
        Some(FileTimeSpec::SymbolicNow) => libc::timespec {
            tv_sec: 0,
            tv_nsec: libc::UTIME_NOW,
        },
        Some(FileTimeSpec::Absolute(ft)) => {
            let nanoseconds = ft.nanoseconds();
            assert_ne!(libc::c_long::from(nanoseconds), libc::UTIME_OMIT);
            assert_ne!(libc::c_long::from(nanoseconds), libc::UTIME_NOW);
            libc::timespec {
                tv_sec: ft.seconds(),
                tv_nsec: nanoseconds.try_into().unwrap(),
            }
        }
    })
}

pub(crate) fn set_file_times_impl(
    file: &fs::File,
    atime: Option<FileTimeSpec>,
    mtime: Option<FileTimeSpec>,
) -> io::Result<()> {
    let times = [to_timespec(atime)?, to_timespec(mtime)?];
    set_file_times_syscall(file, &times)
}

pub(crate) fn set_file_times_syscall(
    file: &fs::File,
    times: &[libc::timespec; 2],
) -> io::Result<()> {
    cvt_i32(unsafe { libc::futimens(file.as_raw_fd(), times.as_ptr()) }).map(|_| ())
}

pub(crate) fn set_times_nofollow_unchecked(
    start: &fs::File,
    path: &Path,
    atime: Option<FileTimeSpec>,
    mtime: Option<FileTimeSpec>,
) -> io::Result<()> {
    let times = [to_timespec(atime)?, to_timespec(mtime)?];

    let path = cstr(path)?;
    cvt_i32(unsafe {
        libc::utimensat(
            start.as_raw_fd(),
            path.as_ptr(),
            times.as_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
        )
    })
    .map(|_| ())
}
