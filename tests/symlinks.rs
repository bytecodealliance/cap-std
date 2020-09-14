#[macro_use]
mod sys_common;

use cap_dir_ext::DirExt;
use cap_std::fs::Dir;
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

#[test]
fn symlink_absolute() {
    let tmpdir = tmpdir();

    error_contains!(
        tmpdir.symlink("/thing", "thing_symlink_file"),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.symlink_file("/file", "file_symlink_file"),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.symlink_dir("/dir", "dir_symlink_dir"),
        "a path led outside of the filesystem"
    );
}

#[test]
fn readlink_absolute() {
    let dir = tempfile::tempdir().unwrap();

    #[cfg(not(windows))]
    check!(std::os::unix::fs::symlink(
        "/thing",
        dir.path().join("thing_symlink")
    ));
    #[cfg(windows)]
    check!(std::os::windows::fs::symlink_file(
        "/file",
        dir.path().join("file_symlink_file")
    ));
    #[cfg(windows)]
    check!(std::os::windows::fs::symlink_dir(
        "/dir",
        dir.path().join("dir_symlink_dir")
    ));

    let tmpdir = check!(unsafe { Dir::open_ambient_dir(dir.path()) });

    #[cfg(not(windows))]
    error_contains!(
        tmpdir.read_link("thing_symlink"),
        "a path led outside of the filesystem"
    );
    #[cfg(windows)]
    error_contains!(
        tmpdir.read_link("file_symlink_file"),
        "a path led outside of the filesystem"
    );
    #[cfg(windows)]
    error_contains!(
        tmpdir.read_link("dir_symlink_dir"),
        "a path led outside of the filesystem"
    );
}
