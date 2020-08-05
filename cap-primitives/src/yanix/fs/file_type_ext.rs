use crate::fs::FileType;
use std::fs;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Inner {
    Symlink,
    BlockDevice,
    CharDevice,
    Fifo,
    Socket,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) struct FileTypeExt(Inner);

impl FileTypeExt {
    /// Constructs a new instance of `Self` from the given `std::fs::FileType`.
    #[inline]
    pub(crate) fn from_std(std: fs::FileType) -> Option<Self> {
        use std::os::unix::fs::FileTypeExt;
        Some(Self(if std.is_symlink() {
            Inner::Symlink
        } else if std.is_block_device() {
            Inner::BlockDevice
        } else if std.is_char_device() {
            Inner::CharDevice
        } else if std.is_fifo() {
            Inner::Fifo
        } else if std.is_socket() {
            Inner::Socket
        } else {
            return None;
        }))
    }

    /// Constructs a new instance of `FileType` from the given `libc::mode_t`.
    #[inline]
    pub(crate) fn from_libc(mode: libc::mode_t) -> FileType {
        match mode & libc::S_IFMT {
            libc::S_IFREG => FileType::file(),
            libc::S_IFDIR => FileType::dir(),
            libc::S_IFLNK => FileType::ext(Self::symlink()),
            libc::S_IFIFO => FileType::ext(Self::fifo()),
            libc::S_IFCHR => FileType::ext(Self::char_device()),
            libc::S_IFBLK => FileType::ext(Self::block_device()),
            libc::S_IFSOCK => FileType::ext(Self::socket()),
            _ => FileType::unknown(),
        }
    }

    /// Creates a `FileType` for which `is_symlink()` returns `true`.
    #[inline]
    pub(crate) const fn symlink() -> Self {
        Self(Inner::Symlink)
    }

    /// Creates a `FileType` for which `is_block_device()` returns `true`.
    #[inline]
    pub(crate) const fn block_device() -> Self {
        Self(Inner::BlockDevice)
    }

    /// Creates a `FileType` for which `is_char_device()` returns `true`.
    #[inline]
    pub(crate) const fn char_device() -> Self {
        Self(Inner::CharDevice)
    }

    /// Creates a `FileType` for which `is_fifo()` returns `true`.
    #[inline]
    pub(crate) const fn fifo() -> Self {
        Self(Inner::Fifo)
    }

    /// Creates a `FileType` for which `is_socket()` returns `true`.
    #[inline]
    pub(crate) const fn socket() -> Self {
        Self(Inner::Socket)
    }

    #[inline]
    pub(crate) fn is_symlink(&self) -> bool {
        self.0 == Inner::Symlink
    }
}
