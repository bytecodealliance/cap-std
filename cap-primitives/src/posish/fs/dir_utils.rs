use crate::fs::OpenOptions;
use posish::fs::OFlags;
#[cfg(unix)]
use std::os::unix::{ffi::OsStrExt, fs::OpenOptionsExt};
#[cfg(target_os = "wasi")]
use std::os::wasi::{ffi::OsStrExt, fs::OpenOptionsExt};
use std::{
    ffi::OsStr,
    fs, io,
    ops::Deref,
    path::{Component, Path},
};
#[cfg(racy_asserts)]
use std::{ffi::OsString, os::unix::ffi::OsStringExt, path::PathBuf};

/// Rust's `Path` implicitly strips redundant slashes, however they aren't
/// redundant in one case: at the end of a path they indicate that a path is
/// expected to name a directory.
pub(crate) fn path_requires_dir(path: &Path) -> bool {
    let bytes = path.as_os_str().as_bytes();

    // If a path ends with '/' or '.', it's a directory. These aren't the only
    // cases, but they are the only cases that Rust's `Path` implicitly
    // normalizes away.
    bytes.ends_with(b"/") || bytes.ends_with(b"/.")
}

/// Rust's `Path` implicitly strips trailing `.` components, however they aren't
/// redundant in one case: at the end of a path they are the final path
/// component, which has different path lookup behavior.
pub(crate) fn path_has_trailing_dot(path: &Path) -> bool {
    let mut bytes = path.as_os_str().as_bytes();

    // If a path ends with '.' followed by any number of '/'s, it's a trailing dot.
    while let Some((last, rest)) = bytes.split_last() {
        if *last == b'/' {
            bytes = rest;
        } else {
            break;
        }
    }
    bytes.ends_with(b"/.") && path.components().next_back() != Some(Component::CurDir)
}

/// Append a trailing `/`. This can be used to require that the given `path`
/// names a directory.
#[cfg(racy_asserts)]
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

#[cfg(racy_asserts)]
#[test]
fn append_dir_suffix_tests() {
    assert!(append_dir_suffix(Path::new("foo").to_path_buf())
        .display()
        .to_string()
        .ends_with('/'));
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

#[test]
fn test_path_requires_dir() {
    assert!(!path_requires_dir(Path::new(".")));
    assert!(path_requires_dir(Path::new("/")));
    assert!(path_requires_dir(Path::new("//")));
    assert!(path_requires_dir(Path::new("/./.")));
    assert!(path_requires_dir(Path::new("foo/")));
    assert!(path_requires_dir(Path::new("foo//")));
    assert!(path_requires_dir(Path::new("foo//.")));
    assert!(path_requires_dir(Path::new("foo/./.")));
    assert!(path_requires_dir(Path::new("foo/./")));
    assert!(path_requires_dir(Path::new("foo/.//")));
}

#[test]
fn test_path_has_trailing_dot() {
    assert!(!path_has_trailing_dot(Path::new(".")));
    assert!(!path_has_trailing_dot(Path::new("/")));
    assert!(!path_has_trailing_dot(Path::new("//")));
    assert!(path_has_trailing_dot(Path::new("/./.")));
    assert!(!path_has_trailing_dot(Path::new("foo/")));
    assert!(!path_has_trailing_dot(Path::new("foo//")));
    assert!(path_has_trailing_dot(Path::new("foo//.")));
    assert!(path_has_trailing_dot(Path::new("foo/./.")));
    assert!(path_has_trailing_dot(Path::new("foo/./")));
    assert!(path_has_trailing_dot(Path::new("foo/.//")));
}
