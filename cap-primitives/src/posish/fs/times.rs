use crate::fs::SystemTimeSpec;
use posish::fs::{futimens, utimensat, AtFlags};
use std::{convert::TryInto, fs, io, path::Path, time::SystemTime};

pub(crate) fn to_timespec(ft: Option<SystemTimeSpec>) -> io::Result<libc::timespec> {
    Ok(match ft {
        None => libc::timespec {
            tv_sec: 0,
            tv_nsec: libc::UTIME_OMIT,
        },
        Some(SystemTimeSpec::SymbolicNow) => libc::timespec {
            tv_sec: 0,
            tv_nsec: libc::UTIME_NOW,
        },
        Some(SystemTimeSpec::Absolute(ft)) => {
            let duration = ft.duration_since(SystemTime::UNIX_EPOCH).unwrap();
            let nanoseconds = duration.subsec_nanos();
            assert_ne!(libc::c_long::from(nanoseconds), libc::UTIME_OMIT);
            assert_ne!(libc::c_long::from(nanoseconds), libc::UTIME_NOW);
            libc::timespec {
                tv_sec: duration
                    .as_secs()
                    .try_into()
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?,
                tv_nsec: nanoseconds.try_into().unwrap(),
            }
        }
    })
}

pub(crate) fn set_file_times_impl(
    file: &fs::File,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    let times = [to_timespec(atime)?, to_timespec(mtime)?];
    set_file_times_syscall(file, &times)
}

pub(crate) fn set_file_times_syscall(
    file: &fs::File,
    times: &[libc::timespec; 2],
) -> io::Result<()> {
    futimens(file, times)
}

pub(crate) fn set_times_nofollow_unchecked(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    let times = [to_timespec(atime)?, to_timespec(mtime)?];
    utimensat(start, path, &times, AtFlags::SYMLINK_NOFOLLOW)
}
