use crate::fs::{FileType, FileTypeExt, Metadata, OpenOptions, ReadDir, ReadDirInner};
use posish::fs::Entry;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::{ffi::OsStrExt, fs::MetadataExt};
use std::{
    ffi::{OsStr, OsString},
    fmt, fs, io,
};

pub(crate) struct DirEntryInner {
    pub(super) posish: Entry,
    pub(super) read_dir: ReadDirInner,
}

impl DirEntryInner {
    #[inline]
    pub(crate) fn open(&self, options: &OpenOptions) -> io::Result<fs::File> {
        self.read_dir.open(self.file_name_bytes(), options)
    }

    #[inline]
    pub(crate) fn metadata(&self) -> io::Result<Metadata> {
        self.read_dir.metadata(self.file_name_bytes())
    }

    #[inline]
    pub(crate) fn remove_file(&self) -> io::Result<()> {
        self.read_dir.remove_file(self.file_name_bytes())
    }

    #[inline]
    pub(crate) fn read_dir(&self) -> io::Result<ReadDir> {
        self.read_dir.read_dir(self.file_name_bytes())
    }

    #[inline]
    pub(crate) fn remove_dir(&self) -> io::Result<()> {
        self.read_dir.remove_dir(self.file_name_bytes())
    }

    #[inline]
    pub(crate) fn file_type(&self) -> io::Result<FileType> {
        Ok(match self.posish.file_type() {
            posish::fs::FileType::Directory => FileType::dir(),
            posish::fs::FileType::RegularFile => FileType::file(),
            posish::fs::FileType::Symlink => FileType::ext(FileTypeExt::symlink()),
            posish::fs::FileType::Fifo => FileType::ext(FileTypeExt::fifo()),
            posish::fs::FileType::Socket => FileType::ext(FileTypeExt::socket()),
            posish::fs::FileType::CharacterDevice => FileType::ext(FileTypeExt::char_device()),
            posish::fs::FileType::BlockDevice => FileType::ext(FileTypeExt::block_device()),
            posish::fs::FileType::Unknown => FileType::unknown(),
        })
    }

    #[inline]
    pub(crate) fn file_name(&self) -> OsString {
        self.file_name_bytes().to_os_string()
    }

    #[inline]
    pub(crate) fn ino(&self) -> u64 {
        self.posish.ino()
    }

    #[inline]
    pub(crate) fn is_same_file(&self, metadata: &Metadata) -> io::Result<bool> {
        Ok(self.ino() == metadata.ino() && self.metadata()?.dev() == metadata.dev())
    }

    fn file_name_bytes(&self) -> &OsStr {
        OsStr::from_bytes(self.posish.file_name().to_bytes())
    }
}

impl fmt::Debug for DirEntryInner {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DirEntry").field(&self.file_name()).finish()
    }
}
