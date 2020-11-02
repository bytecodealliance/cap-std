//! A capability-oriented filesystem API modeled after `async_std::fs`.
//!
//! This corresponds to [`async_std::fs`].
//!
//! Instead of [`async_std::fs`'s free functions] and [`async_std::fs::File`]'s
//! constructors which operate on bare paths, this crate has methods on [`Dir`]
//! which operate on paths which must be relative to the directory.
//!
//! Where `async_std` says "the filesystem", this API says "a filesystem", as
//! it doesn't assume that there's a single global filesystem namespace.
//!
//! Since all functions which expose raw file descriptors are `unsafe`,
//! I/O handles in this API are unforgeable (unsafe code notwithstanding).
//! This combined with a lack of absolute paths provides a natural
//! capability-oriented interface.
//!
//! This crate uses the existing `async_std::path::Path` rather than having its own
//! path type, however while `async_std::path::Path` is mostly just a pure datatype,
//! it includes aliases for several `async_std::fs` functions. To preserve the
//! capability-oriented interface, avoid using `async_std::path::Path`'s
//! `canonicalize`, `read_link`, `read_dir`, `metadata`, and `symlink_metadata`
//! functions.
//!
//! [`async_std::fs`]: https://docs.rs/async-std/latest/async_std/fs/
//! [`async_std::fs`'s free functions]: https://docs.rs/async-std/latest/async_std/fs/#functions
//! [`async_std::fs::File`]: https://docs.rs/async-std/latest/async_std/fs/struct.File.html
//! [`Dir`]: struct.Dir.html

mod dir;
mod dir_entry;
mod file;
mod read_dir;

pub use dir::*;
pub use dir_entry::*;
pub use file::*;
pub use read_dir::*;

// Re-export things from `cap_primitives` that we can use as-is.
#[cfg(not(target_os = "wasi"))]
pub use cap_primitives::fs::{DirBuilder, FileType, Metadata, OpenOptions, Permissions};

// Re-export things from `async_std` that we can use as-is.
#[cfg(target_os = "wasi")]
pub use async_std::fs::{DirBuilder, FileType, Metadata, OpenOptions, Permissions};

/// Utility for returning an `async_std::fs::File` as a `std::fs::File`
/// for synchronous operations.
///
/// # Safety
///
/// Callers must avoid using the `async_std::fs::File` while the
/// resulting `std::fs::File` is live, and must ensure that the resulting
/// `std::fs::File` doesn't outlive the `async_std::fs::File`.
#[inline]
pub(crate) unsafe fn as_sync(
    async_file: &async_std::fs::File,
) -> std::mem::ManuallyDrop<std::fs::File> {
    _as_sync(async_file)
}

#[cfg(not(windows))]
unsafe fn _as_sync(async_file: &async_std::fs::File) -> std::mem::ManuallyDrop<std::fs::File> {
    use std::os::unix::io::{AsRawFd, FromRawFd};
    std::mem::ManuallyDrop::new(std::fs::File::from_raw_fd(async_file.as_raw_fd()))
}

#[cfg(windows)]
unsafe fn _as_sync(async_file: &async_std::fs::File) -> std::mem::ManuallyDrop<std::fs::File> {
    use std::os::windows::io::{AsRawHandle, FromRawHandle};
    std::mem::ManuallyDrop::new(std::fs::File::from_raw_handle(async_file.as_raw_handle()))
}

/// Utility for converting an `async_std::fs::File` into a `std::fs::File`
/// for synchronous operations.
#[inline]
pub(crate) fn into_sync(async_file: async_std::fs::File) -> std::fs::File {
    _into_sync(async_file)
}

#[cfg(not(windows))]
fn _into_sync(async_file: async_std::fs::File) -> std::fs::File {
    use std::os::unix::io::{AsRawFd, FromRawFd};
    unsafe { std::fs::File::from_raw_fd(async_file.as_raw_fd()) }
}

#[cfg(windows)]
fn _into_sync(async_file: async_std::fs::File) -> std::fs::File {
    use std::os::windows::io::{AsRawHandle, FromRawHandle};
    unsafe { std::fs::File::from_raw_handle(async_file.as_raw_handle()) }
}
