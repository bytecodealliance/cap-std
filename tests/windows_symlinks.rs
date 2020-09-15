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
    dbg!("test");
    assert!(tmpdir.open("dir_symlink_file").is_err());
    dbg!("test");
    assert!(tmpdir.open("file_symlink_dir").is_err());
    dbg!("test");
    assert!(tmpdir.open_dir("dir_symlink_file").is_err());
    dbg!("test");
    assert!(tmpdir.open_dir("file_symlink_dir").is_err());
    dbg!("test");
    assert!(tmpdir.metadata("dir_symlink_file").is_err());
    dbg!("test");
    assert!(tmpdir.metadata("file_symlink_dir").is_err());
    dbg!("test");

    assert!(check!(tmpdir.symlink_metadata("file_symlink_dir"))
        .file_type()
        .is_symlink());
    assert!(check!(tmpdir.symlink_metadata("dir_symlink_file"))
        .file_type()
        .is_symlink());
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
    dbg!("test");
    assert!(fs::File::open(dir.path().join("dir_symlink_file")).is_err());
    dbg!("test");
    assert!(fs::File::open(dir.path().join("file_symlink_dir")).is_err());
    dbg!("test");
    //assert!(
    //unsafe { cap_std::fs::Dir::open_ambient_dir(dir.path().join("dir_symlink_file")) }.is_err()
    //);
    if unsafe {
        dbg!(cap_std::fs::Dir::open_ambient_dir(
            dir.path().join("dir_symlink_file")
        ))
    }
    .is_err()
    {
    } else {
        dbg!(fs::metadata(dir.path().join("dir_symlink_file")));
        dbg!(fs::metadata(dir.path().join("dir")));
    }
    dbg!("test");
    //assert!(
    //unsafe { cap_std::fs::Dir::open_ambient_dir(dir.path().join("file_symlink_dir")) }.is_err()
    //);
    if unsafe {
        dbg!(cap_std::fs::Dir::open_ambient_dir(
            dir.path().join("file_symlink_dir")
        ))
    }
    .is_err()
    {
    } else {
        dbg!(fs::metadata(dir.path().join("file_symlink_dir")));
        dbg!(fs::metadata(dir.path().join("file")));
    }
    dbg!("test");
    assert!(fs::metadata(dir.path().join("dir_symlink_file")).is_err());
    dbg!("test");
    assert!(fs::metadata(dir.path().join("file_symlink_dir")).is_err());
    dbg!("test");

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
