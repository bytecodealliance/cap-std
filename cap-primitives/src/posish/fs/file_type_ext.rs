use crate::fs::FileType;
use posish::fs::RawMode;
use std::{fs, io};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum FileTypeExt {
    Symlink,
    BlockDevice,
    CharDevice,
    Fifo,
    Socket,
}

impl FileTypeExt {
    /// Constructs a new instance of `FileType` from the given
    /// [`std::fs::File`] and [`std::fs::FileType`].
    #[inline]
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn from(_file: &fs::File, metadata: &fs::Metadata) -> io::Result<FileType> {
        // On Posish-style platforms, the `Metadata` has everything we need.
        Ok(Self::from_just_metadata(metadata))
    }

    /// Constructs a new instance of `FileType` from the given
    /// [`std::fs::Metadata`].
    #[inline]
    pub(crate) fn from_just_metadata(metadata: &fs::Metadata) -> FileType {
        let std = metadata.file_type();
        Self::from_std(std)
    }

    /// Constructs a new instance of `Self` from the given
    /// [`std::fs::FileType`].
    #[inline]
    pub(crate) fn from_std(std: fs::FileType) -> FileType {
        use posish::fs::FileTypeExt;
        if std.is_file() {
            FileType::file()
        } else if std.is_dir() {
            FileType::dir()
        } else if std.is_symlink() {
            FileType::ext(Self::Symlink)
        } else if std.is_block_device() {
            FileType::ext(Self::BlockDevice)
        } else if std.is_char_device() {
            FileType::ext(Self::CharDevice)
        } else if std.is_fifo() {
            FileType::ext(Self::Fifo)
        } else if std.is_socket() {
            FileType::ext(Self::Socket)
        } else {
            FileType::unknown()
        }
    }

    /// Constructs a new instance of `FileType` from the given
    /// [`RawMode`].
    #[inline]
    pub(crate) const fn from_raw_mode(mode: RawMode) -> FileType {
        match posish::fs::FileType::from_raw_mode(mode) {
            posish::fs::FileType::RegularFile => FileType::file(),
            posish::fs::FileType::Directory => FileType::dir(),
            posish::fs::FileType::Symlink => FileType::ext(Self::symlink()),
            posish::fs::FileType::Fifo => FileType::ext(Self::fifo()),
            posish::fs::FileType::CharacterDevice => FileType::ext(Self::char_device()),
            posish::fs::FileType::BlockDevice => FileType::ext(Self::block_device()),
            posish::fs::FileType::Socket => FileType::ext(Self::socket()),
            _ => FileType::unknown(),
        }
    }

    /// Creates a `FileType` for which `is_symlink()` returns `true`.
    #[inline]
    pub(crate) const fn symlink() -> Self {
        Self::Symlink
    }

    /// Creates a `FileType` for which `is_block_device()` returns `true`.
    #[inline]
    pub(crate) const fn block_device() -> Self {
        Self::BlockDevice
    }

    /// Creates a `FileType` for which `is_char_device()` returns `true`.
    #[inline]
    pub(crate) const fn char_device() -> Self {
        Self::CharDevice
    }

    /// Creates a `FileType` for which `is_fifo()` returns `true`.
    #[inline]
    pub(crate) const fn fifo() -> Self {
        Self::Fifo
    }

    /// Creates a `FileType` for which `is_socket()` returns `true`.
    #[inline]
    pub(crate) const fn socket() -> Self {
        Self::Socket
    }

    /// Tests whether this file type represents a symbolic link.
    #[inline]
    pub(crate) fn is_symlink(&self) -> bool {
        *self == Self::Symlink
    }
}
