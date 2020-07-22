//! OS-specific extensions.
//!
//! This corresponds to [`async_std::os`].
//!
//! [`async_std::os`]: https://docs.rs/async-std/latest/async_std/os/

#[cfg(unix)]
pub mod unix;
