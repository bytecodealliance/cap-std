/// Extension trait for `Dir`.
pub trait MetadataExt {
    /// Returns the ID of the device containing the file.
    ///
    /// This corresponds to [`std::os::unix::fs::MetadataExt::dev`], except
    /// that it's supported on Windows platforms as well.
    ///
    /// [`std::os::unix::fs::MetadataExt::dev`]: https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html#tymethod.dev
    fn dev(&self) -> u64;

    /// Returns the inode number.
    ///
    /// This corresponds to [`std::os::unix::fs::MetadataExt::ino`], except
    /// that it's supported on Windows platforms as well.
    ///
    /// FIXME: On Windows' `ReFS`, file identifiers are 128-bit.
    ///
    /// [`std::os::unix::fs::MetadataExt::ino`]: https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html#tymethod.ino
    fn ino(&self) -> u64;

    /// Returns the number of hard links pointing to this file.
    ///
    /// This corresponds to [`std::os::unix::fs::MetadataExt::nlink`], except
    /// that it's supported on Windows platforms as well.
    ///
    /// [`std::os::unix::fs::MetadataExt::nlink`]: https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html#tymethod.nlink
    fn nlink(&self) -> u64;
}

#[cfg(all(windows, feature = "windows_by_handle"))]
use std::os::windows::fs::MetadataExt as _WindowsByHandle;

#[cfg(all(windows, not(feature = "windows_by_handle")))]
use cap_primitives::fs::_WindowsByHandle;

#[cfg(all(windows, feature = "windows_by_handle"))]
impl MetadataExt for std::fs::Metadata {
    #[inline]
    fn dev(&self) -> u64 {
        _WindowsByHandle::volume_serial_number(self)
            .expect("`dev` depends on a Metadata constructed from an open `File`")
            .into()
    }

    #[inline]
    fn ino(&self) -> u64 {
        _WindowsByHandle::file_index(self)
            .expect("`ino` depends on a Metadata constructed from an open `File`")
    }

    #[inline]
    fn nlink(&self) -> u64 {
        _WindowsByHandle::number_of_links(self)
            .expect("`nlink` depends on a Metadata constructed from an open `File`")
            .into()
    }
}

#[cfg(all(windows, any(feature = "std", feature = "async_std")))]
impl MetadataExt for cap_std::fs::Metadata {
    fn dev(&self) -> u64 {
        unsafe {
            _WindowsByHandle::volume_serial_number(self)
                .expect("`dev` depends on a Metadata constructed from an open `File`")
                .into()
        }
    }

    #[inline]
    fn ino(&self) -> u64 {
        unsafe {
            _WindowsByHandle::file_index(self)
                .expect("`ino` depends on a Metadata constructed from an open `File`")
        }
    }

    #[inline]
    fn nlink(&self) -> u64 {
        unsafe {
            _WindowsByHandle::number_of_links(self)
                .expect("`nlink` depends on a Metadata constructed from an open `File`")
                .into()
        }
    }
}

#[cfg(all(not(windows), any(feature = "std", feature = "async_std")))]
impl MetadataExt for cap_std::fs::Metadata {
    #[inline]
    fn dev(&self) -> u64 {
        std::os::unix::fs::MetadataExt::dev(self)
    }

    #[inline]
    fn ino(&self) -> u64 {
        std::os::unix::fs::MetadataExt::ino(self)
    }

    #[inline]
    fn nlink(&self) -> u64 {
        std::os::unix::fs::MetadataExt::nlink(self)
    }
}
