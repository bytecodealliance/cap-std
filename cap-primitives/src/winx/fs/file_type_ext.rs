use std::fs;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum FileTypeExt {
    #[cfg(feature = "windows_file_type_ext")]
    SymlinkFile,

    #[cfg(feature = "windows_file_type_ext")]
    SymlinkDir,

    #[cfg(not(feature = "windows_file_type_ext"))]
    SymlinkUnknown,
}

impl FileTypeExt {
    /// Constructs a new instance of `Self` from the given `std::fs::FileType`.
    #[cfg(feature = "windows_file_type_ext")]
    #[inline]
    pub(crate) fn from_std(std: fs::FileType) -> Option<Self> {
        use std::os::windows::fs::FileTypeExt;
        Some(if std.is_symlink_file() {
            Self::SymlinkFile
        } else if std.is_symlink_dir() {
            Self::SymlinkDir
        } else {
            return None;
        })
    }

    #[cfg(not(feature = "windows_file_type_ext"))]
    #[inline]
    pub(crate) fn from_std(std: fs::FileType) -> Option<Self> {
        Some(if std.is_symlink() {
            Self::SymlinkUnknown
        } else {
            return None;
        })
    }

    /// Creates a `FileType` for which `is_symlink_file()` returns `true`.
    #[cfg(feature = "windows_file_type_ext")]
    #[inline]
    pub(crate) const fn symlink_file() -> Self {
        Self::SymlinkFile
    }

    /// Creates a `FileType` for which `is_symlink_dir()` returns `true`.
    #[cfg(feature = "windows_file_type_ext")]
    #[inline]
    pub(crate) const fn symlink_dir() -> Self {
        Self::SymlinkDir
    }

    #[inline]
    pub(crate) fn is_symlink(&self) -> bool {
        // All current `FileTypeExt` types are symlinks.
        true
    }
}
