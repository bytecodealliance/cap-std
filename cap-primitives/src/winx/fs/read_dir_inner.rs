use super::get_path::concatenate_or_return_absolute;
use crate::fs::{open_dir, DirEntryInner, Metadata};
use std::{
    fmt, fs, io,
    path::{Component, Path},
};

pub(crate) struct ReadDirInner {
    std: fs::ReadDir,
}

impl ReadDirInner {
    pub(crate) fn new(start: &fs::File, path: &Path) -> io::Result<Self> {
        let dir = open_dir(start, path)?;
        Self::new_unchecked(&dir, Component::CurDir.as_os_str().as_ref())
    }

    pub(crate) fn new_unchecked(start: &fs::File, path: &Path) -> io::Result<Self> {
        let full_path = concatenate_or_return_absolute(start, path)?;
        Ok(Self {
            std: fs::read_dir(full_path)?,
        })
    }

    pub(super) fn from_std(std: fs::ReadDir) -> Self {
        Self { std }
    }
}

impl Iterator for ReadDirInner {
    type Item = io::Result<DirEntryInner>;

    fn next(&mut self) -> Option<Self::Item> {
        self.std
            .next()
            .map(|result| result.map(|std| DirEntryInner { std }))
    }
}

impl fmt::Debug for ReadDirInner {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO: Debug for ReadDirInner")
    }
}
