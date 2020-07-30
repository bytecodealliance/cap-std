use crate::fs::{Dir, File, FileType, Metadata, OpenOptions};
use async_std::io;
use std::{ffi::OsString, fmt};

/// Entries returned by the `ReadDir` iterator.
///
/// This corresponds to [`async_std::fs::DirEntry`].
///
/// Unlike `async_std::fs::DirEntry`, this API has no `DirEntry::path`, because
/// absolute paths don't interoperate well with the capability model.
///
/// There is a `file_name` function, however there are also `open`,
/// `open_with`, `open_dir`, `remove_file`, and `remove_dir` functions for
/// opening or removing the entry directly, which can be more efficient and
/// convenient.
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
    /// Open the file for reading.
    #[inline]
    pub fn open(&self) -> io::Result<File> {
        self.inner.open().map(|f| File::from_std(f.into()))
    }

    /// Open the file with the given options.
    #[inline]
    pub fn open_with(&self, options: &OpenOptions) -> io::Result<File> {
        self.inner
            .open_with(options)
            .map(|f| File::from_std(f.into()))
    }

    /// Open the entry as a directory.
    #[inline]
    pub fn open_dir(&self) -> io::Result<Dir> {
        self.inner.open_dir().map(|f| Dir::from_std_file(f.into()))
    }

    /// Removes the file from its filesystem.
    #[inline]
    pub fn remove_file(&self) -> io::Result<()> {
        self.inner.remove_file()
    }

    /// Removes the directory from its filesystem.
    #[inline]
    pub fn remove_dir(&self) -> io::Result<()> {
        self.inner.remove_dir()
    }

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
    pub fn file_name(&self) -> OsString {
        self.inner.file_name()
    }
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
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
