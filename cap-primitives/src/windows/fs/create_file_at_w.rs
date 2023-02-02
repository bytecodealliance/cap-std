#![allow(unsafe_code)]

use std::mem;
use std::os::windows::io::HandleOrInvalid;
use std::ptr::null_mut;
use windows_sys::core::PCWSTR;
use windows_sys::Win32::Foundation::{
    RtlNtStatusToDosError, SetLastError, ERROR_ALREADY_EXISTS, ERROR_FILE_EXISTS,
    ERROR_INVALID_PARAMETER, ERROR_PATH_NOT_FOUND, HANDLE, INVALID_HANDLE_VALUE, NTSTATUS,
    STATUS_OBJECT_NAME_COLLISION, SUCCESS, UNICODE_STRING,
};
use windows_sys::Win32::Security::{
    SECURITY_ATTRIBUTES, SECURITY_DYNAMIC_TRACKING, SECURITY_QUALITY_OF_SERVICE,
    SECURITY_STATIC_TRACKING,
};
use windows_sys::Win32::Storage::FileSystem::{
    NtCreateFile, CREATE_ALWAYS, CREATE_NEW, DELETE, FILE_ACCESS_FLAGS, FILE_ATTRIBUTE_ARCHIVE,
    FILE_ATTRIBUTE_COMPRESSED, FILE_ATTRIBUTE_DEVICE, FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_EA,
    FILE_ATTRIBUTE_ENCRYPTED, FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_INTEGRITY_STREAM,
    FILE_ATTRIBUTE_NORMAL, FILE_ATTRIBUTE_NOT_CONTENT_INDEXED, FILE_ATTRIBUTE_NO_SCRUB_DATA,
    FILE_ATTRIBUTE_OFFLINE, FILE_ATTRIBUTE_PINNED, FILE_ATTRIBUTE_READONLY,
    FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS, FILE_ATTRIBUTE_RECALL_ON_OPEN,
    FILE_ATTRIBUTE_REPARSE_POINT, FILE_ATTRIBUTE_SPARSE_FILE, FILE_ATTRIBUTE_SYSTEM,
    FILE_ATTRIBUTE_TEMPORARY, FILE_ATTRIBUTE_UNPINNED, FILE_ATTRIBUTE_VIRTUAL, FILE_CREATE,
    FILE_CREATION_DISPOSITION, FILE_FLAGS_AND_ATTRIBUTES, FILE_FLAG_BACKUP_SEMANTICS,
    FILE_FLAG_DELETE_ON_CLOSE, FILE_FLAG_NO_BUFFERING, FILE_FLAG_OVERLAPPED,
    FILE_FLAG_RANDOM_ACCESS, FILE_FLAG_SEQUENTIAL_SCAN, FILE_FLAG_WRITE_THROUGH, FILE_OPEN,
    FILE_OPEN_IF, FILE_OVERWRITE, FILE_OVERWRITE_IF, FILE_READ_ATTRIBUTES, FILE_SHARE_MODE,
    OPEN_ALWAYS, OPEN_EXISTING, SECURITY_CONTEXT_TRACKING, SECURITY_EFFECTIVE_ONLY,
    SECURITY_SQOS_PRESENT, SYNCHRONIZE, TRUNCATE_EXISTING,
};
use windows_sys::Win32::System::Kernel::{OBJ_CASE_INSENSITIVE, OBJ_INHERIT};
use windows_sys::Win32::System::WindowsProgramming::{
    RtlFreeUnicodeString, RtlInitUnicodeString, FILE_DELETE_ON_CLOSE, FILE_NON_DIRECTORY_FILE,
    FILE_NO_INTERMEDIATE_BUFFERING, FILE_OPENED, FILE_OPEN_FOR_BACKUP_INTENT, FILE_OVERWRITTEN,
    FILE_RANDOM_ACCESS, FILE_SEQUENTIAL_ONLY, FILE_SYNCHRONOUS_IO_NONALERT, FILE_WRITE_THROUGH,
    IO_STATUS_BLOCK, OBJECT_ATTRIBUTES,
};

// All currently known `FILE_ATTRIBUTE_*` constants, according to
// windows-sys' documentation.
const FILE_ATTRIBUTE_VALID_FLAGS: FILE_FLAGS_AND_ATTRIBUTES = FILE_ATTRIBUTE_EA
    | FILE_ATTRIBUTE_DEVICE
    | FILE_ATTRIBUTE_HIDDEN
    | FILE_ATTRIBUTE_NORMAL
    | FILE_ATTRIBUTE_PINNED
    | FILE_ATTRIBUTE_SYSTEM
    | FILE_ATTRIBUTE_ARCHIVE
    | FILE_ATTRIBUTE_OFFLINE
    | FILE_ATTRIBUTE_VIRTUAL
    | FILE_ATTRIBUTE_READONLY
    | FILE_ATTRIBUTE_UNPINNED
    | FILE_ATTRIBUTE_DIRECTORY
    | FILE_ATTRIBUTE_ENCRYPTED
    | FILE_ATTRIBUTE_TEMPORARY
    | FILE_ATTRIBUTE_COMPRESSED
    | FILE_ATTRIBUTE_SPARSE_FILE
    | FILE_ATTRIBUTE_NO_SCRUB_DATA
    | FILE_ATTRIBUTE_REPARSE_POINT
    | FILE_ATTRIBUTE_RECALL_ON_OPEN
    | FILE_ATTRIBUTE_INTEGRITY_STREAM
    | FILE_ATTRIBUTE_NOT_CONTENT_INDEXED
    | FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS;

#[allow(non_snake_case)]
pub unsafe fn CreateFileAtW(
    dir: HANDLE,
    lpfilename: PCWSTR,
    dwdesiredaccess: FILE_ACCESS_FLAGS,
    dwsharemode: FILE_SHARE_MODE,
    lpsecurityattributes: *const SECURITY_ATTRIBUTES,
    dwcreationdisposition: FILE_CREATION_DISPOSITION,
    dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
    _htemplatefile: HANDLE,
) -> HandleOrInvalid {
    // Check for a null or empty filename.
    if lpfilename.is_null() || *lpfilename == 0 {
        SetLastError(ERROR_PATH_NOT_FOUND);
        return HandleOrInvalid::from_raw_handle(INVALID_HANDLE_VALUE as _);
    }

    // Convert `dwcreationdisposition` to the `createdisposition` argument
    // to `NtCreateFile`. Do this before converting `lpfilename` so that
    // we can return without having to free anything.
    let createdisposition = match dwcreationdisposition {
        CREATE_NEW => FILE_CREATE,
        CREATE_ALWAYS => FILE_OVERWRITE_IF,
        OPEN_EXISTING => FILE_OPEN,
        OPEN_ALWAYS => FILE_OPEN_IF,
        TRUNCATE_EXISTING => FILE_OVERWRITE,
        _ => {
            SetLastError(ERROR_INVALID_PARAMETER);
            return HandleOrInvalid::from_raw_handle(INVALID_HANDLE_VALUE as _);
        }
    };

    // Convert `lpfilename` to a `UNICODE_STRING`. After this, we'll need to
    // call `RtlFreeUnicodeString` before returning.
    let mut unicode_string = mem::zeroed::<UNICODE_STRING>();
    RtlInitUnicodeString(&mut unicode_string, lpfilename);

    let mut handle = INVALID_HANDLE_VALUE;

    // Convert `dwdesiredaccess` and `dwflagsandattributes` to the
    // `desiredaccess` argument to `NtCreateFile`.
    let mut desiredaccess = dwdesiredaccess | SYNCHRONIZE | FILE_READ_ATTRIBUTES;
    if dwflagsandattributes & FILE_FLAG_DELETE_ON_CLOSE != 0 {
        desiredaccess |= DELETE;
    }

    // Compute `objectattributes`' `Attributes` field. Case-insensitive is
    // the expected behavior on Windows.
    let mut attributes = OBJ_CASE_INSENSITIVE as _;
    if !lpsecurityattributes.is_null() && (*lpsecurityattributes).bInheritHandle != 0 {
        attributes |= OBJ_INHERIT as u32;
    }

    // Compute the `objectattributes` argument to `NtCreateFile`.
    let mut objectattributes = mem::zeroed::<OBJECT_ATTRIBUTES>();
    objectattributes.Length = mem::size_of_val(&objectattributes) as _;
    objectattributes.RootDirectory = dir;
    objectattributes.ObjectName = &mut unicode_string;
    objectattributes.Attributes = attributes;
    if !lpsecurityattributes.is_null() {
        objectattributes.SecurityDescriptor = (*lpsecurityattributes).lpSecurityDescriptor;
    }

    // If needed, set `objectattributes`' `SecurityQualityOfService` field.
    let mut qos;
    if dwflagsandattributes & SECURITY_SQOS_PRESENT != 0 {
        qos = mem::zeroed::<SECURITY_QUALITY_OF_SERVICE>();
        qos.Length = mem::size_of_val(&qos) as _;
        qos.ImpersonationLevel = ((dwflagsandattributes >> 16) & 0x3) as _;
        qos.ContextTrackingMode = if dwflagsandattributes & SECURITY_CONTEXT_TRACKING != 0 {
            SECURITY_DYNAMIC_TRACKING
        } else {
            SECURITY_STATIC_TRACKING
        };
        qos.EffectiveOnly = ((dwflagsandattributes & SECURITY_EFFECTIVE_ONLY) != 0) as _;

        objectattributes.SecurityQualityOfService =
            (&mut qos as *mut SECURITY_QUALITY_OF_SERVICE).cast();
    }

    let mut iostatusblock = mem::zeroed::<IO_STATUS_BLOCK>();

    // Compute the `fileattributes` argument to `NtCreateFile`. Mask off
    // unrecognized flags.
    let fileattributes = dwflagsandattributes & FILE_ATTRIBUTE_VALID_FLAGS;

    // Compute the `createoptions` argument to `NtCreateFile`.
    let mut createoptions = 0;
    if dwflagsandattributes & FILE_FLAG_BACKUP_SEMANTICS == 0 {
        createoptions |= FILE_NON_DIRECTORY_FILE;
    } else {
        createoptions |= FILE_OPEN_FOR_BACKUP_INTENT;
    }
    if dwflagsandattributes & FILE_FLAG_DELETE_ON_CLOSE != 0 {
        createoptions |= FILE_DELETE_ON_CLOSE;
    }
    if dwflagsandattributes & FILE_FLAG_NO_BUFFERING != 0 {
        createoptions |= FILE_NO_INTERMEDIATE_BUFFERING;
    }
    if dwflagsandattributes & FILE_FLAG_OVERLAPPED == 0 {
        createoptions |= FILE_SYNCHRONOUS_IO_NONALERT;
    }
    if dwflagsandattributes & FILE_FLAG_RANDOM_ACCESS != 0 {
        createoptions |= FILE_RANDOM_ACCESS;
    }
    if dwflagsandattributes & FILE_FLAG_SEQUENTIAL_SCAN != 0 {
        createoptions |= FILE_SEQUENTIAL_ONLY;
    }
    if dwflagsandattributes & FILE_FLAG_WRITE_THROUGH != 0 {
        createoptions |= FILE_WRITE_THROUGH;
    }

    // Ok, we have what we need to call `NtCreateFile` now!
    let status = NtCreateFile(
        &mut handle,
        desiredaccess,
        &mut objectattributes,
        &mut iostatusblock,
        null_mut(),
        fileattributes,
        dwsharemode,
        createdisposition,
        createoptions,
        null_mut(),
        0,
    );

    // Check for errors.
    if nt_success(status) {
        handle = INVALID_HANDLE_VALUE;
        if status == STATUS_OBJECT_NAME_COLLISION {
            SetLastError(ERROR_FILE_EXISTS);
        } else {
            SetLastError(RtlNtStatusToDosError(status));
        }
    } else if (dwcreationdisposition == CREATE_ALWAYS
        && iostatusblock.Information == FILE_OVERWRITTEN as _)
        || (dwcreationdisposition == OPEN_ALWAYS && iostatusblock.Information == FILE_OPENED as _)
    {
        // Set `ERROR_ALREADY_EXISTS` according to the table for
        // `dwCreationDisposition` in the [`CreateFileW` docs].
        //
        // [`CreateFileW` docs]: https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew
        SetLastError(ERROR_ALREADY_EXISTS);
    } else {
        // Otherwise indicate that we succeeded.
        SetLastError(SUCCESS);
    }

    // Free `unicode_string`.
    RtlFreeUnicodeString(&mut unicode_string);

    HandleOrInvalid::from_raw_handle(handle as _)
}

// The following is derived from Rust's library/std/src/sys/windows/c.rs
// at revision 47e6304e325463bc6608a6f1eb61391fa36dd76a.

// Equivalent to the `NT_SUCCESS` C preprocessor macro.
// See: https://docs.microsoft.com/en-us/windows-hardware/drivers/kernel/using-ntstatus-values
pub fn nt_success(status: NTSTATUS) -> bool {
    status >= 0
}
