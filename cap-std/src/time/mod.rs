//! A capability-oriented clock API modeled after `std::time`.
//!
//! This corresponds to [`std::time`].
//!
//! Instead of [`std::time`]'s methods which return the current time, this crate
//! has methods on [`SystemClock`] and [`MonotonicClock`].
//!
//! [`std::time`]: https://doc.rust-lang.org/std/time/
//! [`SystemClock`]: struct.SystemClock.html
//! [`MonotonicClock`]: struct.MonotonicClock.html

pub use cap_primitives::time::{
    Duration, Instant, MonotonicClock, SystemClock, SystemTime, SystemTimeError,
};
