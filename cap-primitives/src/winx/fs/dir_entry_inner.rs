use super::open_options_to_std;
use crate::fs::{
    open, open_ambient_dir, FileType, FollowSymlinks, Metadata, OpenOptions, ReadDir, ReadDirInner,
};
use std::{ffi::OsString, fmt, fs, io};

pub(crate) struct DirEntryInner {
    std: fs::DirEntry,
}

impl DirEntryInner {
    #[inline]
    pub(crate) fn open(&self, options: &OpenOptions) -> io::Result<fs::File> {
        match options.follow {
            FollowSymlinks::No => open_options_to_std(options).open(self.std.path()),
            FollowSymlinks::Yes => unsafe {
                let path = self.std.path();
                open(
                    &open_ambient_dir(path.parent().unwrap())?,
                    path.file_name().unwrap().as_ref(),
                    options,
                )
            },
        }
    }

    #[inline]
    pub(crate) fn metadata(&self) -> io::Result<Metadata> {
        self.std.metadata().map(Metadata::from_std)
    }

    #[inline]
    pub(crate) fn remove_file(&self) -> io::Result<()> {
        fs::remove_file(self.std.path())
    }

    #[inline]
    pub(crate) fn remove_dir(&self) -> io::Result<()> {
        fs::remove_dir(self.std.path())
    }

    #[inline]
    pub(crate) fn read_dir(&self) -> io::Result<ReadDir> {
        let std = fs::read_dir(self.std.path())?;
        let inner = ReadDirInner::from_std(std);
        Ok(ReadDir { inner })
    }

    #[inline]
    pub(crate) fn file_type(&self) -> io::Result<FileType> {
        self.std.file_type().map(FileType::from_std)
    }

    #[inline]
    pub(crate) fn file_name(&self) -> OsString {
        self.std.file_name()
    }

    #[inline]
    pub(crate) fn is_same_file(&self, metadata: &Metadata) -> io::Result<bool> {
        // Don't use `self.metadata()`, because that doesn't include the
        // volume serial number which we need.
        // https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html#tymethod.volume_serial_number
        Ok(Metadata::from_std(fs::metadata(self.std.path())?).is_same_file(metadata))
    }

    #[inline]
    pub(super) fn from_std(std: fs::DirEntry) -> Self {
        Self { std }
    }
}

impl fmt::Debug for DirEntryInner {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DirEntry").field(&self.file_name()).finish()
    }
}
