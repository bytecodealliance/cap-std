use crate::fs::{
    open_dir, open_dir_unchecked, open_entry_impl, read_dir_unchecked, remove_dir_unchecked,
    remove_file_unchecked, stat_unchecked, DirEntryInner, FollowSymlinks, Metadata, OpenOptions,
    ReadDir,
};
use posish::fs::Dir;
#[cfg(unix)]
use std::os::unix::{
    ffi::OsStrExt,
    io::{AsRawFd, FromRawFd},
};
#[cfg(target_os = "wasi")]
use std::os::wasi::{
    ffi::OsStrExt,
    io::{AsRawFd, FromRawFd},
};
use std::{
    ffi::OsStr,
    fmt, fs, io,
    mem::ManuallyDrop,
    path::{Component, Path},
    sync::Arc,
};

pub(crate) struct ReadDirInner {
    posish: Arc<Dir>,
}

impl ReadDirInner {
    pub(crate) fn new(start: &fs::File, path: &Path) -> io::Result<Self> {
        Ok(Self {
            posish: Arc::new(Dir::from(open_dir(start, path)?)?),
        })
    }

    pub(crate) fn new_unchecked(start: &fs::File, path: &Path) -> io::Result<Self> {
        let dir = open_dir_unchecked(start, path)?;
        Ok(Self {
            posish: Arc::new(Dir::from(dir)?),
        })
    }

    pub(super) fn open(&self, file_name: &OsStr, options: &OpenOptions) -> io::Result<fs::File> {
        unsafe { open_entry_impl(&self.to_std_file(), file_name, options) }
    }

    pub(super) fn metadata(&self, file_name: &OsStr) -> io::Result<Metadata> {
        unsafe { stat_unchecked(&self.to_std_file(), file_name.as_ref(), FollowSymlinks::No) }
    }

    pub(super) fn remove_file(&self, file_name: &OsStr) -> io::Result<()> {
        unsafe { remove_file_unchecked(&self.to_std_file(), file_name.as_ref()) }
    }

    pub(super) fn remove_dir(&self, file_name: &OsStr) -> io::Result<()> {
        unsafe { remove_dir_unchecked(&self.to_std_file(), file_name.as_ref()) }
    }

    pub(super) fn self_metadata(&self) -> io::Result<Metadata> {
        unsafe { Metadata::from_file(&self.to_std_file()) }
    }

    pub(super) fn read_dir(&self, file_name: &OsStr) -> io::Result<ReadDir> {
        unsafe { read_dir_unchecked(&self.to_std_file(), file_name.as_ref()) }
    }

    /// # Safety
    ///
    /// The resulting `fs::File` shouldn't outlive `self`.
    unsafe fn to_std_file(&self) -> ManuallyDrop<fs::File> {
        ManuallyDrop::<fs::File>::new(fs::File::from_raw_fd(self.posish.as_raw_fd()))
    }
}

impl Iterator for ReadDirInner {
    type Item = io::Result<DirEntryInner>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = match self.posish.read()? {
                Err(e) => return Some(Err(e)),
                Ok(entry) => entry,
            };
            let file_name = entry.file_name().to_bytes();
            if file_name != Component::CurDir.as_os_str().as_bytes()
                && file_name != Component::ParentDir.as_os_str().as_bytes()
            {
                let clone = Arc::clone(&self.posish);
                return Some(Ok(DirEntryInner {
                    posish: entry,
                    read_dir: Self { posish: clone },
                }));
            }
        }
    }
}

impl fmt::Debug for ReadDirInner {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b = f.debug_struct("ReadDir");
        let fd = self.posish.as_raw_fd();
        b.field("fd", &fd);
        b.finish()
    }
}
