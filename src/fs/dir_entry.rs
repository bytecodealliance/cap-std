use crate::fs::{Dir, FileType, Metadata};
use std::{ffi, fmt, io, path::PathBuf};

/// Entries returned by the `ReadDir` iterator.
///
/// This corresponds to [`std::fs::DirEntry`].
///
/// Unlike `std::fs::DirEntry`, this API has no `DirEntry::path`, because
/// absolute paths don't interoperate well with the capability model.
///
/// And unlike `std::fs::DirEntry`, this API has a lifetime parameter.
///
/// Note that there is no `from_std` method, as `std::fs::DirEntry` doesn't
/// provide a way to construct a `DirEntry` without opening directories by
/// ambient paths.
///
/// [`std::fs::DirEntry`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html
pub struct DirEntry<'dir> {
    dir: &'dir Dir,
    name: PathBuf,
    file_type: FileType,
    #[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    ino: u64,
}

impl<'dir> DirEntry<'dir> {
    /// Returns the metadata for the file that this entry points at.
    ///
    /// This corresponds to [`std::fs::DirEntry::metadata`].
    ///
    /// [`std::fs::DirEntry::metadata`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.metadata
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.dir.metadata(&self.name)
    }

    /// Returns the file type for the file that this entry points at.
    ///
    /// This corresponds to [`std::fs::DirEntry::file_type`].
    ///
    /// [`std::fs::DirEntry::file_type`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.file_type
    #[inline]
    pub fn file_type(&self) -> io::Result<FileType> {
        Ok(self.file_type)
    }

    /// Returns the bare file name of this directory entry without any other leading path component.
    ///
    /// This corresponds to [`std::fs::DirEntry::file_name`].
    ///
    /// [`std::fs::DirEntry::file_name`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.file_name
    #[inline]
    pub fn file_name(&self) -> ffi::OsString {
        self.name.clone().into_os_string()
    }
}

#[cfg(unix)]
impl<'dir> std::os::unix::fs::DirEntryExt for DirEntry<'dir> {
    #[inline]
    fn ino(&self) -> u64 {
        self.ino
    }
}

impl<'dir> fmt::Debug for DirEntry<'dir> {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DirEntry").field(&self.file_name()).finish()
    }
}
