use crate::{
    fs::{FileType, Metadata},
    fs_utf8::to_utf8,
};
use std::io;

/// Entries returned by the `ReadDir` iterator.
///
/// This corresponds to [`std::fs::DirEntry`].
///
/// Unlike `async_std::fs::DirEntry`, this API has no `DirEntry::path`, because
/// absolute paths don't interoperate well with the capability model.
///
/// And unlike `async_std::fs::DirEntry`, this API has a lifetime parameter.
///
/// Note that there is no `from_std` method, as `async_std::fs::DirEntry` doesn't
/// provide a way to construct a `DirEntry` without opening directories by
/// ambient paths.
///
/// [`std::fs::DirEntry`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html
pub struct DirEntry<'dir> {
    cap_std: crate::fs::DirEntry<'dir>,
}

impl<'dir> DirEntry<'dir> {
    /// Constructs a new instance of `Self` from the given `cap_std::fs::DirEntry`.
    #[inline]
    pub fn from_cap_std(cap_std: crate::fs::DirEntry<'dir>) -> Self {
        Self { cap_std }
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

#[cfg(unix)]
impl<'dir> std::os::unix::fs::DirEntryExt for DirEntry<'dir> {
    fn ino(&self) -> u64 {
        self.cap_std.ino()
    }
}

// TODO: impl Debug for DirEntry? But don't expose DirEntry's path...
