use crate::fs::{FileType, Metadata};
use async_std::io;
use std::{ffi, fmt};

/// Entries returned by the `ReadDir` iterator.
///
/// This corresponds to [`async_std::fs::DirEntry`].
///
/// Unlike `async_std::fs::DirEntry`, this API has no `DirEntry::path`, because
/// absolute paths don't interoperate well with the capability model.
///
/// Note that there is no `from_std` method, as `async_std::fs::DirEntry` doesn't
/// provide a way to construct a `DirEntry` without opening directories by
/// ambient paths.
///
/// [`async_std::fs::DirEntry`]: https://docs.rs/async-std/latest/async_std/fs/struct.DirEntry.html
pub struct DirEntry {
    pub(crate) inner: cap_primitives::fs::DirEntry,
}

impl DirEntry {
    /// Returns the metadata for the file that this entry points at.
    ///
    /// This corresponds to [`async_std::fs::DirEntry::metadata`].
    ///
    /// [`async_std::fs::DirEntry::metadata`]: https://docs.rs/async-std/latest/async_std/fs/struct.DirEntry.html#method.metadata
    #[inline]
    pub async fn metadata(&self) -> io::Result<Metadata> {
        // TODO: Make this actually async.
        self.inner.metadata()
    }

    /// Returns the file type for the file that this entry points at.
    ///
    /// This corresponds to [`async_std::fs::DirEntry::file_type`].
    ///
    /// [`async_std::fs::DirEntry::file_type`]: https://docs.rs/async-std/latest/async_std/fs/struct.DirEntry.html#method.file_type
    #[inline]
    pub async fn file_type(&self) -> io::Result<FileType> {
        // TODO: Make this actually async.
        self.inner.file_type()
    }

    /// Returns the bare file name of this directory entry without any other leading path component.
    ///
    /// This corresponds to [`async_std::fs::DirEntry::file_name`].
    ///
    /// [`async_std::fs::DirEntry::file_name`]: https://docs.rs/async-std/latest/async_std/fs/struct.DirEntry.html#method.file_name
    #[inline]
    pub fn file_name(&self) -> ffi::OsString {
        self.inner.file_name()
    }
}

#[cfg(unix)]
impl async_std::os::unix::fs::DirEntryExt for DirEntry {
    #[inline]
    fn ino(&self) -> u64 {
        self.inner.ino()
    }
}

impl fmt::Debug for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
