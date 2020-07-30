//! Capability-oriented standard directories.

#![deny(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/sunfishcode/cap-std/main/media/cap-std.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/sunfishcode/cap-std/main/media/cap-std.ico")]

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
