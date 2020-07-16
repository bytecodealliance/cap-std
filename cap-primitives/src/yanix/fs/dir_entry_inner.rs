use crate::fs::{FileType, FileTypeExt, Metadata, ReadDirInner};
use std::{
    ffi::{OsStr, OsString},
    fmt, io,
    os::unix::ffi::{OsStrExt, OsStringExt},
};
use yanix::dir::{Entry, EntryExt};

pub(crate) struct DirEntryInner {
    pub(crate) yanix: Entry,
    pub(crate) read_dir: ReadDirInner,
}

impl DirEntryInner {
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.read_dir
            .metadata(OsStr::from_bytes(self.yanix.file_name().to_bytes()))
    }

    #[inline]
    pub fn file_type(&self) -> FileType {
        match self.yanix.file_type() {
            yanix::file::FileType::Directory => FileType::dir(),
            yanix::file::FileType::RegularFile => FileType::file(),
            yanix::file::FileType::Symlink => FileType::symlink(),
            yanix::file::FileType::Fifo => FileType::ext(FileTypeExt::fifo()),
            yanix::file::FileType::Socket => FileType::ext(FileTypeExt::socket()),
            yanix::file::FileType::CharacterDevice => FileType::ext(FileTypeExt::char_device()),
            yanix::file::FileType::BlockDevice => FileType::ext(FileTypeExt::block_device()),
            yanix::file::FileType::Unknown => FileType::unknown(),
        }
    }

    #[inline]
    pub fn file_name(&self) -> OsString {
        OsString::from_vec(self.yanix.file_name().to_bytes().to_vec())
    }

    #[inline]
    pub fn ino(&self) -> u64 {
        self.yanix.ino()
    }
}

impl fmt::Debug for DirEntryInner {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DirEntry").field(&self.file_name()).finish()
    }
}
