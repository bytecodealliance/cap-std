use crate::fs::{get_access_mode, get_flags_and_attributes, OpenOptions};
use io_lifetimes::AsHandle;
use std::{fs, io};
use windows_sys::Win32::Storage::FileSystem::{
    FILE_FLAG_DELETE_ON_CLOSE, FILE_FLAG_WRITE_THROUGH, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
    SECURITY_CONTEXT_TRACKING, SECURITY_DELEGATION, SECURITY_EFFECTIVE_ONLY,
    SECURITY_IDENTIFICATION, SECURITY_IMPERSONATION,
};
use windows_sys::Win32::System::SystemServices::{GENERIC_READ, GENERIC_WRITE};
use winx::file::{AccessMode, Flags};

/// Implementation of `reopen`.
pub(crate) fn reopen_impl(file: &fs::File, options: &OpenOptions) -> io::Result<fs::File> {
    let old_access_mode = winx::file::query_access_information(file.as_handle())?;
    let new_access_mode = get_access_mode(options)?;
    let flags = get_flags_and_attributes(options);

    let new_access_mode = AccessMode::from_bits(new_access_mode).unwrap();
    let flags = Flags::from_bits(flags).unwrap();

    // Disallow custom access modes might imply or require more access than we
    // have.
    if new_access_mode
        .intersects(AccessMode::from_bits(GENERIC_WRITE | FILE_GENERIC_WRITE).unwrap())
        && !old_access_mode
            .intersects(AccessMode::from_bits(GENERIC_WRITE | FILE_GENERIC_WRITE).unwrap())
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Can't reopen file",
        ));
    }
    if new_access_mode.intersects(AccessMode::from_bits(GENERIC_READ | FILE_GENERIC_READ).unwrap())
        && !old_access_mode
            .intersects(AccessMode::from_bits(GENERIC_READ | FILE_GENERIC_READ).unwrap())
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Can't reopen file",
        ));
    }

    // Disallow custom flags which might imply or require more access than we have.
    if flags
        .intersects(Flags::from_bits(FILE_FLAG_DELETE_ON_CLOSE | FILE_FLAG_WRITE_THROUGH).unwrap())
        && !old_access_mode
            .intersects(AccessMode::from_bits(GENERIC_WRITE | FILE_GENERIC_WRITE).unwrap())
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Can't reopen file",
        ));
    }

    // For now, disallow all non-anonymous security modes.
    /*
    if flags.intersects(
        Flags::from_bits(
            SECURITY_CONTEXT_TRACKING
                | SECURITY_DELEGATION
                | SECURITY_EFFECTIVE_ONLY
                | SECURITY_IDENTIFICATION
                | SECURITY_IMPERSONATION,
        )
        .unwrap(),
    ) {
        return Err(io::Error::new(io::ErrorKind::Other, "Can't reopen file"));
    }
    */
    // And for now, do the bit tests manually.
    if (flags.bits()
        & (SECURITY_CONTEXT_TRACKING
            | SECURITY_DELEGATION
            | SECURITY_EFFECTIVE_ONLY
            | SECURITY_IDENTIFICATION
            | SECURITY_IMPERSONATION))
        != 0
    {
        return Err(io::Error::new(io::ErrorKind::Other, "Can't reopen file"));
    }

    winx::file::reopen_file(file.as_handle(), new_access_mode, flags)
}
