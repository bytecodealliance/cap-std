use crate::fs::DirEntry;
use async_std::io;
use std::fmt;

/// Iterator over the entries in a directory.
///
/// This corresponds to [`async_std::fs::ReadDir`].
///
/// Note that there is no `from_std` method, as `async_std::fs::ReadDir` doesn't
/// provide a way to construct a `ReadDir` without opening directories by
/// ambient paths.
pub struct ReadDir {
    pub(crate) inner: cap_primitives::fs::ReadDir,
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
