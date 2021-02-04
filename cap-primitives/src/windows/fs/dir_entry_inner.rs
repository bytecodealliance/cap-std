use super::open_options_to_std;
use crate::fs::{
    open, open_ambient_dir, FileType, FileTypeExt, FollowSymlinks, Metadata, OpenOptions, ReadDir,
    ReadDirInner,
};
use std::{ffi::OsString, fmt, fs, io};

pub(crate) struct DirEntryInner {
    std: fs::DirEntry,
}

impl DirEntryInner {
    #[inline]
    pub(crate) fn open(&self, options: &OpenOptions) -> io::Result<fs::File> {
        match options.follow {
            FollowSymlinks::No => {
                let (opts, manually_trunc) = open_options_to_std(options);
                let file = opts.open(self.std.path())?;
                if manually_trunc {
                    // Unwrap is ok because 0 never overflows, and we'll only
                    // have `manually_trunc` set when the file is opened for
                    // writing.
                    file.set_len(0).unwrap();
                }
                Ok(file)
            }
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
        self.std.metadata().map(Metadata::from_just_metadata)
    }

    #[inline]
    pub(crate) fn full_metadata(&self) -> io::Result<Metadata> {
        // If we can open the file, we can get a more complete Metadata which
        // includes `file_index`, `volume_serial_number`, and
        // `number_of_links`.
        let mut opts = OpenOptions::new();
        opts.access_mode(0);
        opts.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS);
        opts.follow(FollowSymlinks::No);
        match self.open(&opts) {
            Ok(opened) => Metadata::from_file(&opened),
            Err(_) => self.metadata(),
        }
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
        self.std.file_type().map(FileTypeExt::from_std)
    }

    #[inline]
    pub(crate) fn file_name(&self) -> OsString {
        self.std.file_name()
    }

    #[inline]
    #[cfg(windows_by_handle)]
    pub(crate) fn is_same_file(&self, metadata: &Metadata) -> io::Result<bool> {
        // Don't use `self.metadata()`, because that doesn't include the
        // volume serial number which we need.
        // https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html#tymethod.volume_serial_number
        Ok(Metadata::from_just_metadata(fs::metadata(self.std.path())?).is_same_file(metadata))
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

#[cfg(windows)]
#[doc(hidden)]
unsafe impl crate::fs::_WindowsDirEntryExt for crate::fs::DirEntry {
    #[inline]
    unsafe fn full_metadata(&self) -> io::Result<Metadata> {
        DirEntryInner::full_metadata(&self.inner)
    }
}
