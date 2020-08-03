use crate::fs::{
    dir_options, open, open_entry_impl, open_unchecked, rmdir_unchecked, stat_unchecked,
    unlink_unchecked, DirEntryInner, FollowSymlinks, Metadata, OpenOptions, OpenUncheckedError,
};
use std::{
    ffi::OsStr,
    fmt, fs, io,
    mem::ManuallyDrop,
    os::unix::{
        ffi::OsStrExt,
        io::{AsRawFd, FromRawFd},
    },
    path::{Component, Path},
    sync::Arc,
};
use yanix::dir::{Dir, DirIter};

pub(crate) struct ReadDirInner {
    yanix: Arc<Dir>,
}

impl ReadDirInner {
    pub(crate) fn read_dir(start: &fs::File, path: &Path) -> io::Result<Self> {
        Ok(Self {
            yanix: Arc::new(Dir::from(open(start, path, &dir_options())?)?),
        })
    }

    pub(crate) fn read_dir_unchecked(start: &fs::File, path: &Path) -> io::Result<Self> {
        let dir = open_unchecked(start, path, &dir_options())
            .map_err(OpenUncheckedError::into_io_error)?;
        Ok(Self {
            yanix: Arc::new(Dir::from(dir)?),
        })
    }

    pub(crate) fn open(&self, file_name: &OsStr, options: &OpenOptions) -> io::Result<fs::File> {
        open_entry_impl(&self.to_std_file(), file_name, options)
    }

    pub(crate) fn metadata(&self, file_name: &OsStr) -> io::Result<Metadata> {
        stat_unchecked(&self.to_std_file(), file_name.as_ref(), FollowSymlinks::No)
    }

    pub(crate) fn remove_file(&self, file_name: &OsStr) -> io::Result<()> {
        unlink_unchecked(&self.to_std_file(), file_name.as_ref())
    }

    pub(crate) fn remove_dir(&self, file_name: &OsStr) -> io::Result<()> {
        rmdir_unchecked(&self.to_std_file(), file_name.as_ref())
    }

    pub(crate) fn self_metadata(&self) -> io::Result<Metadata> {
        self.to_std_file().metadata().map(Metadata::from_std)
    }

    fn to_std_file(&self) -> ManuallyDrop<fs::File> {
        ManuallyDrop::<fs::File>::new(unsafe { fs::File::from_raw_fd(self.yanix.as_raw_fd()) })
    }
}

impl Iterator for ReadDirInner {
    type Item = io::Result<DirEntryInner>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = match DirIter::new(&*self.yanix).next()? {
                Err(e) => return Some(Err(e)),
                Ok(entry) => entry,
            };
            let file_name = entry.file_name().to_bytes();
            if file_name != Component::CurDir.as_os_str().as_bytes()
                && file_name != Component::ParentDir.as_os_str().as_bytes()
            {
                let clone = Arc::clone(&self.yanix);
                return Some(Ok(DirEntryInner {
                    yanix: entry,
                    read_dir: Self { yanix: clone },
                }));
            }
        }
    }
}

impl fmt::Debug for ReadDirInner {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b = f.debug_struct("ReadDir");
        let fd = self.yanix.as_raw_fd();
        b.field("fd", &fd);
        b.finish()
    }
}
