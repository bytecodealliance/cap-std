//! Capability-based standard directories.

#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.ico"
)]

use std::io;

mod project_dirs;
mod user_dirs;

#[doc(hidden)]
pub use cap_std::ambient_authority_known_at_compile_time;
pub use cap_std::{ambient_authority, AmbientAuthority};
pub use project_dirs::ProjectDirs;
pub use user_dirs::UserDirs;

#[cfg(not(windows))]
pub(crate) fn not_found() -> io::Error {
    rustix::io::Errno::NOENT.into()
}

#[cfg(windows)]
pub(crate) fn not_found() -> io::Error {
    io::Error::from_raw_os_error(winapi::shared::winerror::ERROR_PATH_NOT_FOUND as i32)
}
