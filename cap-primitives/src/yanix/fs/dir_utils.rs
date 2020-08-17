use crate::fs::OpenOptions;
use std::{
    ffi::OsStr,
    fs, io,
    ops::Deref,
    os::unix::{ffi::OsStrExt, fs::OpenOptionsExt},
    path::Path,
};
#[cfg(not(feature = "no_racy_asserts"))]
use std::{ffi::OsString, os::unix::ffi::OsStringExt, path::PathBuf};
use yanix::file::OFlags;

/// Rust's `Path` implicitly strips redundant slashes and `.` components, however
/// they aren't redundant in one case: at the end of a path they indicate that a
/// path is expected to name a directory.
pub(crate) fn path_requires_dir(path: &Path) -> bool {
    let bytes = path.as_os_str().as_bytes();

    // If a path ends with '/' or '.', it's a directory. These aren't the only
    // cases, but they are the only cases that Rust's `Path` implicitly
    // normalizes away.
    bytes.ends_with(b"/") || bytes.ends_with(b"/.")
}

/// Append a trailing `/`. This can be used to require that the given `path`
/// names a directory.
#[cfg(not(feature = "no_racy_asserts"))]
pub(crate) fn append_dir_suffix(path: PathBuf) -> PathBuf {
    let mut bytes = path.into_os_string().into_vec();
    bytes.push(b'/');
    OsString::from_vec(bytes).into()
}

/// Strip trailing `/`s, unless this reduces `path` to `/` itself. This is
/// used by `mkdir` and others to prevent paths like `foo/` from canonicalizing
/// to `foo/.` since these syscalls treat these differently.
#[allow(clippy::indexing_slicing)]
pub(crate) fn strip_dir_suffix(path: &Path) -> impl Deref<Target = Path> + '_ {
    let mut bytes = path.as_os_str().as_bytes();
    while bytes.len() > 1 && *bytes.last().unwrap() == b'/' {
        bytes = &bytes[..bytes.len() - 1];
    }
    OsStr::from_bytes(bytes).as_ref()
}

/// Return an `OpenOptions` for opening directories.
pub(crate) fn dir_options() -> OpenOptions {
    OpenOptions::new().read(true).dir_required(true).clone()
}

/// Return an `OpenOptions` for canonicalizing paths.
pub(crate) fn canonicalize_options() -> OpenOptions {
    OpenOptions::new().read(true).clone()
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
        .custom_flags(OFlags::DIRECTORY.bits())
        .open(&path)
}

#[test]
fn strip_dir_suffix_tests() {
    assert_eq!(&*strip_dir_suffix(Path::new("/foo//")), Path::new("/foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("/foo/")), Path::new("/foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("foo/")), Path::new("foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("foo")), Path::new("foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("/")), Path::new("/"));
    assert_eq!(&*strip_dir_suffix(Path::new("//")), Path::new("/"));
}
