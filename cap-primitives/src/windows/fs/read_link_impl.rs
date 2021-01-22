use crate::fs::{open, FollowSymlinks, OpenOptions};
use std::{
    ffi::OsString,
    fs, io,
    os::windows::{ffi::OsStringExt, fs::OpenOptionsExt, io::AsRawHandle},
    path::{Path, PathBuf},
    ptr, slice,
};

#[allow(non_snake_case)]
mod c {
    use winapi::ctypes::*;

    pub(super) use winapi::{
        shared::minwindef::*,
        um::{ioapiset::*, winbase::*, winioctl::*, winnt::*},
    };

    // Interfaces derived from Rust's
    // library/std/src/sys/windows/c.rs at revision
    // 108e90ca78f052c0c1c49c42a22c85620be19712.

    #[repr(C)]
    pub(super) struct REPARSE_DATA_BUFFER {
        pub(super) ReparseTag: c_uint,
        pub(super) ReparseDataLength: c_ushort,
        pub(super) Reserved: c_ushort,
        pub(super) rest: (),
    }

    #[repr(C)]
    pub(super) struct SYMBOLIC_LINK_REPARSE_BUFFER {
        pub(super) SubstituteNameOffset: c_ushort,
        pub(super) SubstituteNameLength: c_ushort,
        pub(super) PrintNameOffset: c_ushort,
        pub(super) PrintNameLength: c_ushort,
        pub(super) Flags: c_ulong,
        pub(super) PathBuffer: WCHAR,
    }

    #[repr(C)]
    pub struct MOUNT_POINT_REPARSE_BUFFER {
        pub(super) SubstituteNameOffset: c_ushort,
        pub(super) SubstituteNameLength: c_ushort,
        pub(super) PrintNameOffset: c_ushort,
        pub(super) PrintNameLength: c_ushort,
        pub(super) PathBuffer: WCHAR,
    }

    pub(super) const SYMLINK_FLAG_RELATIVE: DWORD = 0x00000001;
    pub(super) const MAXIMUM_REPARSE_DATA_BUFFER_SIZE: usize = 16 * 1024;
}

// Implementation derived from Rust's
// library/std/src/sys/windows/mod.rs at revision
// 108e90ca78f052c0c1c49c42a22c85620be19712.
fn cvt(i: i32) -> io::Result<i32> {
    if i == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(i)
    }
}

// Implementation derived from Rust's
// library/std/src/sys/windows/fs.rs at revision
// 108e90ca78f052c0c1c49c42a22c85620be19712.

/// *Unsandboxed* function similar to `read_link`, but which does not perform sandboxing.
pub(crate) fn read_link_impl(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    // Open the link with no access mode, instead of generic read.
    // By default FILE_LIST_DIRECTORY is denied for the junction "C:\Documents and Settings", so
    // this is needed for a common case.
    let mut opts = OpenOptions::new();
    opts.access_mode(0);
    opts.custom_flags(c::FILE_FLAG_OPEN_REPARSE_POINT | c::FILE_FLAG_BACKUP_SEMANTICS);
    opts.follow(FollowSymlinks::No);
    let file = open(start, path, &opts)?;
    read_link(&file)
}

fn reparse_point<'a>(
    file: &fs::File,
    space: &'a mut [u8; c::MAXIMUM_REPARSE_DATA_BUFFER_SIZE],
) -> io::Result<(c::DWORD, &'a c::REPARSE_DATA_BUFFER)> {
    unsafe {
        let mut bytes = 0;
        cvt({
            c::DeviceIoControl(
                file.as_raw_handle(),
                c::FSCTL_GET_REPARSE_POINT,
                ptr::null_mut(),
                0,
                space.as_mut_ptr() as *mut _,
                space.len() as c::DWORD,
                &mut bytes,
                ptr::null_mut(),
            )
        })?;
        Ok((bytes, &*(space.as_ptr() as *const c::REPARSE_DATA_BUFFER)))
    }
}

fn read_link(file: &fs::File) -> io::Result<PathBuf> {
    let mut space = [0_u8; c::MAXIMUM_REPARSE_DATA_BUFFER_SIZE];
    let (_bytes, buf) = reparse_point(file, &mut space)?;
    unsafe {
        let (path_buffer, subst_off, subst_len, relative) = match buf.ReparseTag {
            c::IO_REPARSE_TAG_SYMLINK => {
                let info: *const c::SYMBOLIC_LINK_REPARSE_BUFFER =
                    &buf.rest as *const _ as *const _;
                (
                    &(*info).PathBuffer as *const _ as *const u16,
                    (*info).SubstituteNameOffset / 2,
                    (*info).SubstituteNameLength / 2,
                    (*info).Flags & c::SYMLINK_FLAG_RELATIVE != 0,
                )
            }
            c::IO_REPARSE_TAG_MOUNT_POINT => {
                let info: *const c::MOUNT_POINT_REPARSE_BUFFER = &buf.rest as *const _ as *const _;
                (
                    &(*info).PathBuffer as *const _ as *const u16,
                    (*info).SubstituteNameOffset / 2,
                    (*info).SubstituteNameLength / 2,
                    false,
                )
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Unsupported reparse point type",
                ));
            }
        };
        let subst_ptr = path_buffer.offset(subst_off as isize);
        let mut subst = slice::from_raw_parts(subst_ptr, subst_len as usize);
        // Absolute paths start with an NT internal namespace prefix `\??\`
        // We should not let it leak through.
        if !relative && subst.starts_with(&[92u16, 63u16, 63u16, 92u16]) {
            subst = &subst[4..];
        }
        Ok(PathBuf::from(OsString::from_wide(subst)))
    }
}
