use crate::fs::{open_unchecked, OpenOptions};
use std::{fmt, fs, io, mem, ops::Deref, path::Component};
#[cfg(not(feature = "no_racy_asserts"))]
use {crate::fs::get_path, std::path::PathBuf};

enum Inner<'borrow> {
    Owned(fs::File),
    Borrowed(&'borrow fs::File),
}

/// Several places in the code need to be able to handle either owned or
/// borrowed `std::fs::File`s. Cloning a `File` to let them always have an owned
/// `File` is expensive and fallible, so use this `struct` to hold either one,
/// and implement `Deref` to allow them to be handled in a uniform way.
///
/// This is similar to `Cow`, except without the copy-on-write part ;-). `Cow`
/// requires a `Clone` implementation, which `File` doesn't have, and most users
/// of this type don't need copy-on-write behavior.
///
/// And, this type has the special `descend_to`, which just does an assignment,
/// but also some useful assertion checks.
pub(super) struct MaybeOwnedFile<'borrow> {
    inner: Inner<'borrow>,

    #[cfg(not(feature = "no_racy_asserts"))]
    path: Option<PathBuf>,
}

impl<'borrow> MaybeOwnedFile<'borrow> {
    /// Constructs a new `MaybeOwnedFile` which is not owned.
    pub(super) fn borrowed(file: &'borrow fs::File) -> Self {
        #[cfg(not(feature = "no_racy_asserts"))]
        let path = get_path(file);

        Self {
            inner: Inner::Borrowed(file),

            #[cfg(not(feature = "no_racy_asserts"))]
            path,
        }
    }

    /// Constructs a new `MaybeOwnedFile` which is owned.
    pub(super) fn owned(file: fs::File) -> Self {
        #[cfg(not(feature = "no_racy_asserts"))]
        let path = get_path(&file);

        Self {
            inner: Inner::Owned(file),

            #[cfg(not(feature = "no_racy_asserts"))]
            path,
        }
    }

    /// Set this `MaybeOwnedFile` to a new owned file which is from a subtree
    /// of the current file. Return a `MaybeOwnedFile` representing the previous
    /// state.
    pub(super) fn descend_to(&mut self, to: MaybeOwnedFile<'borrow>) -> Self {
        #[cfg(not(feature = "no_racy_asserts"))]
        let path = self.path.clone();

        #[cfg(not(feature = "no_racy_asserts"))]
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
            inner: mem::replace(&mut self.inner, to.inner),

            #[cfg(not(feature = "no_racy_asserts"))]
            path,
        }
    }

    /// Produce an owned `File`. This uses `open` on "." if needed to convert a
    /// borrowed `File` to an owned one.
    pub(super) fn into_file(self, options: &OpenOptions) -> io::Result<fs::File> {
        match self.inner {
            Inner::Owned(file) => Ok(file),
            Inner::Borrowed(file) => {
                // The only situation in which we'd be asked to produce an owned
                // `File` is when there's a need to open "." within a directory
                // to obtain a new handle.
                open_unchecked(file, Component::CurDir.as_ref(), options).map_err(Into::into)
            }
        }
    }

    /// Assuming `self` holds an owned `File`, return it.
    #[cfg_attr(windows, allow(dead_code))]
    pub(super) fn unwrap_owned(self) -> fs::File {
        match self.inner {
            Inner::Owned(file) => file,
            Inner::Borrowed(_) => panic!("expected owned file"),
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
