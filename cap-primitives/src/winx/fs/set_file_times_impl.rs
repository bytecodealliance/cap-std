use crate::fs::SystemTimeSpec;
use std::{
    fs, io,
    os::windows::io::AsRawHandle,
    ptr,
    time::{Duration, SystemTime},
};
use winapi::{
    shared::{
        minwindef::{DWORD, FILETIME},
        winerror::ERROR_NOT_SUPPORTED,
    },
    um::fileapi::SetFileTime,
};

pub(crate) fn set_file_times_impl(
    file: &fs::File,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    let mut now = None;

    let atime = match atime {
        None => None,
        Some(SystemTimeSpec::SymbolicNow) => {
            let right_now = SystemTime::now();
            now = Some(right_now);
            Some(right_now)
        }
        Some(SystemTimeSpec::Absolute(time)) => Some(time),
    };
    let mtime = match mtime {
        None => None,
        Some(SystemTimeSpec::SymbolicNow) => {
            if let Some(prev_now) = now {
                Some(prev_now)
            } else {
                Some(SystemTime::now())
            }
        }
        Some(SystemTimeSpec::Absolute(time)) => Some(time),
    };

    let atime = atime.map(to_filetime).transpose()?;
    let mtime = mtime.map(to_filetime).transpose()?;
    if unsafe {
        SetFileTime(
            file.as_raw_handle(),
            ptr::null(),
            atime
                .as_ref()
                .map(|p| p as *const FILETIME)
                .unwrap_or(ptr::null()),
            mtime
                .as_ref()
                .map(|p| p as *const FILETIME)
                .unwrap_or(ptr::null()),
        )
    } != 0
    {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

fn to_filetime(ft: SystemTime) -> io::Result<FILETIME> {
    // To convert a `SystemTime` to absolute seconds and nanoseconds, we need
    // a reference point. The `UNIX_EPOCH` is the only reference point provided
    // by the standard library, so use that.
    let ft = ft
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    // Windows' time stamps are relative to January 1, 1601 so adjust by the
    // difference between that and the Unix epoch.
    let ft = ft + Duration::from_secs(11_644_473_600);

    let intervals = ft.as_secs() * (1_000_000_000 / 100) + u64::from(ft.subsec_nanos() / 100);

    // On Windows, a zero time is silently ignored, so issue an error instead.
    if intervals == 0 {
        return Err(io::Error::from_raw_os_error(ERROR_NOT_SUPPORTED as i32));
    }

    Ok(FILETIME {
        dwLowDateTime: intervals as DWORD,
        dwHighDateTime: (intervals >> 32) as DWORD,
    })
}
