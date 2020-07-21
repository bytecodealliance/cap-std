use std::{fmt, fs, io, mem, ops::Deref};
#[cfg(debug_assertions)]
use {crate::fs::get_path, std::path::PathBuf};

enum Inner<'borrow> {
    Owned(fs::File),
    Borrowed(&'borrow fs::File),
}

/// Several places in the code need to be able to handle either owned or
/// borrowed `std::fs::File`s. Cloning a `File` to let them always have an
/// owned `File` is expensive and fallble, so use use this `struct` to hold
/// either one, and implement `Deref` to allow them to be handled in a
/// uniform way.
///
/// This is similar to `Cow`, except without the copy-on-write part ;-).
/// `Cow` requires a `Clone` implementation, which `File` doesn't have, and
/// most users of this type don't need copy-on-write behavior.
///
/// And, this type has the special `descend_to`, which just does an
/// assignment, but also some useful assertion checks.
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

    /// Produce an owned `File`. This uses `try_clone` (dup) to convert a
    /// borrowed `File` to an owned one.
    pub(crate) fn into_file(self) -> io::Result<fs::File> {
        match self.inner {
            Inner::Owned(file) => Ok(file),
            Inner::Borrowed(file) => file.try_clone(),
        }
    }
}

impl<'borrow> Deref for MaybeOwnedFile<'borrow> {
    type Target = fs::File;

    fn deref(&self) -> &Self::Target {
        match &self.inner {
            Inner::Owned(f) => f,
            Inner::Borrowed(f) => f,
        }
    }
}

impl<'borrow> fmt::Debug for MaybeOwnedFile<'borrow> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}
