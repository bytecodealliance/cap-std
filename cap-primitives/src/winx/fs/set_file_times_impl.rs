use crate::fs::{FileTime, FileTimeSpec};
use std::{fs, io};
use winapi::shared::winerror::ERROR_NOT_SUPPORTED;

pub(crate) fn set_file_times_impl(
    file: &fs::File,
    atime: Option<FileTimeSpec>,
    mtime: Option<FileTimeSpec>,
) -> io::Result<()> {
    let mut now = None;

    let atime = match atime {
        None => None,
        Some(FileTimeSpec::SymbolicNow) => {
            let right_now = filetime::FileTime::now();
            now = Some(right_now);
            Some(right_now)
        }
        Some(FileTimeSpec::Absolute(time)) => {
            // On Windows, a zero time is silently ignored, so issue an error instead.
            if time == FileTime::zero() {
                return Err(io::Error::from_raw_os_error(ERROR_NOT_SUPPORTED as i32));
            }
            Some(time)
        }
    };

    let mtime = match mtime {
        None => None,
        Some(FileTimeSpec::SymbolicNow) => {
            if let Some(prev_now) = now {
                Some(prev_now)
            } else {
                Some(filetime::FileTime::now())
            }
        }
        Some(FileTimeSpec::Absolute(time)) => {
            // On Windows, a zero time is silently ignored, so issue an error instead.
            if time == FileTime::zero() {
                return Err(io::Error::from_raw_os_error(ERROR_NOT_SUPPORTED as i32));
            }
            Some(time)
        }
    };

    filetime::set_file_handle_times(file, atime, mtime)
}
