use crate::fs::{
    open_dir_for_reading, open_dir_for_reading_unchecked, open_entry_impl, read_dir_unchecked,
    remove_dir_unchecked, remove_file_unchecked, stat_unchecked, DirEntryInner, FollowSymlinks,
    Metadata, OpenOptions, ReadDir,
};
use posish::fs::Dir;
#[cfg(unix)]
use std::os::unix::{
    ffi::OsStrExt,
    io::{AsRawFd, RawFd},
};
#[cfg(target_os = "wasi")]
use std::os::wasi::{
    ffi::OsStrExt,
    io::{AsRawFd, RawFd},
};
use std::{
    ffi::OsStr,
    fmt, fs, io,
    path::{Component, Path},
    sync::Arc,
};
use unsafe_io::AsUnsafeFile;

pub(crate) struct ReadDirInner {
    posish: Arc<Dir>,
}

impl ReadDirInner {
    pub(crate) fn new(start: &fs::File, path: &Path) -> io::Result<Self> {
        Ok(Self {
            posish: Arc::new(Dir::from(open_dir_for_reading(start, path)?)?),
        })
    }

    pub(crate) fn read_base_dir(start: &fs::File) -> io::Result<Self> {
        // Open ".", to obtain a new independent file descriptor. Don't use
        // `dup` since in that case the resulting file descriptor would share
        // a current position with the original, and `read_dir` calls after
        // the first `read_dir` call wouldn't start from the beginning.
        Ok(Self {
            posish: Arc::new(Dir::from(open_dir_for_reading_unchecked(
                start,
                Component::CurDir.as_ref(),
            )?)?),
        })
    }

    pub(crate) fn new_unchecked(start: &fs::File, path: &Path) -> io::Result<Self> {
        let dir = open_dir_for_reading_unchecked(start, path)?;
        Ok(Self {
            posish: Arc::new(Dir::from(dir)?),
        })
    }

    pub(super) fn open(&self, file_name: &OsStr, options: &OpenOptions) -> io::Result<fs::File> {
        open_entry_impl(&self.as_file_view(), file_name, options)
    }

    pub(super) fn metadata(&self, file_name: &OsStr) -> io::Result<Metadata> {
        stat_unchecked(&self.as_file_view(), file_name.as_ref(), FollowSymlinks::No)
    }

    pub(super) fn remove_file(&self, file_name: &OsStr) -> io::Result<()> {
        remove_file_unchecked(&self.as_file_view(), file_name.as_ref())
    }

    pub(super) fn remove_dir(&self, file_name: &OsStr) -> io::Result<()> {
        remove_dir_unchecked(&self.as_file_view(), file_name.as_ref())
    }

    pub(super) fn self_metadata(&self) -> io::Result<Metadata> {
        Metadata::from_file(&self.as_file_view())
    }

    pub(super) fn read_dir(&self, file_name: &OsStr) -> io::Result<ReadDir> {
        read_dir_unchecked(&self.as_file_view(), file_name.as_ref())
    }
}

impl Iterator for ReadDirInner {
    type Item = io::Result<DirEntryInner>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = match self.posish.read()? {
                Ok(entry) => entry,
                Err(e) => return Some(Err(e)),
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

impl AsRawFd for ReadDirInner {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.posish.as_raw_fd()
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
