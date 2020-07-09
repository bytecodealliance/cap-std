mod sys_common;

use std::io;
use sys_common::io::tmpdir;

fn rename_path_in_use() -> String {
    io::Error::from_raw_os_error(libc::EBUSY).to_string()
}

fn no_such_file_or_directory() -> String {
    io::Error::from_raw_os_error(libc::ENOENT).to_string()
}

#[cfg(target_os = "macos")]
fn rename_file_over_dir() -> String {
    io::Error::from_raw_os_error(libc::EISDIR).to_string()
}

#[cfg(not(target_os = "macos"))]
fn rename_file_over_dir() -> String {
    io::Error::from_raw_os_error(libc::ENOTEMPTY).to_string()
}

#[cfg(target_os = "macos")]
fn rename_file_over_dot() -> String {
    rename_file_over_dir()
}

#[cfg(not(target_os = "macos"))]
fn rename_file_over_dot() -> String {
    rename_path_in_use()
}

#[cfg(target_os = "macos")]
fn rename_dot_over_file() -> String {
    io::Error::from_raw_os_error(libc::EINVAL).to_string()
}

#[cfg(not(target_os = "macos"))]
fn rename_dot_over_file() -> String {
    rename_path_in_use()
}

#[test]
fn rename_basics() {
    let tmpdir = tmpdir();

    check!(tmpdir.create_dir_all("foo/bar"));
    check!(tmpdir.create("foo/bar/file.txt"));

    check!(tmpdir.rename("foo/bar/file.txt", &tmpdir, "foo/bar/renamed.txt"));
    assert!(!tmpdir.exists("foo/bar/file.txt"));
    assert!(tmpdir.exists("foo/bar/renamed.txt"));

    check!(tmpdir.rename("foo/bar/renamed.txt", &tmpdir, "foo/bar/renamed.txt"));
    error!(
        tmpdir.rename("foo/bar/renamed.txt", &tmpdir, ".."),
        &rename_path_in_use()
    );
    error!(
        tmpdir.rename("foo/bar/renamed.txt", &tmpdir, "foo/../.."),
        &rename_path_in_use()
    );
    error!(
        tmpdir.rename("foo/bar/renamed.txt", &tmpdir, "/tmp"),
        "a path led outside of the filesystem"
    );
    error!(
        tmpdir.rename("foo/bar/renamed.txt", &tmpdir, "foo/bar/baz/.."),
        &no_such_file_or_directory()
    );
    error!(
        tmpdir.rename("foo/bar/renamed.txt", &tmpdir, "foo/bar"),
        &rename_file_over_dir()
    );
    check!(tmpdir.rename("foo/bar", &tmpdir, "foo/bar"));
    check!(tmpdir.rename("foo/bar/renamed.txt", &tmpdir, "file.txt"));
    assert!(!tmpdir.exists("foo/bar/renamed.txt"));
    assert!(tmpdir.exists("file.txt"));

    error!(
        tmpdir.rename("file.txt", &tmpdir, "foo/.."),
        &rename_path_in_use()
    );
    error!(
        tmpdir.rename("file.txt", &tmpdir, "foo/."),
        "Is a directory"
    );
    error!(
        tmpdir.rename("file.txt", &tmpdir, "foo/bar/../.."),
        &rename_path_in_use()
    );
    error!(
        tmpdir.rename("file.txt", &tmpdir, "foo/bar/../../.."),
        &rename_path_in_use()
    );
    error!(
        tmpdir.rename("file.txt", &tmpdir, "foo/bar/../../../something"),
        "a path led outside of the filesystem"
    );
    error!(
        tmpdir.rename("file.txt", &tmpdir, ""),
        "a path led outside of the filesystem"
    );
    error!(
        tmpdir.rename("file.txt", &tmpdir, "."),
        &rename_file_over_dot()
    );
    error!(
        tmpdir.rename("file.txt", &tmpdir, ".."),
        &rename_path_in_use()
    );
    error!(
        tmpdir.rename("file.txt", &tmpdir, "/"),
        "a path led outside of the filesystem"
    );
    error!(
        tmpdir.rename("file.txt", &tmpdir, "/."),
        "a path led outside of the filesystem"
    );
    error!(
        tmpdir.rename("file.txt", &tmpdir, "/.."),
        "a path led outside of the filesystem"
    );
    error!(
        tmpdir.rename("..", &tmpdir, "nope.txt"),
        &rename_path_in_use()
    );
    error!(
        tmpdir.rename(".", &tmpdir, "nope.txt"),
        &rename_dot_over_file()
    );
    error!(
        tmpdir.rename("/", &tmpdir, "nope.txt"),
        "a path led outside of the filesystem"
    );
    error!(
        tmpdir.rename("/..", &tmpdir, "nope.txt"),
        "a path led outside of the filesystem"
    );

    check!(tmpdir.create("existing.txt"));
    check!(tmpdir.rename("file.txt", &tmpdir, "existing.txt"));
    assert!(!tmpdir.exists("file.txt"));
    assert!(tmpdir.exists("existing.txt"));
}
