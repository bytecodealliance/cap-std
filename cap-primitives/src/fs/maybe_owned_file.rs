use std::{fs, io, mem};
#[cfg(debug_assertions)]
use {crate::fs::get_path, std::path::PathBuf};

enum Inner<'borrow> {
    Owned(fs::File),
    Borrowed(&'borrow fs::File),
}

pub(crate) struct MaybeOwnedFile<'borrow> {
    inner: Inner<'borrow>,

    #[cfg(debug_assertions)]
    path: Option<PathBuf>,
}

impl<'borrow> MaybeOwnedFile<'borrow> {
    /// Constructs a new `ManuallyOwnedFile` which is not owned.
    pub(crate) fn borrowed(file: &'borrow fs::File) -> Self {
        #[cfg(debug_assertions)]
        let path = get_path(file);

        Self {
            inner: Inner::Borrowed(file),

            #[cfg(debug_assertions)]
            path,
        }
    }

    /// Set this `MaybeOwnedFile` to a new owned file which is from a subtree
    /// of the current file. Return a `MaybeOwnedFile` representing the previous
    /// state.
    pub(crate) fn descend_to(&mut self, to: fs::File) -> Self {
        #[cfg(debug_assertions)]
        let path = self.path.clone();

        // TODO: This is a racy check, though it is useful for testing and fuzzing.
        #[cfg(debug_assertions)]
        if let Some(to_path) = get_path(&to) {
            if let Some(current_path) = &self.path {
                assert!(
                    to_path.starts_with(current_path),
                    "attempted to descend from {:?} to {:?}",
                    to_path.display(),
                    current_path.display()
                );
            }
            self.path = Some(to_path);
        }

        Self {
            inner: mem::replace(&mut self.inner, Inner::Owned(to)),

            #[cfg(debug_assertions)]
            path,
        }
    }

    pub(crate) fn into_file(self) -> io::Result<fs::File> {
        match self.inner {
            Inner::Owned(file) => Ok(file),
            Inner::Borrowed(file) => file.try_clone(),
        }
    }
}

impl<'borrow> AsRef<fs::File> for MaybeOwnedFile<'borrow> {
    fn as_ref(&self) -> &fs::File {
        match &self.inner {
            Inner::Owned(f) => f,
            Inner::Borrowed(f) => f,
        }
    }
}
