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
//! On WASI, use of this library more closely reflects the underlying
//! system API, so it avoids the absolute-path compatibility layers.
//!
//! [`std`]: https://doc.rust-lang.org/std/index.html

// TODO: When more things are implemented, remove these.
#![allow(unused_variables, dead_code)]

pub mod fs;
pub mod net;

// TODO: ChildStderr, ChildStdout, Stdin, StdinLock

// TODO: ChildStdin, Stdout, Stderr
