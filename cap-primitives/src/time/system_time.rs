use crate::time::{Duration, SystemTimeError};
use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
    time,
};

/// A measurement of the system clock, useful for talking to external entities
/// like the file system or other processes.
///
/// This corresponds to [`std::time::SystemTime`].
///
/// Note that this `SystemTime` has no `now`, `elapsed` methods. To obtain the
/// current time or measure the duration to the current time, you must first
/// obtain a [`SystemClock`], and then call [`SystemClock::now`] or
/// [`SystemClock::elapsed`] instead. The `UNIX_EPOCH` constant is at
/// [`SystemClock::UNIX_EPOCH`].
///
/// [`std::time::SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
/// [`SystemClock`]: struct.SystemClock.html
/// [`SystemClock::now`]: struct.SystemClock.html#method.now
/// [`SystemClock::elapsed`]: struct.SystemClock.html#method.elapsed
/// [`SystemClock::UNIX_EPOCH`]: struct.SystemClock.html#associatedconstant.UNIX_EPOCH
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SystemTime {
    pub(crate) std: time::SystemTime,
}

impl SystemTime {
    /// Constructs a new instance of `Self` from the given `std::time::SystemTime`.
    #[inline]
    pub const fn from_std(std: time::SystemTime) -> Self {
        Self { std }
    }

    /// Constructs a new instance of `std::time::SystemTime` from the given `Self`.
    #[inline]
    pub const fn into_std(self) -> time::SystemTime {
        self.std
    }

    /// Returns the amount of time elapsed from another instant to this one.
    ///
    /// This corresponds to [`std::time::SystemTime::duration_since`].
    ///
    /// [`std::time::SystemTime::duration_since`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.duration_since
    #[inline]
    pub fn duration_since(&self, earlier: Self) -> Result<Duration, SystemTimeError> {
        self.std.duration_since(earlier.std)
    }

    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be represented as
    /// `SystemTime` (which means it's inside the bounds of the underlying data structure), `None`
    /// otherwise.
    ///
    /// This corresponds to [`std::time::SystemTime::checked_add`].
    ///
    /// [`std::time::SystemTime::checked_add`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.checked_add
    #[inline]
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        self.std.checked_add(duration).map(Self::from_std)
    }

    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` can be represented as
    /// `SystemTime` (which means it's inside the bounds of the underlying data structure), `None`
    /// otherwise.
    ///
    /// This corresponds to [`std::time::SystemTime::checked_sub`].
    ///
    /// [`std::time::SystemTime::checked_sub`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.checked_sub
    #[inline]
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        self.std.checked_sub(duration).map(Self::from_std)
    }
}

impl Add<Duration> for SystemTime {
    type Output = Self;

    /// # Panics
    ///
    /// This function may panic if the resulting point in time cannot be represented by the
    /// underlying data structure. See [`SystemTime::checked_add`] for a version without panic.
    fn add(self, dur: Duration) -> Self {
        self.checked_add(dur)
            .expect("overflow when adding duration to instant")
    }
}

impl AddAssign<Duration> for SystemTime {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

impl Sub<Duration> for SystemTime {
    type Output = Self;

    fn sub(self, dur: Duration) -> Self {
        self.checked_sub(dur)
            .expect("overflow when subtracting duration from instant")
    }
}

impl SubAssign<Duration> for SystemTime {
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}

impl fmt::Debug for SystemTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
