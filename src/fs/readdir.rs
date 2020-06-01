use crate::fs::DirEntry;
use std::{fs, io, path::PathBuf};

/// Iterator over the entries in a directory.
///
/// This corresponds to [`std::fs::ReadDir`].
///
/// Unlike `std::fs::ReadDir`, this `ReadDir`s `Iterator` implementation
/// provides a `PathBuf` giving the name of the directory entry in addition
/// to the `DirEntry`.
///
/// [`std::fs::ReadDir`]: https://doc.rust-lang.org/std/fs/struct.ReadDir.html
pub struct ReadDir {
    file: fs::File,
}

impl ReadDir {
    /// Constructs a new instance of `Self` from the given `std::fs::File`.
    pub fn from_ambient(file: fs::File) -> Self {
        Self { file }
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<(PathBuf, DirEntry)>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!("ReadDir::next");
    }
}

// TODO: impl Debug for ReadDir? But don't expose ReadDir's path...
