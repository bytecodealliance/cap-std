mod sys_common;

use sys_common::io::tmpdir;

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
        "Device or resource busy"
    );
    error!(
        tmpdir.rename("foo/bar/renamed.txt", &tmpdir, "foo/../.."),
        "Device or resource busy"
    );
    error!(
        tmpdir.rename("foo/bar/renamed.txt", &tmpdir, "/tmp"),
        "a path led outside of the filesystem"
    );
    error!(
        tmpdir.rename("foo/bar/renamed.txt", &tmpdir, "foo/bar/baz/.."),
        "No such file or directory"
    );
    error!(
        tmpdir.rename("foo/bar/renamed.txt", &tmpdir, "foo/bar"),
        "Directory not empty"
    );
    check!(tmpdir.rename("foo/bar", &tmpdir, "foo/bar"));
    check!(tmpdir.rename("foo/bar/renamed.txt", &tmpdir, "file.txt"));
    assert!(!tmpdir.exists("foo/bar/renamed.txt"));
    assert!(tmpdir.exists("file.txt"));

    error!(
        tmpdir.rename("file.txt", &tmpdir, "foo/.."),
        "Device or resource busy"
    );
    error!(
        tmpdir.rename("file.txt", &tmpdir, "foo/."),
        "Is a directory"
    );
    error!(
        tmpdir.rename("file.txt", &tmpdir, "foo/bar/../.."),
        "Device or resource busy"
    );
    error!(
        tmpdir.rename("file.txt", &tmpdir, "foo/bar/../../.."),
        "Device or resource busy"
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
        "Device or resource busy"
    );
    error!(
        tmpdir.rename("file.txt", &tmpdir, ".."),
        "Device or resource busy"
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
        "Device or resource busy"
    );
    error!(
        tmpdir.rename(".", &tmpdir, "nope.txt"),
        "Device or resource busy"
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
