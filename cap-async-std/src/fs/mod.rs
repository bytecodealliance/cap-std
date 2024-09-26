//! A capability-based filesystem API modeled after [`async_std::fs`].
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
//! Since all functions which expose raw file descriptors are `unsafe`, I/O
//! handles in this API are unforgeable (unsafe code notwithstanding). This
//! combined with a lack of absolute paths provides a natural capability-based
//! interface.
//!
//! This crate uses the existing `async_std::path::Path` rather than having its
//! own path type, however while `async_std::path::Path` is mostly just a pure
//! datatype, it includes aliases for several `async_std::fs` functions. To
//! preserve the capability-based interface, avoid using
//! `async_std::path::Path`'s `canonicalize`, `read_link`, `read_dir`,
//! `metadata`, and `symlink_metadata` functions.
//!
//! [`async_std::fs`'s free functions]: https://docs.rs/async-std/latest/async_std/fs/#functions

mod dir;
mod dir_entry;
mod file;
mod read_dir;

pub use dir::Dir;
pub use dir_entry::DirEntry;
pub use file::File;
pub use read_dir::ReadDir;

// Re-export things from `cap_primitives` that we can use as-is.
#[cfg(not(target_os = "wasi"))]
pub use cap_primitives::fs::{DirBuilder, FileType, Metadata, OpenOptions, Permissions};

// Re-export conditional types from `cap_primitives`.
#[cfg(any(unix, target_os = "vxworks", all(windows, windows_file_type_ext)))]
pub use cap_primitives::fs::FileTypeExt;
#[cfg(unix)]
pub use cap_primitives::fs::{DirBuilderExt, PermissionsExt};
pub use cap_primitives::fs::{FileExt, MetadataExt, OpenOptionsExt};

// Re-export things from `async_std` that we can use as-is.
#[cfg(target_os = "wasi")]
pub use async_std::fs::{DirBuilder, FileType, Metadata, OpenOptions, Permissions};
