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

    // Now that we've dropped the handle, the same operations should succeed.
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

    // Now that we've dropped the handle, the same operations should succeed.
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

    // Now that we've dropped the handle, the same operations should succeed.
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
    use cap_std::{ambient_authority, fs::Dir};

    let ambient_dir = tempfile::tempdir().unwrap();

    let tmpdir = check!(Dir::open_ambient_dir(
        ambient_dir.path(),
        ambient_authority()
    ));
    check!(tmpdir.create_dir("aaa"));

    let dir = check!(Dir::open_ambient_dir(
        ambient_dir.path().join("aaa"),
        ambient_authority()
    ));

    // Attempts to remove or rename the open directory should fail.
    tmpdir.remove_dir("aaa").unwrap_err();
    tmpdir.rename("aaa", &tmpdir, "zzz").unwrap_err();

    drop(dir);

    // Now that we've dropped the handle, the same operations should succeed.
    check!(tmpdir.rename("aaa", &tmpdir, "xxx"));
    check!(tmpdir.remove_dir("xxx"));
}

#[test]
#[cfg(windows)]
fn windows_open_special() {
    let tmpdir = tmpdir();

    // Opening any of these should fail.
    for device in &[
        "CON", "PRN", "AUX", "NUL", "COM0", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
        "COM8", "COM9", "LPT0", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8",
        "LPT9",
    ] {
        tmpdir.open(device).unwrap_err();
        tmpdir.open(&format!(".\\{}", device)).unwrap_err();
        tmpdir.open(&format!("{}.ext", device)).unwrap_err();
        tmpdir.open(&format!(".\\{}.ext", device)).unwrap_err();

        let mut options = cap_std::fs::OpenOptions::new();
        options.write(true);
        tmpdir.open_with(device, &options).unwrap_err();
        tmpdir
            .open_with(&format!(".\\{}", device), &options)
            .unwrap_err();
        tmpdir
            .open_with(&format!("{}.ext", device), &options)
            .unwrap_err();
        tmpdir
            .open_with(&format!(".\\{}.ext", device), &options)
            .unwrap_err();

        tmpdir.create(device).unwrap_err();
        tmpdir.create(&format!(".\\{}", device)).unwrap_err();
        tmpdir.create(&format!("{}.ext", device)).unwrap_err();
        tmpdir.create(&format!(".\\{}.ext", device)).unwrap_err();
    }
}
