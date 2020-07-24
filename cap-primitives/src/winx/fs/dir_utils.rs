use crate::fs::OpenOptions;
#[cfg(not(feature = "no_racy_asserts"))]
use std::path::PathBuf;
use std::{
    ffi::{OsStr, OsString},
    os::windows::{
        ffi::{OsStrExt, OsStringExt},
        fs::OpenOptionsExt,
    },
    path::Path,
};
use winx::file::Flags;

// Rust's `Path` implicity strips redundant slashes and `.` components, however
// they aren't redundant in one case: at the end of a path they indicate that a
// path is expected to name a directory.
pub(crate) fn path_requires_dir(path: &Path) -> bool {
    let wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    wide.ends_with(&['/' as u16])
        || wide.ends_with(&['/' as u16, '.' as _])
        || wide.ends_with(&['\\' as u16])
        || wide.ends_with(&['\\' as u16, '.' as _])
}

// Append a trailing `/`. This can be used to require that the given `path`
// names a directory.
#[cfg(not(feature = "no_racy_asserts"))]
pub(crate) fn append_dir_suffix(path: PathBuf) -> PathBuf {
    let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    wide.push('/' as u16);
    OsString::from_wide(&wide).into()
}

// Strip trailing `/`s, unless this reduces `path` to `/` itself. This is
// used by `mkdir` and others to prevent paths like `foo/` from canonicalizing
// to `foo/.` since these syscalls treat these differently.
pub(crate) fn strip_dir_suffix(path: &Path) -> &Path {
    unimplemented!("strip_dir_suffix")
}

// Return an `OpenOptions` for opening directories.
pub(crate) fn dir_options() -> OpenOptions {
    OpenOptions::new()
        .read(true)
        .attributes(Flags::FILE_FLAG_BACKUP_SEMANTICS.bits())
        .clone()
}

// Test whether an `OpenOptions` is set to only open directories.
pub(crate) fn is_dir_options(options: &OpenOptions) -> bool {
    (options.ext.attributes & Flags::FILE_FLAG_BACKUP_SEMANTICS.bits())
        == Flags::FILE_FLAG_BACKUP_SEMANTICS.bits()
}
