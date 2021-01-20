use crate::time::{Duration, Instant};
use std::time;

/// A reference to a monotonically nondecreasing clock.
///
/// This does not directly correspond to anything in `std`, however its methods
/// correspond to [methods in `std::time::Instant`].
///
/// [methods in `std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
pub struct MonotonicClock(());

impl MonotonicClock {
    /// Constructs a new instance of `Self`.
    ///
    /// # Safety
    ///
    /// This is unsafe because access to clocks is an ambient authority.
    #[inline]
    pub const unsafe fn new() -> Self {
        Self(())
    }

    /// Returns an instant corresponding to "now".
    ///
    /// This corresponds to [`Instant::now`].
    ///
    /// [`Instant::now`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.now
    #[inline]
    pub fn now(&self) -> Instant {
        Instant::from_std(time::Instant::now())
    }

    /// Returns the amount of time elapsed since this instant was created.
    ///
    /// This corresponds to [`Instant::elapsed`].
    ///
    /// [`Instant::elapsed`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.elapsed
    #[inline]
    pub fn elapsed(&self, instant: Instant) -> Duration {
        instant.std.elapsed()
    }
}
