// This file contains additional fs tests that didn't make it into `fs.rs`.
// The reason for additional module to contain those is so that `fs.rs` mirrors
// Rust's libstd tests.

#[macro_use]
mod sys_common;

use cap_std::fs::{DirBuilder, OpenOptions};
use std::{
    io::{Read, Write},
    str,
};
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
#[cfg_attr(windows, ignore)]
// TODO investigate why this one is failing
fn open_various() {
    let tmpdir = tmpdir();
    #[cfg(not(windows))]
    error!(tmpdir.create(""), "No such file");
    #[cfg(windows)]
    error!(tmpdir.create(""), 2);

    #[cfg(not(windows))]
    error!(tmpdir.create("."), "Is a directory");
    #[cfg(windows)]
    error!(tmpdir.create("."), 2);
}

#[test]
#[cfg_attr(windows, ignore)]
// TODO investigate why this one is failing
fn dir_writable() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir("dir"));
    error_contains!(tmpdir.create("dir"), "Is a directory");
    error_contains!(
        tmpdir.open_with("dir", OpenOptions::new().write(true)),
        "Is a directory"
    );
    error_contains!(
        tmpdir.open_with("dir", OpenOptions::new().append(true)),
        "Is a directory"
    );

    error_contains!(tmpdir.create("dir/."), "Is a directory");
    error_contains!(
        tmpdir.open_with("dir/.", OpenOptions::new().write(true)),
        "Is a directory"
    );
    error_contains!(
        tmpdir.open_with("dir/.", OpenOptions::new().append(true)),
        "Is a directory"
    );

    error_contains!(tmpdir.create("dir/.."), "Is a directory");
    error_contains!(
        tmpdir.open_with("dir/..", OpenOptions::new().write(true)),
        "Is a directory"
    );
    error_contains!(
        tmpdir.open_with("dir/..", OpenOptions::new().append(true)),
        "Is a directory"
    );
}

#[test]
#[cfg_attr(windows, ignore)]
// TODO investigate why this one is failing
fn trailing_slash() {
    let tmpdir = tmpdir();
    check!(tmpdir.create("file"));

    #[cfg(not(windows))]
    error!(tmpdir.open("file/"), "Not a directory");
    #[cfg(windows)]
    error!(tmpdir.open("file/"), 2);
}

#[test]
#[cfg_attr(windows, ignore)]
// TODO investigate why this one is failing
fn rename_slashdots() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir("dir"));
    check!(tmpdir.rename("dir", &tmpdir, "dir"));
    check!(tmpdir.rename("dir", &tmpdir, "dir/"));
    check!(tmpdir.rename("dir/", &tmpdir, "dir"));

    // TODO: Platform-specific error code.
    error_contains!(tmpdir.rename("dir", &tmpdir, "dir/."), "");
    error_contains!(tmpdir.rename("dir/.", &tmpdir, "dir"), "");
}

#[test]
fn optionally_recursive_mkdir() {
    let tmpdir = tmpdir();
    let dir = "d1/d2";
    check!(tmpdir.create_dir_with(dir, DirBuilder::new().recursive(true)));
    assert!(tmpdir.is_dir(dir));
}

#[test]
fn optionally_nonrecursive_mkdir() {
    let tmpdir = tmpdir();
    let dir = "d1/d2";
    #[cfg(not(windows))]
    error!(
        tmpdir.create_dir_with(dir, &DirBuilder::new()),
        "No such file"
    );
    #[cfg(windows)]
    error!(tmpdir.create_dir_with(dir, &DirBuilder::new()), 2);

    assert!(!tmpdir.exists(dir));
}

#[test]
fn file_test_directoryinfo_readdir() {
    let tmpdir = tmpdir();
    let dir = "di_readdir";
    check!(tmpdir.create_dir(dir));
    let prefix = "foo";
    for n in 0..3 {
        let f = format!("{}.txt", n);
        let mut w = check!(tmpdir.create(&f));
        let msg_str = format!("{}{}", prefix, n.to_string());
        let msg = msg_str.as_bytes();
        check!(w.write(msg));
    }
    let files = check!(tmpdir.read_dir(dir));
    let mut mem = [0; 4];
    for f in files {
        let f = f.unwrap();
        {
            check!(check!(f.open()).read(&mut mem));
            let read_str = str::from_utf8(&mem).unwrap();
            let expected = format!("{}{}", prefix, f.file_name().to_str().unwrap());
            assert_eq!(expected, read_str);
        }
        check!(f.remove_file());
    }
    check!(tmpdir.remove_dir(dir));
}
