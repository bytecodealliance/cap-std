use crate::fs::FileType;
use std::{fs, io, os::windows::io::AsRawHandle};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum FileTypeExt {
    CharacterDevice,
    Fifo,
    #[cfg(feature = "windows_file_type_ext")]
    SymlinkFile,
    #[cfg(feature = "windows_file_type_ext")]
    SymlinkDir,
    SymlinkUnknown,
}

impl FileTypeExt {
    /// Constructs a new instance of `Self` from the given `std::fs::File` and
    /// `std::fs::Metadata`.
    pub(crate) fn from(file: &fs::File, metadata: &fs::Metadata) -> io::Result<FileType> {
        // Check for the things we can do with just metadata.
        let file_type = Self::from_just_metadata(metadata);
        if file_type != FileType::unknown() {
            return Ok(file_type);
        }

        // Use the open file to check for one of the exotic file types.
        let file_type = unsafe { winx::file::get_file_type(file.as_raw_handle())? };
        if file_type.is_char() {
            return Ok(FileType::ext(FileTypeExt::CharacterDevice));
        }
        if file_type.is_pipe() {
            return Ok(FileType::ext(FileTypeExt::Fifo));
        }

        Ok(FileType::unknown())
    }

    /// Constructs a new instance of `Self` from the given `std::fs::Metadata`.
    #[inline]
    pub(crate) fn from_just_metadata(metadata: &fs::Metadata) -> FileType {
        let std = metadata.file_type();
        Self::from_std(std)
    }

    /// Constructs a new instance of `Self` from the given `std::fs::FileType`.
    #[inline]
    pub(crate) fn from_std(std: fs::FileType) -> FileType {
        if std.is_file() {
            return FileType::file();
        }
        if std.is_dir() {
            return FileType::dir();
        }

        #[cfg(feature = "windows_file_type_ext")]
        {
            use std::os::windows::fs::FileTypeExt;
            if std.is_symlink_file() {
                return FileType::ext(Self::SymlinkFile);
            }
            if std.is_symlink_dir() {
                return FileType::ext(Self::SymlinkDir);
            }
        }

        if std.is_symlink() {
            return FileType::ext(Self::SymlinkUnknown);
        }

        FileType::unknown()
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
        match self {
            #[cfg(feature = "windows_file_type_ext")]
            Self::SymlinkFile | Self::SymlinkDir => true,
            Self::SymlinkUnknown => true,
            _ => false,
        }
    }
}

#[cfg(windows)]
#[doc(hidden)]
impl crate::fs::_WindowsFileTypeExt for crate::fs::FileType {
    #[inline]
    unsafe fn is_block_device(&self) -> bool {
        false
    }

    #[inline]
    unsafe fn is_char_device(&self) -> bool {
        *self == FileType::ext(FileTypeExt::CharacterDevice)
    }

    #[inline]
    unsafe fn is_fifo(&self) -> bool {
        *self == FileType::ext(FileTypeExt::Fifo)
    }

    #[inline]
    unsafe fn is_socket(&self) -> bool {
        false
    }
}
