//! The `yanix` module contains code specific to the Unix-like platforms
//! supported by the `yanix` crate.

#[cfg(target_os = "linux")]
mod linux;

pub(crate) mod fs;
