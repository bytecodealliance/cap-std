//! A capability-oriented API modeled after `async_std`.
//!
//! This corresponds to [`async_std`].
//!
//! Capability-oriented APIs represent access to external resources as
//! objects which can be passed around between different parts of a
//! program.
//!
//! Two notable features are the [`Dir`] and [`Catalog`] types:
//!  - `Dir` represents an open directory in a filesystem. Instead of
//!    opening files by absolute paths or paths relative to the current
//!    working directory, files are opened via paths relative to a
//!    `Dir`. The concepts of a process-wide "current working directory"
//!    and a single global filesystem namespace are de-emphasized.
//!  - `Catalog` represents a set of network addresses. Instead of
//!    allowing applications to request access to any address and then
//!    applying process-wide filtering rules, filtering rules are
//!    built into catalogs which may be passed through the program.
//!
//! On WASI, use of this library closely reflects the underlying system
//! API, so it avoids compatibility layers.
//!
//! [`async_std`]: https://docs.rs/async-std/latest/async_std/
//! [`Dir`]: fs/struct.Dir.html
//! [`Catalog`]: net/struct.Catalog.html

#![deny(missing_docs)]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
// async_std doesn't have "can_vector".
// async_std doesn't have "read_initializer".
// async_std doesn't have "seek_convenience".
// async_std doesn't have "with_options".
// async_std doesn't have "write_all_vectored".
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.ico"
)]

pub mod fs;
#[cfg(feature = "fs_utf8")]
pub mod fs_utf8;
#[cfg(not(target_os = "wasi"))] // Disable `net` on WASI until it has networking support.
pub mod net;
pub mod os;
pub mod time;
