use super::open_options_to_std;
use crate::fs::{open, open_ambient_dir, FileType, FollowSymlinks, Metadata, OpenOptions};
use std::{ffi::OsString, fmt, fs, io};

pub(crate) struct DirEntryInner {
    pub(crate) std: fs::DirEntry,
}

impl DirEntryInner {
    #[inline]
    pub fn open(&self, options: &OpenOptions) -> io::Result<fs::File> {
        todo!("DirEntryInner::open")
    }

    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.std.metadata().map(Metadata::from_std)
    }

    #[inline]
    pub fn remove_file(&self) -> io::Result<()> {
        fs::remove_file(self.std.path())
    }

    #[inline]
    pub fn remove_dir(&self) -> io::Result<()> {
        fs::remove_dir(self.std.path())
    }

    #[inline]
    pub fn file_type(&self) -> io::Result<FileType> {
        self.std.file_type().map(FileType::from_std)
    }

    #[inline]
    pub fn file_name(&self) -> OsString {
        self.std.file_name()
    }

    #[inline]
    pub(crate) fn is_same_file(&self, metadata: &Metadata) -> io::Result<bool> {
        Ok(self.metadata()?.is_same_file(metadata))
    }
}

impl fmt::Debug for DirEntryInner {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DirEntry").field(&self.file_name()).finish()
    }
}
