use std::{
    ffi::OsString,
    fs, io,
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::{Component, Path, PathBuf},
};

/// Calculates system path of `file`.
///
/// Note that this function will automatically strip the extended
/// prefix from the resultant path to allow for joining this resultant
/// path with relative components.
pub(crate) fn get_path(file: &fs::File) -> io::Result<PathBuf> {
    // get system path to the handle
    winx::file::get_file_path(file).map(PathBuf::from)
}

/// Convenience function for calling `get_path` and concatenating the result
/// with `path`. This function also checks if `path` is absolute in which case,
/// it emulates POSIX and returns the absolute path unmodified instead.
pub(super) fn concatenate_or_return_absolute(
    file: &fs::File,
    path: &Path,
) -> io::Result<(PathBuf, bool)> {
    if path.is_absolute() {
        return Ok((path.into(), false));
    }

    let file_path = get_path(file)?;

    // `path` is relative, so it isn't a UNC path, so Rust will have normalized
    // `.` paths in it, unless it's just `.`. Check for that case so that we
    // don't join `.` to a UNC path, which Windows doesn't support.
    if path.as_os_str() == Component::CurDir.as_os_str() {
        // Return false for `enforce_dir` since Windows canonicalizes a trailing
        // `\.` away, even when it doesn't canonicalize a trailing `\` away.
        return Ok((file_path, false));
    }

    // Convert `file_path` to 16-bit code units.
    let wide_file_path: Vec<_> = file_path.as_os_str().encode_wide().collect();

    // If we don't have a UNC path, use the path as is.
    if !wide_file_path.starts_with(&['\\' as u16, '\\' as _, '?' as _, '\\' as _]) {
        return Ok((file_path.join(path), false));
    }

    // Otherwise, we have a UNC path and it's needed. Perform canonicalization
    // and check for a trailing slash meaning that callers should manually
    // enforce that the path names a directory.
    let mut wide_path: Vec<_> = path.as_os_str().encode_wide().collect();

    // Strip trailing dots and whitespace.
    while wide_path.ends_with(&['.' as u16]) || wide_path.ends_with(&[' ' as u16]) {
        wide_path.pop();
    }

    // Strip trailing slashes.
    let mut dir_required = false;
    while wide_path.ends_with(&['/' as u16]) || wide_path.ends_with(&['\\' as u16]) {
        dir_required = true;
        wide_path.pop();
    }

    let wide_path = PathBuf::from(OsString::from_wide(&wide_path));
    Ok((file_path.join(wide_path), dir_required))
}
