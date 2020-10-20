//! On Windows, cap-std uses the technique of looking up absolute paths for
//! directory handles. This would be racy, except that cap-std uses Windows'
//! sharing modes to prevent open directories from being removed or renamed.
//! Test that this works.

#[cfg(windows)]
#[macro_use]
mod sys_common;

#[cfg(windows)]
use sys_common::io::tmpdir;

#[test]
#[cfg(windows)]
fn windows_open_one() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir("aaa"));

    let dir = check!(tmpdir.open_dir("aaa"));

    // Attempts to remove or rename the open directory should fail.
    tmpdir.remove_dir("aaa").unwrap_err();
    tmpdir.rename("aaa", &tmpdir, "zzz").unwrap_err();

    drop(dir);

    // Now that we've droped the handle, the same operations should succeed.
    check!(tmpdir.rename("aaa", &tmpdir, "xxx"));
    check!(tmpdir.remove_dir("xxx"));
}

#[test]
#[cfg(windows)]
fn windows_open_multiple() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir_all("aaa/bbb"));

    let dir = check!(tmpdir.open_dir("aaa/bbb"));

    // Attempts to remove or rename any component of the open directory should fail.
    tmpdir.remove_dir("aaa/bbb").unwrap_err();
    tmpdir.remove_dir("aaa").unwrap_err();
    tmpdir.rename("aaa/bbb", &tmpdir, "aaa/yyy").unwrap_err();
    tmpdir.rename("aaa", &tmpdir, "zzz").unwrap_err();

    drop(dir);

    // Now that we've droped the handle, the same operations should succeed.
    check!(tmpdir.rename("aaa/bbb", &tmpdir, "aaa/www"));
    check!(tmpdir.rename("aaa", &tmpdir, "xxx"));
    check!(tmpdir.remove_dir("xxx/www"));
    check!(tmpdir.remove_dir("xxx"));
}

/// Like `windows_open_multiple`, but does so within a directory that we
/// can close and then independently mutate.
#[test]
#[cfg(windows)]
fn windows_open_tricky() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir("qqq"));

    let qqq = check!(tmpdir.open_dir("qqq"));
    check!(qqq.create_dir_all("aaa/bbb"));

    let dir = check!(qqq.open_dir("aaa/bbb"));

    // Now drop `qqq`.
    drop(qqq);

    // Attempts to remove or rename any component of the open directory should fail.
    dir.remove_dir("aaa/bbb").unwrap_err();
    dir.remove_dir("aaa").unwrap_err();
    dir.rename("aaa/bbb", &tmpdir, "aaa/yyy").unwrap_err();
    dir.rename("aaa", &tmpdir, "zzz").unwrap_err();
    tmpdir.remove_dir("qqq/aaa/bbb").unwrap_err();
    tmpdir.remove_dir("qqq/aaa").unwrap_err();
    tmpdir.remove_dir("qqq").unwrap_err();
    tmpdir
        .rename("qqq/aaa/bbb", &tmpdir, "qqq/aaa/yyy")
        .unwrap_err();
    tmpdir.rename("qqq/aaa", &tmpdir, "qqq/zzz").unwrap_err();
    tmpdir.rename("qqq", &tmpdir, "vvv").unwrap_err();

    drop(dir);

    // Now that we've droped the handle, the same operations should succeed.
    check!(tmpdir.rename("qqq/aaa/bbb", &tmpdir, "qqq/aaa/www"));
    check!(tmpdir.rename("qqq/aaa", &tmpdir, "qqq/xxx"));
    check!(tmpdir.rename("qqq", &tmpdir, "uuu"));
    check!(tmpdir.remove_dir("uuu/xxx/www"));
    check!(tmpdir.remove_dir("uuu/xxx"));
    check!(tmpdir.remove_dir("uuu"));
}

/// Like `windows_open_one` but uses `open_ambient_dir` instead of `open_dir`.
#[test]
#[cfg(windows)]
fn windows_open_ambient() {
    let ambient_dir = tempfile::tempdir().unwrap();

    let tmpdir = check!(unsafe { cap_std::fs::Dir::open_ambient_dir(ambient_dir.path()) });
    check!(tmpdir.create_dir("aaa"));

    let dir = check!(unsafe { cap_std::fs::Dir::open_ambient_dir(ambient_dir.path().join("aaa")) });

    // Attempts to remove or rename the open directory should fail.
    tmpdir.remove_dir("aaa").unwrap_err();
    tmpdir.rename("aaa", &tmpdir, "zzz").unwrap_err();

    drop(dir);

    // Now that we've droped the handle, the same operations should succeed.
    check!(tmpdir.rename("aaa", &tmpdir, "xxx"));
    check!(tmpdir.remove_dir("xxx"));
}
