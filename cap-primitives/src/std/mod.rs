//! Utilities common to all platforms which use `std`, the standard library.
//!
//! TODO: WASI uses the standard library and does not use this module. And
//! this module's name, `std`, is confusing anyway. We should rename it.
//! `emulated`? `userspace`? `manual`?

pub(crate) mod fs;
pub(crate) mod net;
