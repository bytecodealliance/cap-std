use std::{ffi::CString, io, os::unix::ffi::OsStrExt, path::Path};

pub(crate) fn cstr(path: &Path) -> io::Result<CString> {
    Ok(CString::new(path.as_os_str().as_bytes())?)
}
