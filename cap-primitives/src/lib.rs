//! Capability-oriented primitives.

#![deny(missing_docs)]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(
    all(windows, feature = "windows_by_handle"),
    feature(windows_by_handle)
)]
#![cfg_attr(
    all(windows, feature = "windows_file_type_ext"),
    feature(windows_file_type_ext)
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/sunfishcode/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sunfishcode/cap-std/main/media/cap-std.ico"
)]

cfg_if::cfg_if! {
    if #[cfg(any(unix, target_os = "fuchsia"))] {
        mod yanix;
    } else if #[cfg(windows)] {
        mod winx;
    } else if #[cfg(not(target_os = "wasi"))] {
        compile_error!("cap-std doesn't compile for this platform yet");
    }
}

pub mod fs;
pub mod net;
