use cap_primitives::fs::Metadata;
#[cfg(windows)]
use cap_primitives::fs::_WindowsDirEntryExt;
use std::io;

/// Extension trait for `DirEntry`.
pub trait DirEntryExt {
    /// Return the full metadata, which on Windows includes the optional
    /// values.
    fn full_metadata(&self) -> io::Result<Metadata>;
}

#[cfg(not(windows))]
impl DirEntryExt for std::fs::DirEntry {
    #[inline]
    fn full_metadata(&self) -> io::Result<Metadata> {
        self.metadata().map(Metadata::from_just_metadata)
    }
}

#[cfg(all(not(windows), feature = "std"))]
impl DirEntryExt for cap_std::fs::DirEntry {
    #[inline]
    fn full_metadata(&self) -> io::Result<Metadata> {
        self.metadata()
    }
}

#[cfg(all(windows, feature = "std"))]
impl DirEntryExt for cap_std::fs::DirEntry {
    #[inline]
    fn full_metadata(&self) -> io::Result<Metadata> {
        unsafe { _WindowsDirEntryExt::full_metadata(self) }
    }
}

#[cfg(all(not(windows), feature = "async_std"))]
impl DirEntryExt for cap_async_std::fs::DirEntry {
    #[inline]
    fn full_metadata(&self) -> io::Result<Metadata> {
        self.metadata()
    }
}

#[cfg(all(windows, feature = "async_std"))]
impl DirEntryExt for cap_async_std::fs::DirEntry {
    #[inline]
    fn full_metadata(&self) -> io::Result<Metadata> {
        unsafe { _WindowsDirEntryExt::full_metadata(self) }
    }
}

#[cfg(all(not(windows), feature = "std", feature = "fs_utf8"))]
impl DirEntryExt for cap_std::fs_utf8::DirEntry {
    #[inline]
    fn full_metadata(&self) -> io::Result<Metadata> {
        self.metadata()
    }
}

#[cfg(all(windows, feature = "std", feature = "fs_utf8"))]
impl DirEntryExt for cap_std::fs_utf8::DirEntry {
    #[inline]
    fn full_metadata(&self) -> io::Result<Metadata> {
        unsafe { _WindowsDirEntryExt::full_metadata(self) }
    }
}

#[cfg(all(not(windows), feature = "async_std", feature = "fs_utf8"))]
impl DirEntryExt for cap_async_std::fs_utf8::DirEntry {
    #[inline]
    fn full_metadata(&self) -> io::Result<Metadata> {
        self.metadata()
    }
}

#[cfg(all(windows, feature = "async_std", feature = "fs_utf8"))]
impl DirEntryExt for cap_async_std::fs_utf8::DirEntry {
    #[inline]
    fn full_metadata(&self) -> io::Result<Metadata> {
        unsafe { _WindowsDirEntryExt::full_metadata(self) }
    }
}
