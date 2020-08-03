use crate::fs::{FileType, FileTypeExt, Metadata, OpenOptions, ReadDirInner};
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::fs::MetadataExt;
use std::{
    ffi::{OsStr, OsString},
    fmt, fs, io,
    os::unix::ffi::OsStrExt,
};
use yanix::dir::{Entry, EntryExt};

pub(crate) struct DirEntryInner {
    pub(crate) yanix: Entry,
    pub(crate) read_dir: ReadDirInner,
}

impl DirEntryInner {
    #[inline]
    pub fn open(&self, options: &OpenOptions) -> io::Result<fs::File> {
        self.read_dir.open(self.file_name_bytes(), options)
    }

    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.read_dir.metadata(self.file_name_bytes())
    }

    #[inline]
    pub fn remove_file(&self) -> io::Result<()> {
        self.read_dir.remove_file(self.file_name_bytes())
    }

    #[inline]
    pub fn remove_dir(&self) -> io::Result<()> {
        self.read_dir.remove_dir(self.file_name_bytes())
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
        self.file_name_bytes().to_os_string()
    }

    #[inline]
    pub fn ino(&self) -> u64 {
        self.yanix.ino()
    }

    #[inline]
    pub(crate) fn is_same_file(&self, metadata: &Metadata) -> io::Result<bool> {
        Ok(self.ino() == metadata.ino() && self.metadata()?.dev() == metadata.dev())
    }

    fn file_name_bytes(&self) -> &OsStr {
        OsStr::from_bytes(self.yanix.file_name().to_bytes())
    }
}

impl fmt::Debug for DirEntryInner {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DirEntry").field(&self.file_name()).finish()
    }
}
