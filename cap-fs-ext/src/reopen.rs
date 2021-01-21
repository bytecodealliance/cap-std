use cap_primitives::fs::{reopen, OpenOptions};
use std::io;
#[cfg(any(feature = "std", feature = "async_std"))]
use unsafe_io::AsUnsafeFile;
#[cfg(feature = "async_std")]
use unsafe_io::{FromUnsafeFile, IntoUnsafeFile};

/// A trait for the `reopen` function.
pub trait Reopen {
    /// Re-open a file, producing a new independent handle.
    ///
    /// This operation isn't supported by all operating systems in all
    /// circumstances, or in some operating systems in any circumstances, so it
    /// may return an `io::ErrorKind::Other` error if the file cannot be
    /// reopened.
    ///
    /// For files that aren't deleted, it's supported mostly-reliably on Linux
    /// and Windows and somewhat-reliably on Darwin. Beyond that, it works
    /// reliably on terminal device files and (slowly) on directories. It's not
    /// possible to implement this operation with POSIX APIs alone (short of
    /// traversing the entire filesystem), so further support will depend on
    /// operating systems providing OS-specific APIs.
    ///
    /// This function takes an `OpenOptions`, however it does not acquire new
    /// permissions that the original handle lacks.
    fn reopen(&self, options: &OpenOptions) -> io::Result<Self>
    where
        Self: Sized;
}

impl Reopen for std::fs::File {
    #[inline]
    fn reopen(&self, options: &OpenOptions) -> io::Result<Self> {
        reopen(self, options)
    }
}

#[cfg(feature = "std")]
impl Reopen for cap_std::fs::File {
    #[inline]
    fn reopen(&self, options: &OpenOptions) -> io::Result<Self> {
        let file = reopen(&AsUnsafeFile::as_file_view(self), options)?;
        Ok(unsafe { Self::from_std(file) })
    }
}

#[cfg(all(feature = "std", feature = "fs_utf8"))]
impl Reopen for cap_std::fs_utf8::File {
    #[inline]
    fn reopen(&self, options: &OpenOptions) -> io::Result<Self> {
        let file = reopen(&self.as_file_view(), options)?;
        Ok(unsafe { Self::from_std(file) })
    }
}

#[cfg(feature = "async_std")]
impl Reopen for async_std::fs::File {
    #[inline]
    fn reopen(&self, options: &OpenOptions) -> io::Result<Self> {
        let file = reopen(&self.as_file_view(), options)?;
        let unsafe_file = file.into_unsafe_file();
        Ok(unsafe { async_std::fs::File::from_unsafe_file(unsafe_file) })
    }
}

#[cfg(feature = "async_std")]
impl Reopen for cap_async_std::fs::File {
    #[inline]
    fn reopen(&self, options: &OpenOptions) -> io::Result<Self> {
        let file = reopen(&self.as_file_view(), options)?;
        let unsafe_file = file.into_unsafe_file();
        Ok(unsafe { Self::from_std(async_std::fs::File::from_unsafe_file(unsafe_file)) })
    }
}

#[cfg(all(feature = "async_std", feature = "fs_utf8"))]
impl Reopen for cap_async_std::fs_utf8::File {
    #[inline]
    fn reopen(&self, options: &OpenOptions) -> io::Result<Self> {
        let file = reopen(&self.as_file_view(), options)?;
        let unsafe_file = file.into_unsafe_file();
        Ok(unsafe { Self::from_std(async_std::fs::File::from_unsafe_file(unsafe_file)) })
    }
}
