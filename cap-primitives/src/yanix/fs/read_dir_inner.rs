use crate::fs::{dir_options, open, stat_unchecked, DirEntryInner, FollowSymlinks, Metadata};
use std::{
    ffi::OsStr,
    fmt, fs, io,
    mem::ManuallyDrop,
    os::unix::io::{AsRawFd, FromRawFd},
    path::Path,
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

    pub(crate) fn metadata(&self, file_name: &OsStr) -> io::Result<Metadata> {
        stat_unchecked(
            &ManuallyDrop::<fs::File>::new(unsafe {
                fs::File::from_raw_fd(self.yanix.as_raw_fd())
            }),
            file_name.as_ref(),
            FollowSymlinks::No,
        )
    }
}

impl Iterator for ReadDirInner {
    type Item = io::Result<DirEntryInner>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Could we avoid this `clone` by adjusting yanix's API?
        let mut iter = DirIter::new(Arc::clone(&self.yanix));
        let clone = Arc::clone(&self.yanix);
        loop {
            let entry = match iter.next()? {
                Err(e) => return Some(Err(e)),
                Ok(entry) => entry,
            };
            let file_name = entry.file_name().to_bytes();
            if file_name != b"." && file_name != b".." {
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
