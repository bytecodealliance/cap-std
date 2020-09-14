#![cfg(windows)]

#[macro_use]
mod sys_common;

use sys_common::io::tmpdir;

#[test]
fn windows_symlinks() {
    let tmpdir = tmpdir();

    check!(tmpdir.create("file"));
    check!(tmpdir.create_dir("dir"));

    // Windows lets these succeed.
    check!(tmpdir.symlink_dir("file", "file_symlink_dir"));
    check!(tmpdir.symlink_file("dir", "dir_symlink_file"));

    // That's just how it is.
    assert!(check!(tmpdir.metadata("dir_symlink_file")).is_dir());
    assert!(check!(tmpdir.metadata("file_symlink_dir")).is_file());

    assert!(check!(tmpdir.symlink_metadata("file_symlink_dir"))
        .file_type()
        .is_symlink());
    assert!(check!(tmpdir.symlink_metadata("dir_symlink_file"))
        .file_type()
        .is_symlink());
}

#[test]
fn windows_symlinks_test() {
    use std::{
        fs,
        os::windows::fs::{symlink_dir, symlink_file},
    };

    let dir = tempfile::tempdir().unwrap();

    check!(fs::File::create(dir.path().join("file")));
    check!(fs::create_dir(dir.path().join("dir")));

    // Windows lets these succeed.
    check!(symlink_file("file", dir.path().join("file_symlink_file")));
    check!(symlink_dir("dir", dir.path().join("dir_symlink_dir")));

    // That's just how it is.
    assert!(check!(fs::metadata(dir.path().join("dir_symlink_dir"))).is_dir());
    assert!(check!(fs::metadata(dir.path().join("file_symlink_file"))).is_file());

    assert!(
        check!(fs::symlink_metadata(dir.path().join("file_symlink_file")))
            .file_type()
            .is_symlink()
    );
    assert!(
        check!(fs::symlink_metadata(dir.path().join("dir_symlink_dir")))
            .file_type()
            .is_symlink()
    );
}

#[test]
fn windows_symlinks_ambient() {
    use std::{
        fs,
        os::windows::fs::{symlink_dir, symlink_file},
    };

    let dir = tempfile::tempdir().unwrap();

    check!(fs::File::create(dir.path().join("file")));
    check!(fs::create_dir(dir.path().join("dir")));

    // Windows lets these succeed.
    check!(symlink_dir("file", dir.path().join("file_symlink_dir")));
    check!(symlink_file("dir", dir.path().join("dir_symlink_file")));

    // That's just how it is.
    assert!(check!(fs::metadata(dir.path().join("dir_symlink_file"))).is_dir());
    assert!(check!(fs::metadata(dir.path().join("file_symlink_dir"))).is_file());

    assert!(
        check!(fs::symlink_metadata(dir.path().join("file_symlink_dir")))
            .file_type()
            .is_symlink()
    );
    assert!(
        check!(fs::symlink_metadata(dir.path().join("dir_symlink_file")))
            .file_type()
            .is_symlink()
    );
}
