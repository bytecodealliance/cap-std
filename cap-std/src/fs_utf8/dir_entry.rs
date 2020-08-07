use crate::{
    fs::{FileType, Metadata, OpenOptions},
    fs_utf8::{to_utf8, Dir, File},
};
use std::{fmt, io};

/// Entries returned by the `ReadDir` iterator.
///
/// This corresponds to [`std::fs::DirEntry`].
///
/// Unlike `std::fs::DirEntry`, this API has no `DirEntry::path`, because
/// absolute paths don't interoperate well with the capability model.
///
/// There is a `file_name` function, however there are also `open`,
/// `open_with`, `open_dir`, `remove_file`, and `remove_dir` functions for
/// opening or removing the entry directly, which can be more efficient and
/// convenient.
///
/// Note that there is no `from_std` method, as `std::fs::DirEntry` doesn't
/// provide a way to construct a `DirEntry` without opening directories by
/// ambient paths.
///
/// [`std::fs::DirEntry`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html
pub struct DirEntry {
    cap_std: crate::fs::DirEntry,
}

impl DirEntry {
    /// Constructs a new instance of `Self` from the given `cap_std::fs::DirEntry`.
    #[inline]
    pub fn from_cap_std(cap_std: crate::fs::DirEntry) -> Self {
        Self { cap_std }
    }

    /// Open the file for reading.
    #[inline]
    pub fn open(&self) -> io::Result<File> {
        self.cap_std.open().map(File::from_cap_std)
    }

    /// Open the file with the given options.
    #[inline]
    pub fn open_with(&self, options: &OpenOptions) -> io::Result<File> {
        self.cap_std.open_with(options).map(File::from_cap_std)
    }

    /// Open the entry as a directory.
    #[inline]
    pub fn open_dir(&self) -> io::Result<Dir> {
        self.cap_std.open_dir().map(Dir::from_cap_std)
    }

    /// Removes the file from its filesystem.
    #[inline]
    pub fn remove_file(&self) -> io::Result<()> {
        self.cap_std.remove_file()
    }

    /// Removes the directory from its filesystem.
    #[inline]
    pub fn remove_dir(&self) -> io::Result<()> {
        self.cap_std.remove_dir()
    }

    /// Returns the metadata for the file that this entry points at.
    ///
    /// This corresponds to [`std::fs::DirEntry::metadata`].
    ///
    /// [`std::fs::DirEntry::metadata`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.metadata
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.cap_std.metadata()
    }

    /// Returns the file type for the file that this entry points at.
    ///
    /// This corresponds to [`std::fs::DirEntry::file_type`].
    ///
    /// [`std::fs::DirEntry::file_type`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.file_type
    #[inline]
    pub fn file_type(&self) -> io::Result<FileType> {
        self.cap_std.file_type()
    }

    /// Returns the bare file name of this directory entry without any other leading path component.
    ///
    /// This corresponds to [`std::fs::DirEntry::file_name`].
    ///
    /// [`std::fs::DirEntry::file_name`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.file_name
    #[inline]
    pub fn file_name(&self) -> String {
        // Unwrap because assume that paths coming from the OS don't have embedded NULs.
        to_utf8(self.cap_std.file_name()).unwrap()
    }
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
impl std::os::unix::fs::DirEntryExt for DirEntry {
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
