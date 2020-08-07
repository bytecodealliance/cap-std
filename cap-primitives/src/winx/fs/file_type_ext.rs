use std::fs;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Inner {
    SymlinkFile,
    SymlinkDir,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) struct FileTypeExt(Inner);

impl FileTypeExt {
    /// Constructs a new instance of `Self` from the given `std::fs::FileType`.
    #[inline]
    pub(crate) fn from_std(std: fs::FileType) -> Option<Self> {
        use std::os::windows::fs::FileTypeExt;
        Some(Self(if std.is_symlink_file() {
            Inner::SymlinkFile
        } else if std.is_symlink_dir() {
            Inner::SymlinkDir
        } else {
            return None;
        }))
    }

    /// Creates a `FileType` for which `is_symlink_file()` returns `true`.
    #[inline]
    pub(crate) const fn symlink_file() -> Self {
        Self(Inner::SymlinkFile)
    }

    /// Creates a `FileType` for which `is_symlink_dir()` returns `true`.
    #[inline]
    pub(crate) const fn symlink_dir() -> Self {
        Self(Inner::SymlinkDir)
    }

    #[inline]
    pub(crate) fn is_symlink(&self) -> bool {
        // All current `FileTypeExt` types are symlinks.
        true
    }
}
