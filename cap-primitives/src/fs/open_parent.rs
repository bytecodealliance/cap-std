use crate::fs::{dir_options, errors, open, open_manually, path_requires_dir, MaybeOwnedFile};
use std::{
    ffi::OsStr,
    io,
    path::{Component, Path},
};

/// The primary purpose of this function is to open the "parent" of `path`. `start`
/// is updated to hold the newly opened file descriptor, and the basename of `path`
/// is returned as `Ok(basename)`. Note that the basename may still refer to a
/// symbolic link.
pub(crate) fn open_parent<'path, 'borrow>(
    start: MaybeOwnedFile<'borrow>,
    path: &'path Path,
) -> io::Result<(MaybeOwnedFile<'borrow>, &'path OsStr)> {
    let (dirname, basename) = split_parent(path).ok_or_else(errors::no_such_file_or_directory)?;

    let dir = if dirname.as_os_str().is_empty() {
        start
    } else {
        MaybeOwnedFile::owned(open(&start, dirname, &dir_options())?)
    };

    Ok((dir, basename.as_os_str()))
}

/// Similar to `open_parent`, but with a `symlink_count` argument which allows it
/// to be part of a multi-part lookup that operates under a single symlink count.
///
/// To do this, it uses `open_manually`, so it doesn't benefit from the same
/// optimizations that using plain `open` does.
pub(crate) fn open_parent_manually<'path, 'borrow>(
    start: MaybeOwnedFile<'borrow>,
    path: &'path Path,
    symlink_count: &mut u8,
) -> io::Result<(MaybeOwnedFile<'borrow>, &'path OsStr)> {
    let (dirname, basename) = split_parent(path).ok_or_else(errors::no_such_file_or_directory)?;

    let dir = if dirname.as_os_str().is_empty() {
        start
    } else {
        open_manually(start, dirname, &dir_options(), symlink_count, None)?
    };

    Ok((dir, basename.as_os_str()))
}

/// Split `path` into parent and basename parts. Return `None` if `path`
/// is empty.
///
/// This differs from `path.parent()` and `path.file_name()` in several respects:
///  - Treat paths ending in `/` and `/.` as implying a directory.
///  - Treat the path `.` as a normal component rather than a parent.
///  - Append a `.` to a path with a trailing `..` to avoid requiring
///    our callers to special-case `..`.
///  - Bare absolute paths are ok.
fn split_parent(path: &Path) -> Option<(&Path, Component)> {
    if path.as_os_str().is_empty() {
        return None;
    }

    if !path_requires_dir(path) {
        let mut comps = path.components();
        if let Some(p) = comps.next_back() {
            match p {
                Component::Normal(_) | Component::CurDir => return Some((comps.as_path(), p)),
                _ => (),
            }
        }
    }

    Some((path, Component::CurDir))
}

#[test]
fn split_parent_basics() {
    assert_eq!(
        split_parent(Path::new("foo/bar/qux")).unwrap(),
        (
            Path::new("foo/bar"),
            Component::Normal(Path::new("qux").as_ref())
        )
    );
    assert_eq!(
        split_parent(Path::new("foo/bar")).unwrap(),
        (
            Path::new("foo"),
            Component::Normal(Path::new("bar").as_ref())
        )
    );
    assert_eq!(
        split_parent(Path::new("foo")).unwrap(),
        (Path::new(""), Component::Normal(Path::new("foo").as_ref()))
    );
}

#[test]
fn split_parent_special_cases() {
    assert!(split_parent(Path::new("")).is_none());
    assert_eq!(
        split_parent(Path::new("foo/")).unwrap(),
        (Path::new("foo"), Component::CurDir)
    );
    assert_eq!(
        split_parent(Path::new("foo/.")).unwrap(),
        (Path::new("foo"), Component::CurDir)
    );
    assert_eq!(
        split_parent(Path::new(".")).unwrap(),
        (Path::new(""), Component::CurDir)
    );
    assert_eq!(
        split_parent(Path::new("..")).unwrap(),
        (Path::new(".."), Component::CurDir)
    );
    assert_eq!(
        split_parent(Path::new("../..")).unwrap(),
        (Path::new("../.."), Component::CurDir)
    );
    assert_eq!(
        split_parent(Path::new("../foo")).unwrap(),
        (
            Path::new(".."),
            Component::Normal(Path::new("foo").as_ref())
        )
    );
    assert_eq!(
        split_parent(Path::new("foo/..")).unwrap(),
        (Path::new("foo/.."), Component::CurDir)
    );
    assert_eq!(
        split_parent(Path::new("/foo")).unwrap(),
        (Path::new("/"), Component::Normal(Path::new("foo").as_ref()))
    );
    assert_eq!(
        split_parent(Path::new("/foo/")).unwrap(),
        (Path::new("/foo"), Component::CurDir)
    );
    assert_eq!(
        split_parent(Path::new("/")).unwrap(),
        (Path::new("/"), Component::CurDir)
    );
}
