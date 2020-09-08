#[macro_use]
mod sys_common;

use cap_dir_ext::DirExt;
use sys_common::io::tmpdir;

#[test]
fn basic_symlinks() {
    let tmpdir = tmpdir();

    check!(tmpdir.create("file"));
    check!(tmpdir.create_dir("dir"));

    check!(tmpdir.symlink_file("file", "file_symlink_file"));
    check!(tmpdir.symlink_dir("dir", "dir_symlink_dir"));
    check!(tmpdir.symlink("file", "file_symlink"));
    check!(tmpdir.symlink("dir", "dir_symlink"));

    assert!(check!(tmpdir.metadata("file_symlink_file")).is_file());
    assert!(check!(tmpdir.metadata("dir_symlink_dir")).is_dir());
    assert!(check!(tmpdir.metadata("file_symlink")).is_file());
    assert!(check!(tmpdir.metadata("dir_symlink")).is_dir());

    assert!(check!(tmpdir.symlink_metadata("file_symlink_file"))
        .file_type()
        .is_symlink());
    assert!(check!(tmpdir.symlink_metadata("dir_symlink_dir"))
        .file_type()
        .is_symlink());
    assert!(check!(tmpdir.symlink_metadata("file_symlink"))
        .file_type()
        .is_symlink());
    assert!(check!(tmpdir.symlink_metadata("dir_symlink"))
        .file_type()
        .is_symlink());
}
