//! A capability-oriented filesystem API modeled after `std::fs`.
//!
//! This corresponds to [`std::fs`].
//!
//! Instead of [`std::fs`'s free functions] and [`std::fs::File`]'s
//! constructors which operate on bare paths, this crate has methods on [`Dir`]
//! which operate on paths which must be relative to the directory.
//!
//! Where `std` says "the filesystem", this API says "a filesystem", as
//! it doesn't assume that there's a single global filesystem namespace.
//!
//! Since all functions which expose raw file descriptors are `unsafe`,
//! I/O handles in this API are unforgeable (unsafe code notwithstanding).
//! This combined with a lack of absolute paths provides a natural
//! capability-oriented interface.
//!
//! This crate uses the existing `std::path::Path` rather than having its own
//! path type, however while `std::path::Path` is mostly just a pure datatype,
//! it includes aliases for several `std::fs` functions. To preserve the
//! capability-oriented interface, avoid using `std::path::Path`'s
//! `canonicalize`, `read_link`, `read_dir`, `metadata`, and `symlink_metadata`
//! functions.
//!
//! [`std::fs`]: https://doc.rust-lang.org/std/fs/
//! [`std::fs`'s free functions]: https://doc.rust-lang.org/std/fs/#functions
//! [`std::fs::File`]: https://doc.rust-lang.org/std/fs/struct.File.html
//! [`Dir`]: struct.Dir.html

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

// Re-export things from `std` that we can use as-is.
#[cfg(target_os = "wasi")]
pub use std::fs::{DirBuilder, FileType, Metadata, OpenOptions, Permissions};
