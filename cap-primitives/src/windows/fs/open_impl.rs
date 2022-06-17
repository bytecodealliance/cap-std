use crate::fs::{manually, OpenOptions};
use std::path::Path;
use std::{fs, io};
use windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND;

pub(crate) fn open_impl(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    // Windows reserves several special device paths. Disallow opening any
    // of them.
    if let Some(stem) = path.file_stem() {
        if let Some(stemstr) = stem.to_str() {
            match stemstr.to_uppercase().as_str() {
                "CON" | "PRN" | "AUX" | "NUL" | "COM0" | "COM1" | "COM2" | "COM3" | "COM4"
                | "COM5" | "COM6" | "COM7" | "COM8" | "COM9" | "LPT0" | "LPT1" | "LPT2"
                | "LPT3" | "LPT4" | "LPT5" | "LPT6" | "LPT7" | "LPT8" | "LPT9" => {
                    return Err(io::Error::from_raw_os_error(ERROR_FILE_NOT_FOUND as i32));
                }
                _ => {}
            }
        }
    }

    manually::open(start, path, options)
}
