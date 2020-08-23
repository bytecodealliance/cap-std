//! The `yanix` module contains code specific to the Unix-like platforms
//! supported by the `yanix` crate.

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod darwin;
#[cfg(target_os = "linux")]
mod linux;
#[macro_use]
mod weak;

pub(crate) mod fs;
