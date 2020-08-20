use std::io;

#[derive(Debug)]
pub(crate) enum OpenUncheckedError {
    Other(io::Error),
    Symlink(io::Error),
    NotFound(io::Error),
}

impl From<OpenUncheckedError> for io::Error {
    fn from(error: OpenUncheckedError) -> Self {
        match error {
            OpenUncheckedError::Other(err)
            | OpenUncheckedError::Symlink(err)
            | OpenUncheckedError::NotFound(err) => err,
        }
    }
}
