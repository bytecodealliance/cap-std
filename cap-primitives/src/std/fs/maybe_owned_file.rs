use std::{fs, io};

pub(crate) enum MaybeOwnedFile<'borrow> {
    Owned(fs::File),
    Borrowed(&'borrow fs::File),
}

impl<'borrow> MaybeOwnedFile<'borrow> {
    pub(crate) fn as_file(&'borrow self) -> &'borrow fs::File {
        match self {
            MaybeOwnedFile::Owned(f) => f,
            MaybeOwnedFile::Borrowed(f) => f,
        }
    }

    pub(crate) fn into_file(self) -> io::Result<fs::File> {
        match self {
            MaybeOwnedFile::Owned(file) => Ok(file),
            MaybeOwnedFile::Borrowed(file) => file.try_clone(),
        }
    }
}
