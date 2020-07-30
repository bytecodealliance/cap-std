//! Capability-oriented standard directories.

#![deny(missing_docs)]
#![doc(html_logo_url = "https://github.com/sunfishcode/cap-std/tree/main/media/cap-std.svg")]

use std::io;

mod project_dirs;
mod user_dirs;

pub use project_dirs::*;
pub use user_dirs::*;

#[cfg(not(windows))]
pub(crate) fn not_found() -> io::Error {
    io::Error::from_raw_os_error(libc::ENOENT)
}

#[cfg(windows)]
pub(crate) fn not_found() -> io::Error {
    io::Error::from_raw_os_error(winapi::shared::winerror::ERROR_PATH_NOT_FOUND as i32)
}
