//! A capability-based API modeled after `async_std`.
//!
//! This corresponds to [`async_std`].
//!
//! Capability-based APIs represent access to external resources as
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

#![allow(dead_code, unused_variables)] // TODO: When more things are implemented, remove these.
#![deny(missing_docs)]

mod sys;

pub mod fs;
#[cfg(feature = "fs_utf8")]
pub mod fs_utf8;
pub mod net;
pub mod os;
