use crate::time::{Duration, SystemTime, SystemTimeError};
use std::time;

/// A reference to a system clock, useful for talking to external entities like
/// the file system or other processes.
///
/// This does not directly correspond to anything in `std`, however its methods
/// correspond to [methods in `std::time::SystemTime`].
///
/// [methods in `std::time::SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
pub struct SystemClock(());

impl SystemClock {
    /// An anchor in time which can be used to create new `SystemTime`
    /// instances or learn about where in time a `SystemTime` lies.
    ///
    /// This corresponds to [`std::time::SystemTime::UNIX_EPOCH`].
    pub const UNIX_EPOCH: SystemTime = SystemTime {
        std: time::SystemTime::UNIX_EPOCH,
    };

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
    /// This corresponds to [`std::time::SystemTime::now`].
    #[inline]
    pub fn now(&self) -> SystemTime {
        SystemTime::from_std(time::SystemTime::now())
    }

    /// Returns the amount of time elapsed since this instant was created.
    ///
    /// This corresponds to [`std::time::SystemTime::elapsed`].
    #[inline]
    pub fn elapsed(&self, system_time: SystemTime) -> Result<Duration, SystemTimeError> {
        system_time.std.elapsed()
    }
}
