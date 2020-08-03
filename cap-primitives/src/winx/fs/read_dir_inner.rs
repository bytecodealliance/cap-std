use crate::fs::{DirEntryInner, Metadata};
use std::{ffi::OsStr, fmt, fs, io, path::Path};

pub(crate) struct ReadDirInner {}

impl ReadDirInner {
    pub(crate) fn read_dir(start: &fs::File, path: &Path) -> io::Result<Self> {
        todo!("ReadDirInner::read_dir")
    }

    pub(crate) fn read_dir_unchecked(start: &fs::File, path: &Path) -> io::Result<Self> {
        todo!("ReadDirInner::read_dir_unchecked")
    }

    pub(crate) fn metadata(&self, file_name: &OsStr) -> io::Result<Metadata> {
        todo!("ReadDirInner::metadata")
    }

    pub(crate) fn self_metadata(&self) -> io::Result<Metadata> {
        todo!("ReadDirInner::self_metadata")
    }
}

impl Iterator for ReadDirInner {
    type Item = io::Result<DirEntryInner>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("ReadDirInner::next")
    }
}

impl fmt::Debug for ReadDirInner {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO: Debug for ReadDirInner")
    }
}
