use crate::fs::{DirEntryInner, FileType, Metadata};
use std::{ffi::OsString, fmt, io};

/// Entries returned by the `ReadDir` iterator.
///
/// This corresponds to [`std::fs::DirEntry`].
///
/// Unlike `std::fs::DirEntry`, this API has no `DirEntry::path`, because
/// absolute paths don't interoperate well with the capability model.
///
/// Note that there is no `from_std` method, as `std::fs::DirEntry` doesn't
/// provide a way to construct a `DirEntry` without opening directories by
/// ambient paths.
///
/// [`std::fs::DirEntry`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html
pub struct DirEntry {
    pub(crate) inner: DirEntryInner,
}

impl DirEntry {
    /// Returns the metadata for the file that this entry points at.
    ///
    /// This corresponds to [`std::fs::DirEntry::metadata`].
    ///
    /// [`std::fs::DirEntry::metadata`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.metadata
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.inner.metadata()
    }

    /// Returns the file type for the file that this entry points at.
    ///
    /// This corresponds to [`std::fs::DirEntry::file_type`].
    ///
    /// [`std::fs::DirEntry::file_type`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.file_type
    #[inline]
    pub fn file_type(&self) -> io::Result<FileType> {
        Ok(self.inner.file_type())
    }

    /// Returns the bare file name of this directory entry without any other leading path component.
    ///
    /// This corresponds to [`std::fs::DirEntry::file_name`].
    ///
    /// [`std::fs::DirEntry::file_name`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.file_name
    #[inline]
    pub fn file_name(&self) -> OsString {
        self.inner.file_name()
    }
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
impl std::os::unix::fs::DirEntryExt for DirEntry {
    #[inline]
    fn ino(&self) -> u64 {
        self.inner.ino()
    }
}

impl fmt::Debug for DirEntry {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
