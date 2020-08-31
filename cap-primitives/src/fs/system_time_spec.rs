use std::time::SystemTime;

/// A value for specifying a time.
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum SystemTimeSpec {
    /// A value which always represents the current time, in symbolic
    /// form, so that even as time elapses, it continues to represent
    /// the current time.
    SymbolicNow,

    /// An abslute time value.
    Absolute(SystemTime),
}

impl From<SystemTime> for SystemTimeSpec {
    fn from(time: SystemTime) -> Self {
        Self::Absolute(time)
    }
}
