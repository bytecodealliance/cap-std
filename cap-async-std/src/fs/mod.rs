//! A capability-based filesystem API modeled after `async_std::fs`.
//!
//! This corresponds to [`async_std::fs`].
//!
//! Instead of [`async_std::fs`'s free functions] which operate on paths, this
//! crate has methods on [`Dir`] which operate on paths which must be
//! relative to the directory.
//!
//! Where `async_std` says "the filesystem", this API says "a filesystem", as
//! it doesn't assume that there's a single global filesystem namespace.
//!
//! Since all functions which expose raw file descriptors are `unsafe`,
//! I/O handles in this API are unforgeable (unsafe code notwithstanding).
//! This combined a lack of absolute paths provides a natural
//! capability-oriented interface.
//!
//! [`async_std::fs`]: https://docs.rs/async-std/latest/async_std/fs/index.html
//! [`async_std::fs`'s free functions]: https://docs.rs/async-std/latest/async_std/fs/index.html#functions
//! [`Dir`]: struct.Dir.html

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

// Re-export things from `async_std::fs` that we can use as-is.
pub use async_std::fs::{FileType, Metadata, Permissions};

// Re-export things from `cap_primitives` that we can use as-is.
pub use cap_primitives::fs::OpenOptions;
