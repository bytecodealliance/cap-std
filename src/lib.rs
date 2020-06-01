//! A capability-based API modeled after `std`.
//!
//! This corresponds to [`std`].
//!
//! Capability-based APIs represent access to external resources as
//! objects which can be passed around between different parts of a
//! program.
//!
//! TODO: say more
//!
//! TODO: should try_clone methods require a capability?
//! TODO: `std::process::Command`
//!
//! TODO: Rust's `Path` has several ambient-authority methods: `metadata`,
//! `read_link`, `read_dir`, `symlink_metadata`, `canonicalize`. Is it
//! worth having our own version of `Path` just to exclude those? Such a
//! thing could also exclude absolute paths.
//!
//! On WASI, use of this library more closely reflects the underlying
//! system API, so it avoids the absolute-path compatibility layers.
//!
//! [`std`]: https://doc.rust-lang.org/std/index.html

#![allow(dead_code, unused_variables)] // TODO: When more things are implemented, remove these.
#![deny(missing_docs)]

pub mod fs;
pub mod net;
pub mod os;
