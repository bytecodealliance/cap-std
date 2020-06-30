// This file contains additional fs tests that didn't make it into `fs.rs`.
// The reason for additional module to contain those is so that `fs.rs` mirrors
// Rust's libstd tests.

mod sys_common;

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
