use cap_primitives::fs::is_read_write;
use std::io;
use unsafe_io::AsUnsafeFile;

/// A trait for the `is_read_write` function for `File` types.
///
/// This is only implemented for `File` types; for arbitrary I/O handles, use
/// [`system_interface::io::IsReadWrite`] instead.
///
/// [`system_interface::io::IsReadWrite`]: https://docs.rs/system-interface/latest/system_interface/io/trait.ReadReady.html
pub trait IsReadWrite {
    /// Test whether the given file is readable and/or writable.
    fn is_read_write(&self) -> io::Result<(bool, bool)>;
}

impl IsReadWrite for std::fs::File {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        is_read_write(self)
    }
}

#[cfg(all(feature = "std"))]
impl IsReadWrite for cap_std::fs::File {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        is_read_write(&self.as_file_view())
    }
}

#[cfg(all(feature = "std", feature = "fs_utf8"))]
impl IsReadWrite for cap_std::fs_utf8::File {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        is_read_write(&self.as_file_view())
    }
}

#[cfg(all(feature = "async_std"))]
impl IsReadWrite for async_std::fs::File {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        is_read_write(&self.as_file_view())
    }
}

#[cfg(all(feature = "async_std"))]
impl IsReadWrite for cap_async_std::fs::File {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        is_read_write(&self.as_file_view())
    }
}

#[cfg(all(feature = "async_std", feature = "fs_utf8"))]
impl IsReadWrite for cap_async_std::fs_utf8::File {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        is_read_write(&self.as_file_view())
    }
}
