use crate::fs::{DirEntry, ReadDirInner};
use std::{fmt, fs, io, path::Path};

/// Construct a `ReadDir` to iterate over the contents of a directory,
/// ensuring that the resolution of the path never escapes the directory
/// tree rooted at `start`.
#[inline]
pub fn read_dir(start: &fs::File, path: &Path) -> io::Result<ReadDir> {
    Ok(ReadDir {
        inner: ReadDirInner::read_dir(start, path)?,
    })
}

/// Like `read_dir`, but doesn't perform sandboxing.
#[inline]
pub(crate) fn read_dir_unchecked(start: &fs::File, path: &Path) -> io::Result<ReadDir> {
    Ok(ReadDir {
        inner: ReadDirInner::read_dir_unchecked(start, path)?,
    })
}

/// Iterator over the entries in a directory.
///
/// This corresponds to [`std::fs::ReadDir`].
///
/// Note that there is no `from_std` method, as `std::fs::ReadDir` doesn't
/// provide a way to construct a `ReadDir` without opening directories by
/// ambient paths.
///
/// [`std::fs::ReadDir`]: https://doc.rust-lang.org/std/fs/struct.ReadDir.html
pub struct ReadDir {
    pub(crate) inner: ReadDirInner,
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|inner| inner.map(|inner| DirEntry { inner }))
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
