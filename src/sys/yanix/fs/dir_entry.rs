#![allow(dead_code)] // TODO: When more things are implemented, remove these.

use crate::fs::{Dir, FileType, Metadata};
use std::{ffi, io, path::PathBuf};

pub(crate) struct DirEntry<'dir> {
    dir: &'dir Dir,
    name: PathBuf,
    file_type: FileType,
    ino: u64,
}

impl<'dir> DirEntry<'dir> {
    fn new(dir: &'dir Dir, name: PathBuf, file_type: FileType, ino: u64) -> Self {
        Self {
            dir,
            name,
            file_type,
            ino,
        }
    }

    pub(crate) fn metadata(&self) -> io::Result<Metadata> {
        self.dir.metadata(&self.name)
    }

    pub(crate) fn file_type(&self) -> io::Result<FileType> {
        Ok(self.file_type)
    }

    pub(crate) fn file_name(&self) -> ffi::OsString {
        self.name.clone().into_os_string()
    }
}

impl<'dir> std::os::unix::fs::DirEntryExt for DirEntry<'dir> {
    fn ino(&self) -> u64 {
        self.ino
    }
}
