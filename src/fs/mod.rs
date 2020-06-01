//! A capability-based filesystem API modeled after `std::fs`.
//!
//! This corresponds to [`std::fs`].
//!
//! Instead of [`std::fs`'s free functions] which operate on paths, this
//! crate has methods on [`Dir`] which operate on paths which must be
//! relative to the directory.
//!
//! Since all functions which expose raw file descriptors are `unsafe`,
//! I/O handles in this API are unforgeable (unsafe code notwithstanding).
//! This combined a absolute paths provides a natural capability-oriented
//! interface.
//!
//! [`std::fs`]: https://doc.rust-lang.org/std/fs/index.html
//! [`std::fs`'s free functions]: https://doc.rust-lang.org/std/fs/index.html#functions
//! [`Dir`]: struct.Dir.html

mod dir;
mod dir_builder;
mod dir_entry;
mod file;
mod file_type;
mod open_options;
mod readdir;

pub use dir::*;
pub use dir_builder::*;
pub use dir_entry::*;
pub use file::*;
pub use file_type::*;
pub use open_options::*;
pub use readdir::*;

// Re-export things from std::fs that we can use as-is.
pub use std::fs::{Metadata, Permissions};
