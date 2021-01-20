#![allow(clippy::useless_conversion)]

use std::{fs, io};

#[derive(Debug, Clone)]
pub(crate) struct MetadataExt {
    file_attributes: u32,
    creation_time: u64,
    last_access_time: u64,
    last_write_time: u64,
    file_size: u64,
    volume_serial_number: Option<u32>,
    number_of_links: Option<u32>,
    file_index: Option<u64>,
}

impl MetadataExt {
    /// Constructs a new instance of `Self` from the given `std::fs::File` and
    /// `std::fs::Metadata`.
    #[inline]
    pub(crate) fn from(file: &fs::File, std: &fs::Metadata) -> io::Result<Self> {
        #[cfg(windows_by_handle)]
        let (volume_serial_number, number_of_links, file_index) = {
            use std::os::windows::fs::MetadataExt;
            (
                std.volume_serial_number(),
                std.number_of_links(),
                std.file_index(),
            )
        };

        #[cfg(not(windows_by_handle))]
        let (volume_serial_number, number_of_links, file_index) = {
            let fileinfo = winx::file::get_fileinfo(file)?;
            (
                Some(fileinfo.dwVolumeSerialNumber),
                Some(fileinfo.nNumberOfLinks),
                Some(
                    (u64::from(fileinfo.nFileIndexHigh) << 32) | u64::from(fileinfo.nFileIndexLow),
                ),
            )
        };

        Ok(Self::from_parts(
            std,
            volume_serial_number,
            number_of_links,
            file_index,
        ))
    }

    /// Constructs a new instance of `Self` from the given `std::fs::Metadata`.
    ///
    /// As with the comments in [`std::fs::Metadata::volume_serial_number`] and
    /// nearby functions, some fields of the resulting metadata will be `None`.
    ///
    /// [`std::fs::Metadata::volume_serial_number`]: https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html#tymethod.volume_serial_number
    #[inline]
    pub(crate) fn from_just_metadata(std: &fs::Metadata) -> Self {
        Self::from_parts(std, None, None, None)
    }

    #[inline]
    fn from_parts(
        std: &fs::Metadata,
        volume_serial_number: Option<u32>,
        number_of_links: Option<u32>,
        file_index: Option<u64>,
    ) -> Self {
        use std::os::windows::fs::MetadataExt;
        Self {
            file_attributes: std.file_attributes(),
            creation_time: std.creation_time(),
            last_access_time: std.last_access_time(),
            last_write_time: std.last_write_time(),
            file_size: std.file_size(),
            volume_serial_number,
            number_of_links,
            file_index,
        }
    }

    /// Determine if `self` and `other` refer to the same inode on the same device.
    #[cfg(windows_by_handle)]
    pub(crate) fn is_same_file(&self, other: &Self) -> bool {
        // From [MSDN]:
        // The identifier (low and high parts) and the volume serial number
        // uniquely identify a file on a single computer. To determine whether
        // two open handles represent the same file, combine the identifier
        // and the volume serial number for each file and compare them.
        // [MSDN]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-by_handle_file_information
        let self_vsn = self
            .volume_serial_number
            .expect("could extract volume serial number of `self`");
        let other_vsn = other
            .volume_serial_number
            .expect("could extract volume serial number of `other`");
        let self_file_index = self.file_index.expect("could extract file index `self`");
        let other_file_index = other.file_index.expect("could extract file index `other`");
        self_vsn == other_vsn && self_file_index == other_file_index
    }

    /// `MetadataExt` requires nightly to be implemented, but we sometimes
    /// just need the file attributes.
    #[inline]
    pub(crate) fn file_attributes(&self) -> u32 {
        self.file_attributes
    }
}

#[cfg(windows_by_handle)]
impl std::os::windows::fs::MetadataExt for MetadataExt {
    #[inline]
    fn file_attributes(&self) -> u32 {
        self.file_attributes
    }

    #[inline]
    fn creation_time(&self) -> u64 {
        self.creation_time
    }

    #[inline]
    fn last_access_time(&self) -> u64 {
        self.last_access_time
    }

    #[inline]
    fn last_write_time(&self) -> u64 {
        self.last_write_time
    }

    #[inline]
    fn file_size(&self) -> u64 {
        self.file_size
    }

    #[inline]
    fn volume_serial_number(&self) -> Option<u32> {
        self.volume_serial_number
    }

    #[inline]
    fn number_of_links(&self) -> Option<u32> {
        self.number_of_links
    }

    #[inline]
    fn file_index(&self) -> Option<u64> {
        self.file_index
    }
}

#[cfg(all(windows, not(windows_by_handle)))]
#[doc(hidden)]
impl crate::fs::_WindowsByHandle for crate::fs::Metadata {
    #[inline]
    unsafe fn volume_serial_number(&self) -> Option<u32> {
        self.ext.volume_serial_number
    }

    #[inline]
    unsafe fn number_of_links(&self) -> Option<u32> {
        self.ext.number_of_links
    }

    #[inline]
    unsafe fn file_index(&self) -> Option<u64> {
        self.ext.file_index
    }
}
