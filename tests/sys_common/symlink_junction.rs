// Implementation derived from `symlink_junction` and related code in Rust's
// library/std/src/sys/windows/fs.rs at revision
// 108e90ca78f052c0c1c49c42a22c85620be19712.

// TODO: Replace this definition of `MAXIMUM_REPARSE_DATA_BUFFER_SIZE`
// once windows-sys has it.
//  - [windows-sys bug filed]
//  - [winapi doc]
//
// [windows-sys bug filed]: https://github.com/microsoft/windows-rs/issues/1823>
// [winapi doc]: https://docs.rs/winapi/latest/winapi/um/winnt/constant.MAXIMUM_REPARSE_DATA_BUFFER_SIZE.html
pub const MAXIMUM_REPARSE_DATA_BUFFER_SIZE: u32 = 16 * 1024; // 16_384u32

#[cfg(feature = "fs_utf8")]
use camino::Utf8Path;
use cap_std::fs::Dir;
use std::io;
use std::path::Path;

#[cfg(not(windows))]
#[allow(dead_code)]
pub fn symlink_junction<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst_dir: &Dir,
    dst: Q,
) -> io::Result<()> {
    dst_dir.symlink(src, dst)
}

#[cfg(windows)]
#[allow(dead_code)]
pub fn symlink_junction<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst_dir: &Dir,
    dst: Q,
) -> io::Result<()> {
    symlink_junction_inner(src.as_ref(), dst_dir, dst.as_ref())
}

#[cfg(feature = "fs_utf8")]
#[cfg(not(windows))]
#[allow(dead_code)]
pub fn symlink_junction_utf8<P: AsRef<Utf8Path>, Q: AsRef<Utf8Path>>(
    src: P,
    dst_dir: &cap_std::fs_utf8::Dir,
    dst: Q,
) -> io::Result<()> {
    dst_dir.symlink(src, dst)
}

#[cfg(feature = "fs_utf8")]
#[cfg(windows)]
#[allow(dead_code)]
pub fn symlink_junction_utf8<P: AsRef<Utf8Path>, Q: AsRef<Utf8Path>>(
    src: P,
    dst_dir: &cap_std::fs_utf8::Dir,
    dst: Q,
) -> io::Result<()> {
    symlink_junction_inner_utf8(src.as_ref(), dst_dir, dst.as_ref())
}

#[cfg(windows)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct REPARSE_MOUNTPOINT_DATA_BUFFER {
    pub ReparseTag: u32,
    pub ReparseDataLength: u32,
    pub Reserved: u16,
    pub ReparseTargetLength: u16,
    pub ReparseTargetMaximumLength: u16,
    pub Reserved1: u16,
    pub ReparseTarget: libc::wchar_t,
}

#[cfg(windows)]
#[allow(dead_code)]
pub fn cvt(
    i: windows_sys::Win32::Foundation::BOOL,
) -> io::Result<windows_sys::Win32::Foundation::BOOL> {
    if i == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(i)
    }
}

// Creating a directory junction on windows involves dealing with reparse
// points and the `DeviceIoControl` function, and this code is a skeleton of
// what can be found here:
//
// http://www.flexhex.com/docs/articles/hard-links.phtml
#[cfg(windows)]
#[allow(dead_code)]
fn symlink_junction_inner(target: &Path, dir: &Dir, junction: &Path) -> io::Result<()> {
    use cap_std::fs::OpenOptions;
    use std::os::windows::ffi::OsStrExt;
    use std::os::windows::fs::OpenOptionsExt;
    use std::os::windows::io::AsRawHandle;
    use std::ptr;

    dir.create_dir(junction)?;

    let mut opts = OpenOptions::new();
    opts.write(true);
    opts.custom_flags(
        windows_sys::Win32::Storage::FileSystem::FILE_FLAG_OPEN_REPARSE_POINT
            | windows_sys::Win32::Storage::FileSystem::FILE_FLAG_BACKUP_SEMANTICS,
    );
    let f = dir.open_with(junction, &opts)?;
    let h = f.as_raw_handle();

    unsafe {
        let mut data = [0_u8; MAXIMUM_REPARSE_DATA_BUFFER_SIZE as usize];
        let db = data.as_mut_ptr() as *mut REPARSE_MOUNTPOINT_DATA_BUFFER;
        let buf = &mut (*db).ReparseTarget as *mut libc::wchar_t;
        let mut i = 0;
        // FIXME: this conversion is very hacky
        let v = br"\??\";
        let v = v.iter().map(|x| *x as u16);
        for c in v.chain(target.as_os_str().encode_wide()) {
            *buf.offset(i) = c;
            i += 1;
        }
        *buf.offset(i) = 0;
        i += 1;
        (*db).ReparseTag = windows_sys::Win32::System::SystemServices::IO_REPARSE_TAG_MOUNT_POINT;
        (*db).ReparseTargetMaximumLength = (i * 2) as u16;
        (*db).ReparseTargetLength = ((i - 1) * 2) as u16;
        (*db).ReparseDataLength = (*db).ReparseTargetLength as u32 + 12;

        let mut ret = 0;
        cvt(windows_sys::Win32::System::IO::DeviceIoControl(
            h as _,
            windows_sys::Win32::System::Ioctl::FSCTL_SET_REPARSE_POINT,
            data.as_ptr() as *mut _,
            (*db).ReparseDataLength + 8,
            ptr::null_mut(),
            0,
            &mut ret,
            ptr::null_mut(),
        ))
        .map(drop)
    }
}

/// Same as above, but for fs_utf8.
#[cfg(feature = "fs_utf8")]
#[cfg(windows)]
#[allow(dead_code)]
fn symlink_junction_inner_utf8(
    target: &Utf8Path,
    dir: &cap_std::fs_utf8::Dir,
    junction: &Utf8Path,
) -> io::Result<()> {
    use cap_std::fs::OpenOptions;
    use std::os::windows::ffi::OsStrExt;
    use std::os::windows::fs::OpenOptionsExt;
    use std::os::windows::io::AsRawHandle;
    use std::ptr;

    dir.create_dir(junction)?;

    let mut opts = OpenOptions::new();
    opts.write(true);
    opts.custom_flags(
        windows_sys::Win32::Storage::FileSystem::FILE_FLAG_OPEN_REPARSE_POINT
            | windows_sys::Win32::Storage::FileSystem::FILE_FLAG_BACKUP_SEMANTICS,
    );
    let f = dir.open_with(junction, &opts)?;
    let h = f.as_raw_handle();

    unsafe {
        let mut data = [0_u8; MAXIMUM_REPARSE_DATA_BUFFER_SIZE as usize];
        let db = data.as_mut_ptr() as *mut REPARSE_MOUNTPOINT_DATA_BUFFER;
        let buf = &mut (*db).ReparseTarget as *mut libc::wchar_t;
        let mut i = 0;
        // FIXME: this conversion is very hacky
        let v = br"\??\";
        let v = v.iter().map(|x| *x as u16);
        for c in v.chain(target.as_os_str().encode_wide()) {
            *buf.offset(i) = c;
            i += 1;
        }
        *buf.offset(i) = 0;
        i += 1;
        (*db).ReparseTag = windows_sys::Win32::System::SystemServices::IO_REPARSE_TAG_MOUNT_POINT;
        (*db).ReparseTargetMaximumLength = (i * 2) as u16;
        (*db).ReparseTargetLength = ((i - 1) * 2) as u16;
        (*db).ReparseDataLength = (*db).ReparseTargetLength as u32 + 12;

        let mut ret = 0;
        cvt(windows_sys::Win32::System::IO::DeviceIoControl(
            h as _,
            windows_sys::Win32::System::Ioctl::FSCTL_SET_REPARSE_POINT,
            data.as_ptr() as *mut _,
            (*db).ReparseDataLength + 8,
            ptr::null_mut(),
            0,
            &mut ret,
            ptr::null_mut(),
        ))
        .map(drop)
    }
}
