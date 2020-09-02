use crate::fs::{
    open_dir, open_dir_unchecked, open_entry_impl, read_dir_unchecked, rmdir_unchecked,
    stat_unchecked, unlink_unchecked, DirEntryInner, FollowSymlinks, Metadata, OpenOptions,
    ReadDir,
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
    pub(crate) fn new(start: &fs::File, path: &Path) -> io::Result<Self> {
        Ok(Self {
            yanix: Arc::new(Dir::from(open_dir(start, path)?)?),
        })
    }

    pub(crate) fn new_unchecked(start: &fs::File, path: &Path) -> io::Result<Self> {
        let dir = open_dir_unchecked(start, path)?;
        Ok(Self {
            yanix: Arc::new(Dir::from(dir)?),
        })
    }

    pub(super) fn open(&self, file_name: &OsStr, options: &OpenOptions) -> io::Result<fs::File> {
        open_entry_impl(&self.to_std_file(), file_name, options)
    }

    pub(super) fn metadata(&self, file_name: &OsStr) -> io::Result<Metadata> {
        stat_unchecked(&self.to_std_file(), file_name.as_ref(), FollowSymlinks::No)
    }

    pub(super) fn remove_file(&self, file_name: &OsStr) -> io::Result<()> {
        unlink_unchecked(&self.to_std_file(), file_name.as_ref())
    }

    pub(super) fn remove_dir(&self, file_name: &OsStr) -> io::Result<()> {
        rmdir_unchecked(&self.to_std_file(), file_name.as_ref())
    }

    pub(super) fn self_metadata(&self) -> io::Result<Metadata> {
        self.to_std_file().metadata().map(Metadata::from_std)
    }

    pub(super) fn read_dir(&self, file_name: &OsStr) -> io::Result<ReadDir> {
        read_dir_unchecked(&self.to_std_file(), file_name.as_ref())
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
