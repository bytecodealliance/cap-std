//! A fully UTF-8 filesystem API modeled after [`cap_async_std::fs`].
//!
//! Where `cap_async_std::fs` would use `Path` and `PathBuf`, this `fs_utf8` module
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
//! If you don't want this, use the regular [`cap_async_std::fs`] module instead.
//!
//! [`cap_async_std::fs`]: ../fs/
//! [ARF strings]: https://crates.io/crates/arf-strings

mod dir;
mod dir_entry;
mod file;
mod read_dir;

pub use dir::Dir;
pub use dir_entry::DirEntry;
pub use file::File;
pub use read_dir::ReadDir;

// Re-export things from `cap_std::fs` that we can use as-is.
pub use crate::fs::{DirBuilder, FileType, Metadata, OpenOptions, Permissions};

fn from_utf8<P: AsRef<str>>(path: P) -> std::io::Result<async_std::path::PathBuf> {
    #[cfg(not(windows))]
    let path = {
        #[cfg(unix)]
        use std::{ffi::OsString, os::unix::ffi::OsStringExt};
        #[cfg(target_os = "wasi")]
        use std::{ffi::OsString, os::wasi::ffi::OsStringExt};

        let string = arf_strings::str_to_host(path.as_ref())?;
        OsString::from_vec(string.into_bytes())
    };

    #[cfg(windows)]
    let path = arf_strings::str_to_host(path.as_ref())?;

    Ok(path.into())
}

fn to_utf8<P: AsRef<async_std::path::Path>>(path: P) -> std::io::Result<String> {
    // For now, for WASI use the same logic as other OS's, but
    // in the future, the idea is we could avoid this.
    let osstr = path.as_ref().as_os_str();

    #[cfg(not(windows))]
    {
        arf_strings::host_os_str_to_str(osstr).map(std::borrow::Cow::into_owned)
    }

    #[cfg(windows)]
    {
        arf_strings::host_to_str(osstr)
    }
}
