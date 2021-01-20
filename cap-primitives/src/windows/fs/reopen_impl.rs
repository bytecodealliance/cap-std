use crate::fs::OpenOptions;
use std::{
    fs, io,
    os::windows::io::{AsRawHandle, FromRawHandle},
};
use winapi::{
    shared::{minwindef::DWORD, winerror::ERROR_INVALID_PARAMETER},
    um::{
        winbase::{
            FILE_FLAG_DELETE_ON_CLOSE, FILE_FLAG_OPEN_REPARSE_POINT, FILE_FLAG_WRITE_THROUGH,
            SECURITY_CONTEXT_TRACKING, SECURITY_DELEGATION, SECURITY_EFFECTIVE_ONLY,
            SECURITY_IDENTIFICATION, SECURITY_IMPERSONATION,
        },
        winnt::{
            FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_WRITE_DATA, GENERIC_READ, GENERIC_WRITE,
        },
    },
};
use winx::file::{AccessMode, Flags};

/// Implementation of `reopen`.
pub(crate) fn reopen_impl(file: &fs::File, options: &OpenOptions) -> io::Result<fs::File> {
    let access_mode = get_access_mode(options)?;
    let flags = get_flags_and_attributes(options);

    let access_mode = AccessMode::from_bits(access_mode).unwrap();
    let flags = Flags::from_bits(flags).unwrap();

    // Disallow custom access modes might imply or require more access than we have.
    if !options.write
        && (access_mode
            .intersects(AccessMode::from_bits(GENERIC_WRITE | FILE_GENERIC_WRITE).unwrap()))
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Can't reopen file",
        ));
    }
    if !options.read
        && (access_mode
            .intersects(AccessMode::from_bits(GENERIC_READ | FILE_GENERIC_READ).unwrap()))
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Can't reopen file",
        ));
    }

    // Disallow custom flags which might imply or require more access than we have.
    if !options.write
        && (flags.intersects(
            Flags::from_bits(FILE_FLAG_DELETE_ON_CLOSE | FILE_FLAG_WRITE_THROUGH).unwrap(),
        ))
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

    let raw_handle = winx::file::reopen_file(file.as_raw_handle(), access_mode, flags)?;
    Ok(unsafe { fs::File::from_raw_handle(raw_handle) })
}

fn get_access_mode(options: &OpenOptions) -> io::Result<DWORD> {
    match (
        options.read,
        options.write,
        options.append,
        options.ext.access_mode,
    ) {
        (.., Some(mode)) => Ok(mode),
        (true, false, false, None) => Ok(GENERIC_READ),
        (false, true, false, None) => Ok(GENERIC_WRITE),
        (true, true, false, None) => Ok(GENERIC_READ | GENERIC_WRITE),
        (false, _, true, None) => Ok(FILE_GENERIC_WRITE & !FILE_WRITE_DATA),
        (true, _, true, None) => Ok(GENERIC_READ | (FILE_GENERIC_WRITE & !FILE_WRITE_DATA)),
        (false, false, false, None) => {
            Err(io::Error::from_raw_os_error(ERROR_INVALID_PARAMETER as i32))
        }
    }
}

fn get_flags_and_attributes(options: &OpenOptions) -> DWORD {
    options.ext.custom_flags
        | options.ext.attributes
        | options.ext.security_qos_flags
        | if options.create_new {
            FILE_FLAG_OPEN_REPARSE_POINT
        } else {
            0
        }
}
