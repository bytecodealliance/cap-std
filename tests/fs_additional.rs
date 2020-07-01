// This file contains additional fs tests that didn't make it into `fs.rs`.
// The reason for additional module to contain those is so that `fs.rs` mirrors
// Rust's libstd tests.

mod sys_common;

use std::{io, path::PathBuf};
use sys_common::io::tmpdir;

#[test]
fn recursive_mkdir() {
    let tmpdir = tmpdir();
    let dir = "d1/d2";
    check!(tmpdir.create_dir_all(dir));
    assert!(tmpdir.is_dir("d1"));
    let dir = check!(tmpdir.open_dir("d1"));
    assert!(dir.is_dir("d2"));
    assert!(tmpdir.is_dir("d1/d2"));
}

#[test]
fn canonicalize_escape_attempt() {
    let tmpdir = tmpdir();
    assert_eq!(
        tmpdir.canonicalize("..").unwrap_err().kind(),
        io::ErrorKind::PermissionDenied
    );
    assert_eq!(
        tmpdir.canonicalize("../../").unwrap_err().kind(),
        io::ErrorKind::PermissionDenied
    );
    assert_eq!(
        tmpdir.canonicalize("../../..").unwrap_err().kind(),
        io::ErrorKind::PermissionDenied
    );
    assert_eq!(
        tmpdir.canonicalize("/").unwrap_err().kind(),
        io::ErrorKind::PermissionDenied
    );

    check!(tmpdir.create_dir_all("a/b"));
    assert_eq!(
        tmpdir.canonicalize("a/b/../../../").unwrap_err().kind(),
        io::ErrorKind::PermissionDenied
    );

    let subdir = check!(tmpdir.open_dir("a"));
    assert_eq!(
        subdir.canonicalize("..").unwrap_err().kind(),
        io::ErrorKind::PermissionDenied
    );
}

#[test]
fn canonicalize_interesting_paths() {
    let tmpdir = tmpdir();
    assert_eq!(tmpdir.canonicalize(".").unwrap(), PathBuf::from(""));

    check!(tmpdir.create_dir("a/"));
    assert_eq!(tmpdir.canonicalize("a/../").unwrap(), PathBuf::from(""));
    assert_eq!(tmpdir.canonicalize("a/../.").unwrap(), PathBuf::from(""));
    assert_eq!(tmpdir.canonicalize("a/.").unwrap(), PathBuf::from("a"));
    assert_eq!(tmpdir.canonicalize("a///").unwrap(), PathBuf::from("a"));

    check!(tmpdir.create_file("a/file"));
    assert_eq!(
        tmpdir.canonicalize("a/file").unwrap(),
        PathBuf::from("a/file")
    );
    assert_eq!(
        tmpdir.canonicalize("a/file/.").unwrap(),
        PathBuf::from("a/file")
    );

    let res = tmpdir.canonicalize("a/file/..");
    error_contains!(res, "Not a directory");
    assert_eq!(res.unwrap_err().kind(), io::ErrorKind::Other);
}
