use crate::{fs::SystemTimeSpec, time::SystemClock};
use posish::fs::{futimens, utimensat, AtFlags};
use std::{convert::TryInto, fs, io, path::Path};

pub(crate) fn to_timespec(ft: Option<SystemTimeSpec>) -> io::Result<libc::timespec> {
    Ok(match ft {
        None => libc::timespec {
            tv_sec: 0,
            tv_nsec: libc::UTIME_OMIT.into(),
        },
        Some(SystemTimeSpec::SymbolicNow) => libc::timespec {
            tv_sec: 0,
            tv_nsec: libc::UTIME_NOW.into(),
        },
        Some(SystemTimeSpec::Absolute(ft)) => {
            let duration = ft.duration_since(SystemClock::UNIX_EPOCH).unwrap();
            let nanoseconds = duration.subsec_nanos();
            assert_ne!(i64::from(nanoseconds), i64::from(libc::UTIME_OMIT));
            assert_ne!(i64::from(nanoseconds), i64::from(libc::UTIME_NOW));
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
