//! A capability-based API modeled after `std`.
//!
//! This corresponds to [`std`].
//!
//! Capability-based security represents access to external resources as
//! objects which can be passed around between different parts of a
//! program.
//!
//! TODO: say more
//!
//! TODO: should try_clone methods require a capability?
//!
//! On WASI, use of this library more closely reflects the underlying
//! system API, so it avoids the absolute-path compatibility layers.
//!
//! [`std`]: https://doc.rust-lang.org/std/index.html

#![allow(unused_variables, dead_code)] // TODO: When more things are implemented, remove these.
#![deny(missing_docs)]

pub mod fs;
pub mod net;
pub mod os;
