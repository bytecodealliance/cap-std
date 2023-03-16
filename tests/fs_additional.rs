// This file contains additional fs tests that didn't make it into `fs.rs`.
// The reason for additional module to contain those is so that `fs.rs` mirrors
// Rust's libstd tests.

#[macro_use]
mod sys_common;

use cap_std::fs::{DirBuilder, OpenOptions};
use cap_std::time::SystemClock;
use std::io::{self, Read, Write};
use std::path::Path;
use std::str;
use sys_common::io::{tmpdir, TempDir};
use sys_common::symlink_supported;

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
        error!(tmpdir.open("file/../file"), "Not a directory");
        error!(tmpdir.open("file/.."), "Not a directory");
        error!(tmpdir.open("file/."), "Not a directory");
        error!(tmpdir.open("file/../file/"), "Not a directory");
        error!(tmpdir.open("file/"), "Not a directory");
        error!(tmpdir.open_dir("file/../file/"), "Not a directory");
        error!(tmpdir.open_dir("file/../file"), "Not a directory");
        error!(tmpdir.open_dir("file/.."), "Not a directory");
        error!(tmpdir.open_dir("file/."), "Not a directory");
        error!(tmpdir.open_dir("file/"), "Not a directory");
    }

    #[cfg(windows)]
    {
        assert!(check!(check!(tmpdir.open("file/../file")).metadata()).is_file());
        assert!(check!(check!(tmpdir.open_dir("file/..")).dir_metadata()).is_dir());
        assert!(check!(check!(tmpdir.open("file/.")).metadata()).is_file());
        assert!(tmpdir.open_dir("file/../file/").is_err());
        assert!(tmpdir.open_dir("file/./").is_err());
        assert!(tmpdir.open_dir("file//").is_err());
        assert!(tmpdir.open_dir("file/../file").is_err());
        assert!(tmpdir.open_dir("file/.").is_err());
        assert!(tmpdir.open_dir("file/").is_err());
    }
}

#[test]
fn trailing_slash_in_dir() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir("dir"));
    check!(tmpdir.create("dir/file"));

    #[cfg(not(windows))]
    {
        error!(tmpdir.open("dir/file/../file"), "Not a directory");
        error!(tmpdir.open("dir/file/.."), "Not a directory");
        error!(tmpdir.open("dir/file/."), "Not a directory");
        error!(tmpdir.open("dir/file/../file/"), "Not a directory");
        error!(tmpdir.open("dir/file/"), "Not a directory");
        error!(tmpdir.open_dir("dir/file/../file/"), "Not a directory");
        error!(tmpdir.open_dir("dir/file/../file"), "Not a directory");
        error!(tmpdir.open_dir("dir/file/.."), "Not a directory");
        error!(tmpdir.open_dir("dir/file/."), "Not a directory");
        error!(tmpdir.open_dir("dir/file/"), "Not a directory");
    }

    #[cfg(windows)]
    {
        assert!(check!(check!(tmpdir.open("dir/file/../file")).metadata()).is_file());
        assert!(check!(check!(tmpdir.open_dir("dir/file/..")).dir_metadata()).is_dir());
        assert!(check!(check!(tmpdir.open("dir/file/.")).metadata()).is_file());
        assert!(tmpdir.open("dir/file/../file/").is_err());
        let _ = check!(tmpdir.open("dir/file/../file/."));
        assert!(tmpdir.open("dir/file/../file/./").is_err());
        assert!(tmpdir.open("dir/file/").is_err());
        let _ = check!(tmpdir.open("dir/file/."));
        let _ = check!(tmpdir.open("dir/file/../file/."));
        assert!(tmpdir.open("dir/file/../file/./").is_err());
        assert!(tmpdir.open("dir/file/").is_err());
        let _ = check!(tmpdir.open("dir/file/."));
        assert!(tmpdir.open("dir/file/./").is_err());
        assert!(tmpdir.open_dir("dir/file/../file/").is_err());
        assert!(tmpdir.open_dir("dir/file/../file/.").is_err());
        assert!(tmpdir.open_dir("dir/file/../file/./").is_err());
        assert!(tmpdir.open_dir("dir/file/../file").is_err());
        assert!(tmpdir.open_dir("dir/file/.").is_err());
        assert!(tmpdir.open_dir("dir/file/./").is_err());
        assert!(tmpdir.open_dir("dir/file/").is_err());
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
    check!(tmpdir.rename("dir/", &tmpdir, "dir/"));

    // TODO: Platform-specific error code.
    error_contains!(tmpdir.rename("dir", &tmpdir, "dir/."), "");
    error_contains!(tmpdir.rename("dir/.", &tmpdir, "dir"), "");
}

#[test]
#[cfg_attr(windows, ignore)] // TODO investigate why this one is failing
fn rename_slashdots_ambient() {
    let dir = tempfile::tempdir().unwrap();

    check!(std::fs::create_dir_all(dir.path().join("dir")));
    check!(std::fs::rename(
        dir.path().join("dir"),
        dir.path().join("dir")
    ));
    check!(std::fs::rename(
        dir.path().join("dir"),
        dir.path().join("dir/")
    ));
    check!(std::fs::rename(
        dir.path().join("dir/"),
        dir.path().join("dir")
    ));
    check!(std::fs::rename(
        dir.path().join("dir/"),
        dir.path().join("dir/")
    ));

    // TODO: Platform-specific error code.
    error_contains!(
        std::fs::rename(dir.path().join("dir"), dir.path().join("dir/.")),
        ""
    );
    error_contains!(
        std::fs::rename(dir.path().join("dir/."), dir.path().join("dir")),
        ""
    );
}

#[test]
fn optionally_recursive_mkdir() {
    let tmpdir = tmpdir();
    let dir = "d1/d2";
    check!(tmpdir.create_dir_with(dir, DirBuilder::new().recursive(true)));
    assert!(tmpdir.is_dir(dir));
}

#[test]
fn try_exists() {
    let tmpdir = tmpdir();
    assert_eq!(tmpdir.try_exists("somefile").unwrap(), false);
    let dir = Path::new("d1/d2");
    let parent = dir.parent().unwrap();
    assert_eq!(tmpdir.try_exists(parent).unwrap(), false);
    assert_eq!(tmpdir.try_exists(dir).unwrap(), false);
    check!(tmpdir.create_dir(parent));
    assert_eq!(tmpdir.try_exists(parent).unwrap(), true);
    assert_eq!(tmpdir.try_exists(dir).unwrap(), false);
    check!(tmpdir.create_dir(dir));
    assert_eq!(tmpdir.try_exists(dir).unwrap(), true);
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

    #[cfg(windows)]
    {
        error!(tmpdir.open_dir("a/b/d"), 123);
        error!(tmpdir.metadata("a/b/d"), 123);

        error!(tmpdir.open_dir("a/b/e"), 123);
        error!(tmpdir.metadata("a/b/e"), 123);

        error!(tmpdir.open_dir("a/b/f"), 123);
        error!(tmpdir.metadata("a/b/f"), 123);
    }

    #[cfg(not(windows))]
    {
        check!(tmpdir.open_dir("a/b/d"));
        assert!(check!(tmpdir.metadata("a/b/d")).is_dir());

        assert!(tmpdir.open_dir("a/b/e").is_err());
        assert!(tmpdir.metadata("a/b/e").is_err());

        assert!(tmpdir.open_dir("a/b/f").is_err());
        assert!(tmpdir.metadata("a/b/f").is_err());
    }
}

#[test]
fn follow_dotdot_symlink_ambient() {
    use cap_std::ambient_authority;
    use cap_std::fs::Dir;
    #[cfg(unix)]
    use std::os::unix::fs::symlink as symlink_dir;
    #[cfg(windows)]
    use std::os::windows::fs::symlink_dir;

    if !symlink_supported() {
        return;
    }

    let dir = tempfile::tempdir().unwrap();
    check!(std::fs::create_dir_all(dir.path().join("a/b")));
    check!(symlink_dir("..", dir.path().join("a/b/c")));
    check!(symlink_dir("../..", dir.path().join("a/b/d")));
    check!(symlink_dir("../../..", dir.path().join("a/b/e")));
    check!(symlink_dir("../../../..", dir.path().join("a/b/f")));

    check!(Dir::open_ambient_dir(
        dir.path().join("a/b/c"),
        ambient_authority()
    ));
    assert!(check!(std::fs::metadata(dir.path().join("a/b/c"))).is_dir());

    #[cfg(windows)]
    {
        error!(
            Dir::open_ambient_dir(dir.path().join("a/b/d"), ambient_authority()),
            123
        );
        error!(std::fs::metadata(dir.path().join("a/b/d")), 123);

        error!(
            Dir::open_ambient_dir(dir.path().join("a/b/e"), ambient_authority()),
            123
        );
        error!(std::fs::metadata(dir.path().join("a/b/e")), 123);

        error!(
            Dir::open_ambient_dir(dir.path().join("a/b/f"), ambient_authority()),
            123
        );
        error!(std::fs::metadata(dir.path().join("a/b/f")), 123);
    }

    #[cfg(not(windows))]
    {
        check!(Dir::open_ambient_dir(
            dir.path().join("a/b/d"),
            ambient_authority()
        ));
        assert!(check!(std::fs::metadata(dir.path().join("a/b/d"))).is_dir());

        check!(Dir::open_ambient_dir(
            dir.path().join("a/b/e"),
            ambient_authority()
        ));
        assert!(check!(std::fs::metadata(dir.path().join("a/b/e"))).is_dir());

        check!(Dir::open_ambient_dir(
            dir.path().join("a/b/f"),
            ambient_authority()
        ));
        assert!(check!(std::fs::metadata(dir.path().join("a/b/f"))).is_dir());
    }
}

#[test]
fn follow_file_symlink() {
    if !symlink_supported() {
        return;
    }

    let tmpdir = tmpdir();

    check!(tmpdir.create("file"));

    check!(symlink_file("file", &tmpdir, "link"));
    check!(symlink_dir("file/", &tmpdir, "link_slash"));
    check!(symlink_file("file/.", &tmpdir, "link_slashdot"));
    check!(symlink_dir("file/..", &tmpdir, "link_slashdotdot"));

    check!(tmpdir.open("link"));
    assert!(tmpdir.open("link_slash").is_err());

    #[cfg(windows)]
    {
        error!(tmpdir.open("link_slashdot"), 123);
        error!(tmpdir.open_dir("link_slashdotdot"), 123);
    }
    #[cfg(not(windows))]
    {
        assert!(tmpdir.open("link_slash").is_err());
        assert!(tmpdir.open("link_slashdot").is_err());
        assert!(tmpdir.open_dir("link_slashdotdot").is_err());
    }
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
    use std::fs;
    use std::os::unix::fs::DirBuilderExt;

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

// Windows allows one to open "file/." and "file/.." and similar, however it
// doesn't allow "file/" or similar.
#[cfg(windows)]
#[test]
fn file_with_trailing_slashdot() {
    let tmpdir = tmpdir();
    check!(tmpdir.create("file"));
    check!(tmpdir.open("file"));
    check!(tmpdir.open("file\\."));
    check!(tmpdir.open("file/."));
    check!(tmpdir.open("file\\.\\."));
    check!(tmpdir.open("file/./."));
    assert!(tmpdir.open("file\\").is_err());
    assert!(tmpdir.open("file/").is_err());
    assert!(tmpdir.open("file\\.\\").is_err());
    assert!(tmpdir.open("file/./").is_err());
    check!(tmpdir.open_dir("file\\.."));
    check!(tmpdir.open_dir("file/.."));
    check!(tmpdir.open_dir("file\\.\\.."));
    check!(tmpdir.open_dir("file/./.."));
    check!(tmpdir.open_dir("file\\..\\."));
    check!(tmpdir.open_dir("file/../."));
    check!(tmpdir.open_dir("file\\..\\"));
    check!(tmpdir.open_dir("file/../"));
    assert!(tmpdir.open_dir("file\\...").is_err());
    assert!(tmpdir.open_dir("file/...").is_err());
}

/// This is just to confirm that Windows really does allow one to open "file/."
/// and "file/..", and similar, however it doesn't allow "file/" or similar.
#[cfg(windows)]
#[test]
fn file_with_trailing_slashdot_ambient() {
    use cap_std::ambient_authority;
    use cap_std::fs::Dir;
    let dir = tempfile::tempdir().unwrap();
    check!(std::fs::File::create(dir.path().join("file")));
    check!(std::fs::File::open(dir.path().join("file")));
    check!(std::fs::File::open(dir.path().join("file\\.")));
    check!(std::fs::File::open(dir.path().join("file/.")));
    check!(std::fs::File::open(dir.path().join("file\\.\\.")));
    check!(std::fs::File::open(dir.path().join("file/./.")));
    assert!(std::fs::File::open(dir.path().join("file\\")).is_err());
    assert!(std::fs::File::open(dir.path().join("file/")).is_err());
    assert!(std::fs::File::open(dir.path().join("file\\.\\")).is_err());
    assert!(std::fs::File::open(dir.path().join("file/./")).is_err());
    check!(Dir::open_ambient_dir(
        dir.path().join("file/.."),
        ambient_authority()
    ));
    check!(Dir::open_ambient_dir(
        dir.path().join("file\\.\\.."),
        ambient_authority()
    ));
    check!(Dir::open_ambient_dir(
        dir.path().join("file/./.."),
        ambient_authority()
    ));
    check!(Dir::open_ambient_dir(
        dir.path().join("file\\..\\."),
        ambient_authority()
    ));
    check!(Dir::open_ambient_dir(
        dir.path().join("file/../."),
        ambient_authority()
    ));
    check!(Dir::open_ambient_dir(
        dir.path().join("file\\..\\"),
        ambient_authority()
    ));
    check!(Dir::open_ambient_dir(
        dir.path().join("file/../"),
        ambient_authority()
    ));
    assert!(Dir::open_ambient_dir(dir.path().join("file\\..."), ambient_authority()).is_err());
    assert!(Dir::open_ambient_dir(dir.path().join("file/..."), ambient_authority()).is_err());
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
    use std::fs;
    use std::os::unix::fs::DirBuilderExt;

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
#[cfg(unix)]
#[test]
fn dir_unsearchable_unreadable() {
    use cap_std::fs::DirBuilder;
    use std::os::unix::fs::DirBuilderExt;

    let tmpdir = tmpdir();

    let mut options = DirBuilder::new();
    options.mode(0o000);
    check!(tmpdir.create_dir_with("dir", &options));

    // Platforms with `O_PATH` can open a directory with no permissions. And
    // somehow FreeBSD can too; see `dir_unsearchable_unreadable_ambient`
    // below confirming this.
    if cfg!(any(
        target_os = "android",
        target_os = "linux",
        target_os = "redox",
    )) {
        let dir = check!(tmpdir.open_dir("dir"));
        assert!(dir.entries().is_err());
        assert!(dir.open_dir(".").is_err());
    } else if cfg!(target_os = "freebsd") {
        let dir = check!(tmpdir.open_dir("dir"));
        check!(dir.metadata("."));
        check!(dir.entries());
        check!(dir.open_dir("."));
    } else {
        assert!(tmpdir.open_dir("dir").is_err());
    }
}

/// Like `dir_unsearchable_unreadable`, but uses ambient-authority APIs
/// to test underlying host functionality.
#[cfg(unix)]
#[test]
fn dir_unsearchable_unreadable_ambient() {
    use std::fs::DirBuilder;
    use std::os::unix::fs::DirBuilderExt;

    let dir = tempfile::tempdir().unwrap();

    let mut options = DirBuilder::new();
    options.mode(0o000);
    check!(options.create(dir.path().join("dir")));

    // FreeBSD seems able to open directories with 0o000 permissions.
    if cfg!(any(
        target_os = "android",
        target_os = "linux",
        target_os = "redox",
    )) {
        assert!(std::fs::File::open(dir.path().join("dir")).is_err());
        assert!(std::fs::read_dir(dir.path().join("dir")).is_err());
        assert!(std::fs::File::open(dir.path().join("dir/.")).is_err());
    } else if cfg!(target_os = "freebsd") {
        check!(std::fs::File::open(dir.path().join("dir")));
        check!(std::fs::read_dir(dir.path().join("dir")));
        check!(std::fs::File::open(dir.path().join("dir/.")));
    }
}

/// This test is the same as `symlink_hard_link` but uses `std::fs`'
/// ambient API instead of `cap_std`. The purpose of this test is to
/// confirm fundamentally OS-specific behaviors.
#[test]
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

#[test]
fn readdir_with_trailing_slashdot() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir("dir"));
    check!(tmpdir.create("dir/red"));
    check!(tmpdir.create("dir/green"));
    check!(tmpdir.create("dir/blue"));

    assert_eq!(check!(tmpdir.read_dir("dir")).count(), 3);
    assert_eq!(check!(tmpdir.read_dir("dir/")).count(), 3);
    assert_eq!(check!(tmpdir.read_dir("dir/.")).count(), 3);
}

#[test]
fn readdir_write() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir("dir"));
    assert!(tmpdir
        .open_with("dir", OpenOptions::new().write(true))
        .is_err());
    assert!(tmpdir
        .open_with("dir", OpenOptions::new().append(true))
        .is_err());
    assert!(tmpdir
        .open_with("dir/", OpenOptions::new().write(true))
        .is_err());
    assert!(tmpdir
        .open_with("dir/", OpenOptions::new().append(true))
        .is_err());

    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        use std::os::unix::fs::OpenOptionsExt;
        assert!(tmpdir
            .open_with(
                "dir",
                OpenOptions::new()
                    .write(true)
                    .custom_flags(rustix::fs::OFlags::DIRECTORY.bits() as i32)
            )
            .is_err());
        assert!(tmpdir
            .open_with(
                "dir",
                OpenOptions::new()
                    .append(true)
                    .custom_flags(rustix::fs::OFlags::DIRECTORY.bits() as i32)
            )
            .is_err());
    }
}

#[test]
fn maybe_dir() {
    use cap_fs_ext::OpenOptionsMaybeDirExt;

    let tmpdir = tmpdir();
    check!(tmpdir.create_dir("dir"));

    // Opening directories works on non-Windows platforms.
    #[cfg(not(windows))]
    check!(tmpdir.open("dir"));

    // Opening directories fails on Windows.
    #[cfg(windows)]
    assert!(tmpdir.open("dir").is_err());

    // Opening directories works on all platforms with `maybe_dir`.
    check!(tmpdir.open_with("dir", OpenOptions::new().read(true).maybe_dir(true)));
}

#[test]
fn sync() {
    use cap_fs_ext::OpenOptionsSyncExt;

    let tmpdir = tmpdir();
    check!(tmpdir.create("file"));

    check!(tmpdir.open_with("file", OpenOptions::new().write(true).sync(true)));
    check!(tmpdir.open_with("file", OpenOptions::new().write(true).dsync(true)));
    check!(tmpdir.open_with(
        "file",
        OpenOptions::new()
            .read(true)
            .write(true)
            .sync(true)
            .rsync(true)
    ));
    check!(tmpdir.open_with(
        "file",
        OpenOptions::new()
            .read(true)
            .write(true)
            .dsync(true)
            .rsync(true)
    ));
}

#[test]
fn reopen_fd() {
    use io_lifetimes::AsFilelike;
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir("subdir"));
    let tmpdir2 = check!(cap_std::fs::Dir::reopen_dir(&tmpdir.as_filelike()));
    assert!(tmpdir2.exists("subdir"));
}

#[test]
fn metadata_vs_std_fs() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir("dir"));
    let dir = check!(tmpdir.open_dir("dir"));
    let file = check!(dir.create("file"));

    let cap_std_dir = check!(dir.dir_metadata());
    let cap_std_file = check!(file.metadata());
    let cap_std_dir_entry = {
        let mut entries = check!(dir.entries());
        let entry = check!(entries.next().unwrap());
        assert_eq!(entry.file_name(), "file");
        assert!(entries.next().is_none(), "unexpected dir entry");
        check!(entry.metadata())
    };

    let std_dir = check!(dir.into_std_file().metadata());
    let std_file = check!(file.into_std().metadata());

    match std_dir.created() {
        Ok(_) => println!("std::fs supports file created times"),
        Err(e) => println!("std::fs doesn't support file created times: {}", e),
    }

    check_metadata(&std_dir, &cap_std_dir);
    check_metadata(&std_file, &cap_std_file);
    check_metadata(&std_file, &cap_std_dir_entry);
}

fn check_metadata(std: &std::fs::Metadata, cap: &cap_std::fs::Metadata) {
    assert_eq!(std.is_dir(), cap.is_dir());
    assert_eq!(std.is_file(), cap.is_file());
    assert_eq!(std.is_symlink(), cap.is_symlink());
    assert_eq!(std.file_type().is_dir(), cap.file_type().is_dir());
    assert_eq!(std.file_type().is_file(), cap.file_type().is_file());
    assert_eq!(std.file_type().is_symlink(), cap.file_type().is_symlink());
    #[cfg(unix)]
    {
        assert_eq!(
            std::os::unix::fs::FileTypeExt::is_block_device(&std.file_type()),
            cap_std::fs::FileTypeExt::is_block_device(&cap.file_type())
        );
        assert_eq!(
            std::os::unix::fs::FileTypeExt::is_char_device(&std.file_type()),
            cap_std::fs::FileTypeExt::is_char_device(&cap.file_type())
        );
        assert_eq!(
            std::os::unix::fs::FileTypeExt::is_fifo(&std.file_type()),
            cap_std::fs::FileTypeExt::is_fifo(&cap.file_type())
        );
        assert_eq!(
            std::os::unix::fs::FileTypeExt::is_socket(&std.file_type()),
            cap_std::fs::FileTypeExt::is_socket(&cap.file_type())
        );
    }

    assert_eq!(std.len(), cap.len());

    assert_eq!(std.permissions().readonly(), cap.permissions().readonly());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(std.permissions().mode(), cap.permissions().mode());
    }

    // If the standard library supports file modified/accessed/created times,
    // then cap-std should too.
    match std.modified() {
        Ok(expected) => assert_eq!(expected, check!(cap.modified()).into_std()),
        Err(e) => assert!(
            cap.modified().is_err(),
            "modified time should be error ({}), got {:#?}",
            e,
            cap.modified()
        ),
    }
    // The access times might be a little different due to either our own
    // or concurrent accesses.
    const ACCESS_TOLERANCE_SEC: u32 = 60;
    match std.accessed() {
        Ok(expected) => {
            let access_tolerance = std::time::Duration::from_secs(ACCESS_TOLERANCE_SEC.into());
            assert!(
                ((expected - access_tolerance)..(expected + access_tolerance))
                    .contains(&check!(cap.accessed()).into_std()),
                "std accessed {:#?}, cap accessed {:#?}",
                expected,
                cap.accessed()
            );
        }
        Err(e) => assert!(
            cap.accessed().is_err(),
            "accessed time should be error ({}), got {:#?}",
            e,
            cap.accessed()
        ),
    }
    match std.created() {
        Ok(expected) => assert_eq!(expected, check!(cap.created()).into_std()),
        Err(e) => {
            // An earlier bug returned the Unix epoch instead of `None` when
            // created times were unavailable. This tries to catch such errors,
            // while also allowing some targets to return valid created times
            // even when std doesn't.
            if let Ok(actual) = cap.created() {
                println!(
                    "std returned error for created time ({}) but got {:#?}",
                    e, actual
                );
                assert_ne!(actual, SystemClock::UNIX_EPOCH);
            }
        }
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        assert_eq!(std.dev(), cap.dev());
        assert_eq!(std.ino(), cap.ino());
        assert_eq!(std.mode(), cap.mode());
        assert_eq!(std.nlink(), cap.nlink());
        assert_eq!(std.uid(), cap.uid());
        assert_eq!(std.gid(), cap.gid());
        assert_eq!(std.rdev(), cap.rdev());
        assert_eq!(std.size(), cap.size());
        assert!(
            ((std.atime() - i64::from(ACCESS_TOLERANCE_SEC))
                ..(std.atime() + i64::from(ACCESS_TOLERANCE_SEC)))
                .contains(&cap.atime()),
            "std atime {}, cap atime {}",
            std.atime(),
            cap.atime()
        );
        assert!((0..1_000_000_000).contains(&cap.atime_nsec()));
        assert_eq!(std.mtime(), cap.mtime());
        assert_eq!(std.mtime_nsec(), cap.mtime_nsec());
        assert_eq!(std.ctime(), cap.ctime());
        assert_eq!(std.ctime_nsec(), cap.ctime_nsec());
        assert_eq!(std.blksize(), cap.blksize());
        assert_eq!(std.blocks(), cap.blocks());
    }
}
