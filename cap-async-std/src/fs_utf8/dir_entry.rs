use crate::{
    fs::{FileType, Metadata},
    fs_utf8::to_utf8,
};
use std::{fmt, io};

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
    cap_std: crate::fs::DirEntry,
}

impl DirEntry {
    /// Constructs a new instance of `Self` from the given `cap_std::fs::DirEntry`.
    #[inline]
    pub fn from_cap_std(cap_std: crate::fs::DirEntry) -> Self {
        Self { cap_std }
    }

    /// Returns the metadata for the file that this entry points at.
    ///
    /// This corresponds to [`async_std::fs::DirEntry::metadata`].
    ///
    /// [`async_std::fs::DirEntry::metadata`]: https://docs.rs/async-std/latest/async_std/fs/struct.DirEntry.html#method.metadata
    #[inline]
    pub async fn metadata(&self) -> io::Result<Metadata> {
        self.cap_std.metadata().await
    }

    /// Returns the file type for the file that this entry points at.
    ///
    /// This corresponds to [`async_std::fs::DirEntry::file_type`].
    ///
    /// [`async_std::fs::DirEntry::file_type`]: https://docs.rs/async-std/latest/async_std/fs/struct.DirEntry.html#method.file_type
    #[inline]
    pub async fn file_type(&self) -> io::Result<FileType> {
        self.cap_std.file_type().await
    }

    /// Returns the bare file name of this directory entry without any other leading path component.
    ///
    /// This corresponds to [`async_std::fs::DirEntry::file_name`].
    ///
    /// [`async_std::fs::DirEntry::file_name`]: https://docs.rs/async-std/latest/async_std/fs/struct.DirEntry.html#method.file_name
    #[inline]
    pub fn file_name(&self) -> String {
        // Unwrap because assume that paths coming from the OS don't have embedded NULs.
        to_utf8(self.cap_std.file_name()).unwrap()
    }
}

#[cfg(unix)]
impl async_std::os::unix::fs::DirEntryExt for DirEntry {
    #[inline]
    fn ino(&self) -> u64 {
        self.cap_std.ino()
    }
}

impl fmt::Debug for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cap_std.fmt(f)
    }
}
