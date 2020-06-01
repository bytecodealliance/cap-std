use crate::fs::DirEntry;
use std::{fs, io};

/// Iterator over the entries in a directory.
///
/// This corresponds to [`std::fs::ReadDir`].
///
/// [`std::fs::ReadDir`]: https://doc.rust-lang.org/std/fs/struct.ReadDir.html
pub struct ReadDir {
    std: fs::ReadDir,
}

impl ReadDir {
    /// Constructs a new instance of `Self` from the given `std::fs::File`.
    #[inline]
    pub fn from_ambient(std: fs::ReadDir) -> Self {
        Self { std }
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.std
            .next()
            .map(|result| result.map(DirEntry::from_ambient))
    }
}

// TODO: impl Debug for ReadDir? But don't expose ReadDir's path...
