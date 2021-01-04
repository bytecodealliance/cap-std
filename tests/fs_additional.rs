// This file contains additional fs tests that didn't make it into `fs.rs`.
// The reason for additional module to contain those is so that `fs.rs` mirrors
// Rust's libstd tests.

#[macro_use]
mod sys_common;

use cap_std::fs::{DirBuilder, OpenOptions};
use std::{
    io::{self, Read, Write},
    path::Path,
    str,
};
use sys_common::{
    io::{tmpdir, TempDir},
    symlink_supported,
};

#[cfg(not(windows))]
fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(src: P, tmpdir: &TempDir, dst: Q) -> io::Result<()> {
    tmpdir.symlink(src, dst)
}
#[cfg(not(windows))]
fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    tmpdir: &TempDir,
    dst: Q,
) -> io::Result<()> {
    tmpdir.symlink(src, dst)
}
#[cfg(windows)]
fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(src: P, tmpdir: &TempDir, dst: Q) -> io::Result<()> {
    tmpdir.symlink_dir(src, dst)
}
#[cfg(windows)]
fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    tmpdir: &TempDir,
    dst: Q,
) -> io::Result<()> {
    tmpdir.symlink_file(src, dst)
}

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
#[cfg_attr(windows, ignore)] // TODO investigate why this one is failing
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
#[cfg_attr(windows, ignore)] // TODO investigate why this one is failing
fn dir_writable() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir("dir"));
    #[cfg(not(windows))]
    error_contains!(tmpdir.create("dir"), "Is a directory");
    #[cfg(windows)]
    error!(tmpdir.create("dir"), 5);
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

    #[cfg(not(windows))]
    {
        error!(tmpdir.open("file/"), "Not a directory");
        error!(tmpdir.open_dir("file/"), "Not a directory");
    }

    #[cfg(windows)]
    {
        error!(tmpdir.open("file/"), 123);
        error!(tmpdir.open_dir("file/"), 123);
    }
}

#[test]
fn trailing_slash_in_dir() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir("dir"));
    check!(tmpdir.create("dir/file"));

    check!(tmpdir.open_dir("dir"));
    check!(tmpdir.open_dir("dir/"));
    check!(tmpdir.open_dir("dir/."));
    check!(tmpdir.open("dir/file"));

    #[cfg(not(windows))]
    {
        error!(tmpdir.open("dir/file/"), "Not a directory");
        error!(tmpdir.open_dir("dir/file/"), "Not a directory");
    }

    #[cfg(windows)]
    {
        error!(tmpdir.open("dir/file/"), 123);
        error!(tmpdir.open_dir("dir/file/"), 123);
    }
}

#[test]
#[cfg_attr(windows, ignore)] // TODO investigate why this one is failing
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

#[test]
fn follow_dotdot_symlink() {
    if !symlink_supported() {
        return;
    }

    let tmpdir = tmpdir();
    check!(tmpdir.create_dir_all("a/b"));
    check!(symlink_dir("..", &tmpdir, "a/b/c"));
    check!(symlink_dir("../..", &tmpdir, "a/b/d"));
    check!(symlink_dir("../../..", &tmpdir, "a/b/e"));
    check!(symlink_dir("../../../..", &tmpdir, "a/b/f"));

    check!(tmpdir.open_dir("a/b/c"));
    assert!(check!(tmpdir.metadata("a/b/c")).is_dir());

    check!(tmpdir.open_dir("a/b/d"));
    assert!(check!(tmpdir.metadata("a/b/d")).is_dir());

    assert!(tmpdir.open_dir("a/b/e").is_err());
    assert!(tmpdir.metadata("a/b/e").is_err());

    assert!(tmpdir.open_dir("a/b/f").is_err());
    assert!(tmpdir.metadata("a/b/f").is_err());
}

#[test]
fn follow_file_symlink() {
    if !symlink_supported() {
        return;
    }

    let tmpdir = tmpdir();

    check!(tmpdir.create("file"));

    check!(symlink_file("file", &tmpdir, "link"));
    check!(symlink_file("file/", &tmpdir, "link_slash"));
    check!(symlink_file("file/.", &tmpdir, "link_slashdot"));
    check!(symlink_file("file/..", &tmpdir, "link_slashdotdot"));

    check!(tmpdir.open("link"));
    assert!(tmpdir.open("link_slash").is_err());
    assert!(tmpdir.open("link_slashdot").is_err());
    assert!(tmpdir.open("link_slashdotdot").is_err());
}

#[cfg(unix)]
#[test]
fn check_dot_access() {
    use cap_std::fs::DirBuilder;
    use std::os::unix::fs::DirBuilderExt;

    let tmpdir = tmpdir();

    let mut options = DirBuilder::new();
    options.mode(0o477);
    check!(tmpdir.create_dir_with("dir", &options));

    check!(tmpdir.metadata("."));
    check!(tmpdir.metadata("dir"));
    check!(tmpdir.metadata("dir/"));
    check!(tmpdir.metadata("dir//"));

    #[cfg(not(target_os = "freebsd"))]
    {
        assert!(tmpdir.metadata("dir/.").is_err());
        assert!(tmpdir.metadata("dir/./").is_err());
        assert!(tmpdir.metadata("dir/.//").is_err());
        assert!(tmpdir.metadata("dir/./.").is_err());
        assert!(tmpdir.metadata("dir/.//.").is_err());
        assert!(tmpdir.metadata("dir/..").is_err());
        assert!(tmpdir.metadata("dir/../").is_err());
        assert!(tmpdir.metadata("dir/..//").is_err());
        assert!(tmpdir.metadata("dir/../.").is_err());
        assert!(tmpdir.metadata("dir/..//.").is_err());
    }

    #[cfg(target_os = "freebsd")]
    {
        assert!(tmpdir.metadata("dir/.").is_ok());
        assert!(tmpdir.metadata("dir/./").is_ok());
        assert!(tmpdir.metadata("dir/.//").is_ok());
        assert!(tmpdir.metadata("dir/./.").is_ok());
        assert!(tmpdir.metadata("dir/.//.").is_ok());
        assert!(tmpdir.metadata("dir/..").is_ok());
        assert!(tmpdir.metadata("dir/../").is_ok());
        assert!(tmpdir.metadata("dir/..//").is_ok());
        assert!(tmpdir.metadata("dir/../.").is_ok());
        assert!(tmpdir.metadata("dir/..//.").is_ok());
    }
}

/// This test is the same as `check_dot_access` but uses `std::fs`'
/// ambient API instead of `cap_std`. The purpose of this test is to
/// confirm fundamentally OS-specific differences.
#[cfg(unix)]
#[test]
fn check_dot_access_ambient() {
    use std::{fs, os::unix::fs::DirBuilderExt};

    let dir = tempfile::tempdir().unwrap();

    let mut options = std::fs::DirBuilder::new();
    options.mode(0o477);
    check!(options.create(dir.path().join("dir")));

    check!(fs::metadata(dir.path().join(".")));
    check!(fs::metadata(dir.path().join("dir")));
    check!(fs::metadata(dir.path().join("dir/")));
    check!(fs::metadata(dir.path().join("dir//")));

    #[cfg(not(target_os = "freebsd"))]
    {
        assert!(fs::metadata(dir.path().join("dir/.")).is_err());
        assert!(fs::metadata(dir.path().join("dir/./")).is_err());
        assert!(fs::metadata(dir.path().join("dir/.//")).is_err());
        assert!(fs::metadata(dir.path().join("dir/./.")).is_err());
        assert!(fs::metadata(dir.path().join("dir/.//.")).is_err());
        assert!(fs::metadata(dir.path().join("dir/..")).is_err());
        assert!(fs::metadata(dir.path().join("dir/../")).is_err());
        assert!(fs::metadata(dir.path().join("dir/..//")).is_err());
        assert!(fs::metadata(dir.path().join("dir/../.")).is_err());
        assert!(fs::metadata(dir.path().join("dir/..//.")).is_err());
    }

    #[cfg(target_os = "freebsd")]
    {
        assert!(fs::metadata(dir.path().join("dir/.")).is_ok());
        assert!(fs::metadata(dir.path().join("dir/./")).is_ok());
        assert!(fs::metadata(dir.path().join("dir/.//")).is_ok());
        assert!(fs::metadata(dir.path().join("dir/./.")).is_ok());
        assert!(fs::metadata(dir.path().join("dir/.//.")).is_ok());
        assert!(fs::metadata(dir.path().join("dir/..")).is_ok());
        assert!(fs::metadata(dir.path().join("dir/../")).is_ok());
        assert!(fs::metadata(dir.path().join("dir/..//")).is_ok());
        assert!(fs::metadata(dir.path().join("dir/../.")).is_ok());
        assert!(fs::metadata(dir.path().join("dir/..//.")).is_ok());
    }
}

#[cfg(all(unix, not(any(target_os = "ios", target_os = "macos"))))]
#[test]
fn dir_searchable_unreadable() {
    use cap_std::fs::DirBuilder;
    use std::os::unix::fs::DirBuilderExt;

    let tmpdir = tmpdir();

    let mut options = DirBuilder::new();
    options.mode(0o333);
    check!(tmpdir.create_dir_with("dir", &options));
    check!(tmpdir.create_dir_with("dir/writeable_subdir", &options));
    options.mode(0o111);
    check!(tmpdir.create_dir_with("dir/subdir", &options));

    assert!(check!(tmpdir.metadata("dir/.")).is_dir());
    assert!(check!(tmpdir.metadata("dir/subdir")).is_dir());
    assert!(check!(tmpdir.metadata("dir/subdir/.")).is_dir());
}

/// This test is the same as `dir_searchable_unreadable` but uses `std::fs`'
/// ambient API instead of `cap_std`. The purpose of this test is to
/// confirm fundamentally OS-specific differences.
#[cfg(all(unix, not(any(target_os = "ios", target_os = "macos"))))]
#[test]
fn dir_searchable_unreadable_ambient() {
    use std::{fs, os::unix::fs::DirBuilderExt};

    let dir = tempfile::tempdir().unwrap();

    let mut options = std::fs::DirBuilder::new();
    options.mode(0o333);
    check!(options.create(dir.path().join("dir")));
    check!(options.create(dir.path().join("dir/writeable_subdir")));
    options.mode(0o111);
    check!(options.create(dir.path().join("dir/subdir")));

    assert!(check!(fs::metadata(dir.path().join("dir/."))).is_dir());
    assert!(check!(fs::metadata(dir.path().join("dir/subdir"))).is_dir());
    assert!(check!(fs::metadata(dir.path().join("dir/subdir/."))).is_dir());
}

/// On Darwin, we don't have a race-free way to create a subdirectory within
/// a directory that we don't have read access to.
#[cfg(any(target_os = "ios", target_os = "macos"))]
#[test]
fn dir_searchable_unreadable() {
    use cap_std::fs::DirBuilder;
    use std::os::unix::fs::DirBuilderExt;

    let tmpdir = tmpdir();

    let mut options = DirBuilder::new();
    options.mode(0o333);
    check!(tmpdir.create_dir_with("dir", &options));
    assert!(tmpdir
        .create_dir_with("dir/writeable_subdir", &options)
        .is_err());
}

/// Test opening a directory with no permissions.
#[test]
fn dir_unsearchable_unreadable() {
    use cap_std::fs::DirBuilder;
    use std::os::unix::fs::DirBuilderExt;

    let tmpdir = tmpdir();

    let mut options = DirBuilder::new();
    options.mode(0o000);
    check!(tmpdir.create_dir_with("dir", &options));

    // Platforms with `O_PATH` can open a directory with no permissions.
    if cfg!(any(target_os = "linux", target_os = "android", target_os = "redox")) {
        check!(tmpdir.open_dir("dir"));
    } else {
        assert!(tmpdir.open_dir("dir").is_err());
    }
}

/// This test is the same as `symlink_hard_link` but uses `std::fs`'
/// ambient API instead of `cap_std`. The purpose of this test is to
/// confirm fundamentally OS-specific behaviors.
#[test]
#[cfg_attr(any(target_os = "macos", target_os = "freebsd"), ignore)] // submitted to upstream as https://github.com/rust-lang/rust/pull/78026
fn symlink_hard_link_ambient() {
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    #[cfg(windows)]
    use std::os::windows::fs::symlink_file;

    if !symlink_supported() {
        return;
    }

    let dir = tempfile::tempdir().unwrap();

    check!(std::fs::File::create(dir.path().join("file")));
    #[cfg(not(windows))]
    check!(symlink("file", dir.path().join("symlink")));
    #[cfg(windows)]
    check!(symlink_file("file", dir.path().join("symlink")));
    check!(std::fs::hard_link(
        dir.path().join("symlink"),
        dir.path().join("hard_link")
    ));
    assert!(
        check!(std::fs::symlink_metadata(dir.path().join("hard_link")))
            .file_type()
            .is_symlink()
    );
    let _ = check!(std::fs::File::open(dir.path().join("file")));
    assert!(std::fs::File::open(dir.path().join("file.renamed")).is_err());
    let _ = check!(std::fs::File::open(dir.path().join("symlink")));
    let _ = check!(std::fs::File::open(dir.path().join("hard_link")));
    check!(std::fs::rename(
        dir.path().join("file"),
        dir.path().join("file.renamed")
    ));
    assert!(std::fs::File::open(dir.path().join("file")).is_err());
    let _ = check!(std::fs::File::open(dir.path().join("file.renamed")));
    assert!(std::fs::File::open(dir.path().join("symlink")).is_err());
    assert!(std::fs::File::open(dir.path().join("hard_link")).is_err());
    assert!(std::fs::read_link(dir.path().join("file")).is_err());
    assert!(std::fs::read_link(dir.path().join("file.renamed")).is_err());
    assert_eq!(
        check!(std::fs::read_link(dir.path().join("symlink"))),
        Path::new("file")
    );
    assert_eq!(
        check!(std::fs::read_link(dir.path().join("hard_link"))),
        Path::new("file")
    );
    check!(std::fs::remove_file(dir.path().join("file.renamed")));
    assert!(std::fs::File::open(dir.path().join("file")).is_err());
    assert!(std::fs::File::open(dir.path().join("file.renamed")).is_err());
    assert!(std::fs::File::open(dir.path().join("symlink")).is_err());
    assert!(std::fs::File::open(dir.path().join("hard_link")).is_err());
    assert!(
        check!(std::fs::symlink_metadata(dir.path().join("hard_link")))
            .file_type()
            .is_symlink()
    );
}

/// POSIX says that whether or not `link` follows symlinks in the `old`
/// path is implementation-defined. We want `hard_link` to not follow
/// symbolic links.
#[test]
fn symlink_hard_link() {
    if !symlink_supported() {
        return;
    }

    let tmpdir = tmpdir();

    check!(tmpdir.create("file"));
    check!(symlink_file("file", &tmpdir, "symlink"));
    check!(tmpdir.hard_link("symlink", &tmpdir, "hard_link"));
    assert!(check!(tmpdir.symlink_metadata("hard_link"))
        .file_type()
        .is_symlink());
    let _ = check!(tmpdir.open("file"));
    assert!(tmpdir.open("file.renamed").is_err());
    let _ = check!(tmpdir.open("symlink"));
    let _ = check!(tmpdir.open("hard_link"));
    check!(tmpdir.rename("file", &tmpdir, "file.renamed"));
    assert!(tmpdir.open("file").is_err());
    let _ = check!(tmpdir.open("file.renamed"));
    assert!(tmpdir.open("symlink").is_err());
    assert!(tmpdir.open("hard_link").is_err());
    assert!(tmpdir.read_link("file").is_err());
    assert!(tmpdir.read_link("file.renamed").is_err());
    assert_eq!(check!(tmpdir.read_link("symlink")), Path::new("file"));
    assert_eq!(check!(tmpdir.read_link("hard_link")), Path::new("file"));
    check!(tmpdir.remove_file("file.renamed"));
    assert!(tmpdir.open("file").is_err());
    assert!(tmpdir.open("file.renamed").is_err());
    assert!(tmpdir.open("symlink").is_err());
    assert!(tmpdir.open("hard_link").is_err());
    assert!(check!(tmpdir.symlink_metadata("hard_link"))
        .file_type()
        .is_symlink());
}
