// This file contains additional fs tests that didn't make it into `fs.rs`.
// The reason for additional module to contain those is so that `fs.rs` mirrors
// Rust's libstd tests.

mod sys_common;

use cap_std::fs::OpenOptions;
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
fn open_various() {
    let tmpdir = tmpdir();
    error_contains!(tmpdir.create(""), "No such file");
    error_contains!(tmpdir.create("."), "Is a directory");
}

#[test]
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
fn trailing_slash() {
    let tmpdir = tmpdir();
    check!(tmpdir.create("file"));
    error_contains!(tmpdir.open("file/"), "Not a directory");
}

#[test]
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
