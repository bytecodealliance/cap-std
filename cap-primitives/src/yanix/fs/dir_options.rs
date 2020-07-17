use crate::fs::OpenOptions;
use std::{
    ffi::OsStr,
    os::unix::{ffi::OsStrExt, fs::OpenOptionsExt},
    path::Path,
};
#[cfg(debug_assertions)]
use std::{ffi::OsString, os::unix::ffi::OsStringExt, path::PathBuf};
use yanix::file::OFlags;

// Rust's `Path` implicity strips redundant slashes and `.` components, however
// they aren't redundant in one case: at the end of a path they indicate that a
// path is expected to name a directory.
pub(crate) fn path_requires_dir(path: &Path) -> bool {
    let bytes = path.as_os_str().as_bytes();

    // If a path ends with '/' or '.', it's a directory. These aren't the only
    // cases, but they are the only cases that Rust's `Path` implicitly
    // normalizes away.
    bytes.ends_with(b"/") || bytes.ends_with(b"/.")
}

// Append a trailing `/`. This can be used to require that the given `path`
// names a directory.
#[cfg(debug_assertions)]
pub(crate) fn append_dir_suffix(path: PathBuf) -> PathBuf {
    let mut bytes = path.into_os_string().into_vec();
    bytes.push(b'/');
    OsString::from_vec(bytes).into()
}

// Strip trailing `/`s, unless this reduces `path` to `/` itself. This is
// used by `mkdir` and others to prevent paths like `foo/` from canonicalizing
// to `foo/.` since these syscalls treat these differently.
#[allow(clippy::indexing_slicing)]
pub(crate) fn strip_dir_suffix(path: &Path) -> &Path {
    let mut bytes = path.as_os_str().as_bytes();
    while bytes.len() > 1 && *bytes.last().unwrap() == b'/' {
        bytes = &bytes[..bytes.len() - 1];
    }
    OsStr::from_bytes(bytes).as_ref()
}

// Return an `OpenOptions` for opening directories.
pub(crate) fn dir_options() -> OpenOptions {
    OpenOptions::new()
        .read(true)
        .custom_flags(OFlags::DIRECTORY.bits())
        .clone()
}

// Test whether an `OpenOptions` is set to only open directories.
pub(crate) fn is_dir_options(options: &OpenOptions) -> bool {
    (options.ext.custom_flags & OFlags::DIRECTORY.bits()) == OFlags::DIRECTORY.bits()
}

#[test]
fn strip_dir_suffix_tests() {
    assert_eq!(strip_dir_suffix(Path::new("/foo//")), Path::new("/foo"));
    assert_eq!(strip_dir_suffix(Path::new("/foo/")), Path::new("/foo"));
    assert_eq!(strip_dir_suffix(Path::new("foo/")), Path::new("foo"));
    assert_eq!(strip_dir_suffix(Path::new("foo")), Path::new("foo"));
    assert_eq!(strip_dir_suffix(Path::new("/")), Path::new("/"));
    assert_eq!(strip_dir_suffix(Path::new("//")), Path::new("/"));
}
