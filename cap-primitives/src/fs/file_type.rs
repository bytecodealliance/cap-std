//! The `FileType` struct.

use crate::fs::FileTypeExt;

/// `FileType`'s inner state.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Inner {
    /// A directory.
    Dir,

    /// A file.
    File,

    /// An unknown entity.
    Unknown,

    /// A `FileTypeExt` type.
    Ext(FileTypeExt),
}

/// A structure representing a type of file with accessors for each file type.
///
/// This corresponds to [`std::fs::FileType`].
///
/// <details>
/// We need to define our own version because the libstd `FileType` doesn't
/// have a public constructor that we can use.
/// </details>
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct FileType(Inner);

impl FileType {
    /// Creates a `FileType` for which `is_dir()` returns `true`.
    #[inline]
    pub const fn dir() -> Self {
        Self(Inner::Dir)
    }

    /// Creates a `FileType` for which `is_file()` returns `true`.
    #[inline]
    pub const fn file() -> Self {
        Self(Inner::File)
    }

    /// Creates a `FileType` for which `is_unknown()` returns `true`.
    #[inline]
    pub const fn unknown() -> Self {
        Self(Inner::Unknown)
    }

    /// Creates a `FileType` from extension type.
    #[inline]
    pub(crate) const fn ext(ext: FileTypeExt) -> Self {
        Self(Inner::Ext(ext))
    }

    /// Tests whether this file type represents a directory.
    ///
    /// This corresponds to [`std::fs::FileType::is_dir`].
    #[inline]
    pub fn is_dir(&self) -> bool {
        self.0 == Inner::Dir
    }

    /// Tests whether this file type represents a regular file.
    ///
    /// This corresponds to [`std::fs::FileType::is_file`].
    #[inline]
    pub fn is_file(&self) -> bool {
        self.0 == Inner::File
    }

    /// Tests whether this file type represents a symbolic link.
    ///
    /// This corresponds to [`std::fs::FileType::is_symlink`].
    #[inline]
    pub fn is_symlink(&self) -> bool {
        if let Inner::Ext(ext) = self.0 {
            ext.is_symlink()
        } else {
            false
        }
    }
}

#[cfg(unix)]
impl std::os::unix::fs::FileTypeExt for FileType {
    #[inline]
    fn is_block_device(&self) -> bool {
        self.0 == Inner::Ext(crate::fs::FileTypeExt::block_device())
    }

    #[inline]
    fn is_char_device(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::char_device())
    }

    #[inline]
    fn is_fifo(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::fifo())
    }

    #[inline]
    fn is_socket(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::socket())
    }
}

#[cfg(target_os = "vxworks")]
impl std::os::vxworks::fs::FileTypeExt for FileType {
    #[inline]
    fn is_block_device(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::BlockDevice)
    }

    #[inline]
    fn is_char_device(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::CharDevice)
    }

    #[inline]
    fn is_fifo(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::Fifo)
    }

    #[inline]
    fn is_socket(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::Socket)
    }
}

#[cfg(all(windows, windows_file_type_ext))]
impl std::os::windows::fs::FileTypeExt for FileType {
    #[inline]
    fn is_symlink_dir(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::symlink_dir())
    }

    #[inline]
    fn is_symlink_file(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::symlink_file())
    }
}

/// Extension trait to allow `is_block_device` etc. to be exposed by
/// the `cap-fs-ext` crate.
///
/// This is hidden from the main API since this functionality isn't present in
/// `std`. Use `cap_fs_ext::FileTypeExt` instead of calling this directly.
#[cfg(windows)]
#[doc(hidden)]
pub trait _WindowsFileTypeExt {
    fn is_block_device(&self) -> bool;
    fn is_char_device(&self) -> bool;
    fn is_fifo(&self) -> bool;
    fn is_socket(&self) -> bool;
}
