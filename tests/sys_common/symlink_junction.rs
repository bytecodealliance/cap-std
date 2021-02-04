// Implementation derived from `symlink_junction` and related code in Rust's
// library/std/src/sys/windows/fs.rs at revision
// 108e90ca78f052c0c1c49c42a22c85620be19712.

use cap_std::fs::Dir;
use std::{io, path::Path};

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

#[cfg(windows)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct REPARSE_MOUNTPOINT_DATA_BUFFER {
    pub ReparseTag: winapi::shared::minwindef::DWORD,
    pub ReparseDataLength: winapi::shared::minwindef::DWORD,
    pub Reserved: winapi::shared::minwindef::WORD,
    pub ReparseTargetLength: winapi::shared::minwindef::WORD,
    pub ReparseTargetMaximumLength: winapi::shared::minwindef::WORD,
    pub Reserved1: winapi::shared::minwindef::WORD,
    pub ReparseTarget: winapi::um::winnt::WCHAR,
}

#[cfg(windows)]
#[allow(dead_code)]
pub fn cvt(i: winapi::shared::minwindef::BOOL) -> io::Result<winapi::shared::minwindef::BOOL> {
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
    use std::{
        os::windows::{ffi::OsStrExt, fs::OpenOptionsExt, io::AsRawHandle},
        ptr,
    };

    dir.create_dir(junction)?;

    let mut opts = OpenOptions::new();
    opts.write(true);
    opts.custom_flags(
        winapi::um::winbase::FILE_FLAG_OPEN_REPARSE_POINT
            | winapi::um::winbase::FILE_FLAG_BACKUP_SEMANTICS,
    );
    let f = dir.open_with(junction, &opts)?;
    let h = f.as_raw_handle();

    unsafe {
        let mut data = [0_u8; winapi::um::winnt::MAXIMUM_REPARSE_DATA_BUFFER_SIZE as usize];
        let db = data.as_mut_ptr() as *mut REPARSE_MOUNTPOINT_DATA_BUFFER;
        let buf = &mut (*db).ReparseTarget as *mut winapi::um::winnt::WCHAR;
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
        (*db).ReparseTag = winapi::um::winnt::IO_REPARSE_TAG_MOUNT_POINT;
        (*db).ReparseTargetMaximumLength = (i * 2) as winapi::shared::minwindef::WORD;
        (*db).ReparseTargetLength = ((i - 1) * 2) as winapi::shared::minwindef::WORD;
        (*db).ReparseDataLength =
            (*db).ReparseTargetLength as winapi::shared::minwindef::DWORD + 12;

        let mut ret = 0;
        cvt(winapi::um::ioapiset::DeviceIoControl(
            h as *mut _,
            winapi::um::winioctl::FSCTL_SET_REPARSE_POINT,
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
