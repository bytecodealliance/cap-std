use std::{
    ffi::OsString,
    fs, io,
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::PathBuf,
};

pub(crate) fn get_path(file: &fs::File) -> io::Result<PathBuf> {
    // get system path to the handle
    let path = winx::file::get_file_path(file)?;

    // strip extended prefix; otherwise we will error out on any relative
    // components with `out_path`
    let wide: Vec<_> = path.as_os_str().encode_wide().collect();
    if let Some(prefix) = wide.get(0..4) {
        if &[92, 92, 63, 92] == prefix {
            return Ok(PathBuf::from(OsString::from_wide(&wide)));
        }
    }
    Ok(PathBuf::from(OsString::from_wide(&wide)))
}
