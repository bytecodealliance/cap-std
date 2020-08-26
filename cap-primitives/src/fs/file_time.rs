pub use filetime::FileTime;

/// A value for specifying a time.
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum FileTimeSpec {
    /// A value which always represents the current time, in symbolic
    /// form, so that even as time elapses, it continues to represent
    /// the current time.
    SymbolicNow,

    /// An abslute time value.
    Absolute(FileTime),
}

impl From<FileTime> for FileTimeSpec {
    fn from(time: FileTime) -> Self {
        Self::Absolute(time)
    }
}
