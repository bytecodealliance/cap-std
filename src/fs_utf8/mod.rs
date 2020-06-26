//! A fully UTF-8 filesystem API modeled after `cap_std::fs`.
//!
//! It uses [ARF strings] to encode non-UTF-8-encodable filenames.
//!
//! [`cap_std::fs`]: ../fs/index.html
//! [ARF strings]: https://crates.io/crates/arf-strings

mod dir;
mod dir_builder;
mod dir_entry;
mod file;
mod read_dir;

pub use dir::*;
pub use dir_builder::*;
pub use dir_entry::*;
pub use file::*;
pub use read_dir::*;

// Re-export things from `std::fs` that we can use as-is.
pub use std::fs::{FileType, Metadata, Permissions};

// Re-export things from `cap_std::fs` that we can use as-is.
pub use crate::fs::OpenOptions;

fn from_utf8<P: AsRef<str>>(path: P) -> std::io::Result<std::path::PathBuf> {
    // For now, for WASI use the same logic as other OS's, but
    // in the future, the idea is we could avoid this.
    let string = arf_strings::PosixString::from_path_str(path.as_ref())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid path string"))?;
    let bytes = string.as_cstr().to_bytes();

    #[cfg(any(unix, target_os = "redox", target_os = "wasi"))]
    let path = {
        use std::{ffi::OsStr, os::unix::ffi::OsStrExt};
        OsStr::from_bytes(bytes).to_owned()
    };

    #[cfg(windows)]
    let path = {
        use std::{ffi::OsString, os::windows::prelude::*};
        OsString::from_wide(bytes)
    };

    Ok(path.into())
}

fn to_utf8<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    // For now, for WASI use the same logic as other OS's, but
    // in the future, the idea is we could avoid this.
    let osstr = path.as_ref().as_os_str();

    #[cfg(any(unix, target_os = "redox", target_os = "wasi"))]
    let cstr = {
        use std::{ffi::CString, os::unix::ffi::OsStrExt};
        CString::new(osstr.as_bytes())?
    };

    #[cfg(windows)]
    let cstr = { panic!("windows not yet implemented") };

    Ok(arf_strings::WasiString::from_maybe_nonutf8_cstr(&cstr)
        .as_str()
        .to_owned())
}
