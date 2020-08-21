use crate::fs::OpenOptions;
use std::{
    ffi::OsString,
    fs, io,
    ops::Deref,
    os::windows::{
        ffi::{OsStrExt, OsStringExt},
        fs::OpenOptionsExt,
    },
    path::{Path, PathBuf},
};
use winapi::um::winnt;
use winx::file::Flags;

/// Rust's `Path` implicitly strips redundant slashes and `.` components, however
/// they aren't redundant in one case: at the end of a path they indicate that a
/// path is expected to name a directory.
pub(crate) fn path_requires_dir(path: &Path) -> bool {
    let wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    wide.ends_with(&['/' as u16])
        || wide.ends_with(&['/' as u16, '.' as _])
        || wide.ends_with(&['\\' as u16])
        || wide.ends_with(&['\\' as u16, '.' as _])
}

/// Append a trailing `/`. This can be used to require that the given `path`
/// names a directory.
#[cfg(not(feature = "no_racy_asserts"))]
pub(crate) fn append_dir_suffix(path: PathBuf) -> PathBuf {
    let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    wide.push('/' as u16);
    OsString::from_wide(&wide).into()
}

/// Strip trailing `/`s, unless this reduces `path` to `/` itself. This is
/// used by `mkdir` and others to prevent paths like `foo/` from canonicalizing
/// to `foo/.` since these syscalls treat these differently.
pub(crate) fn strip_dir_suffix(path: &Path) -> impl Deref<Target = Path> + '_ {
    let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    while wide.len() > 1
        && (*wide.last().unwrap() == '/' as u16 || *wide.last().unwrap() == '\\' as u16)
    {
        wide.pop();
    }
    PathBuf::from(OsString::from_wide(&wide))
}

/// Return an `OpenOptions` for opening directories.
pub(crate) fn dir_options() -> OpenOptions {
    // Set `FILE_FLAG_BACKUP_SEMANTICS` so that we can open directories. Unset
    // `FILE_SHARE_DELETE` so that directories can't be renamed or deleted
    // underneath us, since we use paths to implement many directory operations.
    OpenOptions::new()
        .read(true)
        .dir_required(true)
        .custom_flags(Flags::FILE_FLAG_BACKUP_SEMANTICS.bits())
        .share_mode(winnt::FILE_SHARE_READ | winnt::FILE_SHARE_WRITE)
        .clone()
}

/// Return an `OpenOptions` for canonicalizing paths.
pub(crate) fn canonicalize_options() -> OpenOptions {
    OpenOptions::new()
        .read(true)
        .custom_flags(Flags::FILE_FLAG_BACKUP_SEMANTICS.bits())
        .clone()
}

/// Open a directory named by a bare path, using the host process' ambient
/// authority.
///
/// # Safety
///
/// This function is not sandboxed and may trivially access any path that the
/// host process has access to.
pub(crate) unsafe fn open_ambient_dir_impl(path: &Path) -> io::Result<fs::File> {
    fs::OpenOptions::new()
        .read(true)
        .custom_flags(Flags::FILE_FLAG_BACKUP_SEMANTICS.bits())
        .open(&path)
}
