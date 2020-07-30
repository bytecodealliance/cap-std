use crate::fs::{FileType, Metadata, OpenOptions, ReadDirInner};
use std::{ffi::OsString, fmt, fs, io};

pub(crate) struct DirEntryInner {
    pub(crate) read_dir: ReadDirInner,
}

impl DirEntryInner {
    #[inline]
    pub fn open(&self, options: &OpenOptions) -> io::Result<fs::File> {
        todo!("DirEntryInner::open")
    }

    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        todo!("DirEntryInner::metadata")
    }

    #[inline]
    pub fn remove_file(&self) -> io::Result<()> {
        todo!("DirEntryInner::remove_file")
    }

    #[inline]
    pub fn remove_dir(&self) -> io::Result<()> {
        todo!("DirEntryInner::remove_dir")
    }

    #[inline]
    pub fn file_type(&self) -> FileType {
        todo!("DirEntryInner::file_type")
    }

    #[inline]
    pub fn file_name(&self) -> OsString {
        todo!("DirEntryInner::file_name")
    }

    #[inline]
    pub fn ino(&self) -> u64 {
        todo!("DirEntryInner::ino")
    }
}

impl fmt::Debug for DirEntryInner {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DirEntry").field(&self.file_name()).finish()
    }
}
