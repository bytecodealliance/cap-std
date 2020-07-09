//! A fully UTF-8 filesystem API modeled after [`cap_std::fs`].
//!
//! Where `cap_std::fs` would use `Path` and `PathBuf`, this `fs_utf8` module
//! uses `str` and `String`, meaning that all paths are valid UTF-8.
//!
//! But wait, POSIX doesn't require filenames to be UTF-8! What happens if
//! there's a file with a non-UTF-8 name? To address this, this module uses
//! [ARF strings] to encode non-UTF-8-encodable filenames as UTF-8. See the
//! link for details, but the big picture is that all possible byte sequences
//! are losslessly representable. The easy case of a file with a valid UTF-8
//! name is easy, and the tricky case of a valid with an invalid UTF-8 name
//! is possible -- it may take more work to handle properly, especially if
//! you want to do interesting path manipulation, but it is possible.
//!
//! TODO: This whole scheme is still under development.
//!
//! If you don't want this, use the regular [`cap_std::fs`] module instead.
//!
//! [`cap_std::fs`]: ../fs/
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

// Re-export things from `cap_std::fs` that we can use as-is.
pub use crate::fs::{FileType, Metadata, OpenOptions, Permissions};

fn from_utf8<P: AsRef<str>>(path: P) -> std::io::Result<std::path::PathBuf> {
    // For now, for WASI use the same logic as other OS's, but
    // in the future, the idea is we could avoid this.
    let string = arf_strings::PosixString::from_path_str(path.as_ref())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid path string"))?;

    #[cfg(any(unix, target_os = "redox", target_os = "wasi"))]
    let path = {
        use std::{ffi::OsStr, os::unix::ffi::OsStrExt};
        let bytes = string.as_cstr().to_bytes();
        OsStr::from_bytes(bytes).to_owned()
    };

    #[cfg(windows)]
    let path = {
        use std::{ffi::OsString, os::windows::ffi::OsStringExt};
        let utf8 = string.as_cstr().to_string_lossy(); // This is OK since we went through arf_strings first.
        let utf16: Vec<_> = utf8.encode_utf16().collect();
        OsString::from_wide(&utf16)
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
    let cstr = {
        use std::{ffi::CString, os::windows::ffi::OsStrExt};
        let utf16: Vec<_> = osstr.encode_wide().collect();
        let str = String::from_utf16(&utf16).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid path string")
        })?;
        CString::new(str)?
    };

    Ok(arf_strings::WasiString::from_maybe_nonutf8_cstr(&cstr)
        .as_str()
        .to_owned())
}
