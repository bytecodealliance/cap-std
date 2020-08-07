//! OS-specific functionality.
//!
//! This corresponds to [`std::os`].
//!
//! [`std::os`]: https://doc.rust-lang.org/std/os/

#[cfg(unix)]
pub mod unix;
