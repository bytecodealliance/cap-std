//! Capability-oriented primitives.

#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(all(windows, windows_by_handle), feature(windows_by_handle))]
#![cfg_attr(all(windows, windows_file_type_ext), feature(windows_file_type_ext))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.ico"
)]

#[cfg(not(windows))]
mod posish;
#[cfg(windows)]
mod windows;

pub mod fs;
pub mod net;
pub mod time;

pub use ambient_authority::{ambient_authority, AmbientAuthority};
