use crate::fs::OpenOptions;
#[cfg(debug_assertions)]
use std::path::PathBuf;
use std::{ffi::OsStr, path::Path};
use yanix::file::OFlag;

// Rust's `Path` implicity strips redundant slashes and `.` components, however
// they aren't redundant in one case: at the end of a path they indicate that a
// path is expected to name a directory.
pub(crate) fn path_requires_dir(path: &Path) -> bool {
    unimplemented!("path_requires_dir")
}

// Append a trailing `/`. This can be used to require that the given `path`
// names a directory.
#[cfg(debug_assertions)]
pub(crate) fn append_dir_suffix(path: PathBuf) -> PathBuf {
    unimplemented!("append_dir_suffix")
}

// Strip trailing `/`s, unless this reduces `path` to `/` itself. This is
// used by `mkdir` and others to prevent paths like `foo/` from canonicalizing
// to `foo/.` since these syscalls treat these differently.
pub(crate) fn strip_dir_suffix(path: &Path) -> &Path {
    unimplemented!("strip_dir_suffix")
}

// Return an `OpenOptions` for opening directories.
pub(crate) fn dir_options() -> OpenOptions {
    unimplemented!("dir_options")
}

// Test whether an `OpenOptions` is set to only open directories.
pub(crate) fn is_dir_options(options: &OpenOptions) -> bool {
    unimplemented!("is_dir_options")
}
