#![allow(unsafe_code)]

use std::{fs, io, iter, os::windows::ffi::OsStrExt, path::Path, ptr};

use windows_sys::Win32::{
    Foundation::{LocalFree, FALSE},
    Security::{
        Authorization::{GetNamedSecurityInfoW, SetNamedSecurityInfoW, SE_FILE_OBJECT},
        ACL, DACL_SECURITY_INFORMATION, GROUP_SECURITY_INFORMATION, OWNER_SECURITY_INFORMATION,
        PSECURITY_DESCRIPTOR, PSID,
    },
    Storage::FileSystem::{MoveFileExW, MOVEFILE_COPY_ALLOWED},
};

use super::get_path::concatenate;

pub(crate) fn rename_excl_unchecked(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    let old_full_path: Vec<u16> = concatenate(old_start, old_path)?
        .into_os_string()
        .encode_wide()
        .into_iter()
        .chain(iter::once(0u16))
        .collect();
    let new_full_path: Vec<u16> = concatenate(new_start, new_path)?
        .into_os_string()
        .encode_wide()
        .into_iter()
        .chain(iter::once(0u16))
        .collect();

    // Save permissions in case file will be moved across volumes
    // https://learn.microsoft.com/en-us/windows/win32/api/aclapi/nf-aclapi-getnamedsecurityinfoa
    // https://learn.microsoft.com/en-us/windows/win32/secauthz/security-information
    let mut owner: PSID = ptr::null_mut();
    let mut group: PSID = ptr::null_mut();
    let mut dacl: *mut ACL = ptr::null_mut();
    // According to the docs, the pointers above are pointers into the PSECURITY_DESCRIPTOR
    // struct, so `security` should only be freed after using `owner`, `group`, and `dacl`
    let mut security: PSECURITY_DESCRIPTOR = ptr::null_mut();

    let move_result = unsafe {
        // SAFETY: `old_full_path` is a valid pointer to a NUL terminated wide string
        GetNamedSecurityInfoW(
            old_full_path.as_ptr(),
            SE_FILE_OBJECT,
            OWNER_SECURITY_INFORMATION | GROUP_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
            &mut owner,
            &mut group,
            &mut dacl,
            ptr::null_mut(),
            &mut security,
        )
    };

    // Set/GetNamedSecurityInfoW return the error code directly rather than a bool
    if move_result != 0 {
        return Err(io::Error::from_raw_os_error(move_result as i32));
    }

    unsafe {
        // SAFETY:
        // * `concatenate` calls `get_path` which calls `encode_wide` so we have a wide string
        // * Both paths are NUL terminated above
        if MoveFileExW(
            old_full_path.as_ptr(),
            new_full_path.as_ptr(),
            MOVEFILE_COPY_ALLOWED,
        ) == FALSE
        {
            LocalFree(security);
            return Err(io::Error::last_os_error());
        }
    }

    let set_result = unsafe {
        SetNamedSecurityInfoW(
            new_full_path.as_ptr(),
            SE_FILE_OBJECT,
            OWNER_SECURITY_INFORMATION | GROUP_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
            owner,
            group,
            dacl,
            ptr::null_mut(),
        )
    };

    // SAFETY: `security` isn't used after `SetNamedSecurityInfoW` above.
    unsafe {
        LocalFree(security);
    }

    if set_result != 0 {
        Err(io::Error::from_raw_os_error(set_result as i32))
    } else {
        Ok(())
    }
}
