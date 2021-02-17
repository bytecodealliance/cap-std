#![cfg(windows)]

#[macro_use]
mod sys_common;

use sys_common::io::tmpdir;

/// Windows strips trailing dots and trailing whitespace.
#[test]
fn trailing_dots() {
    let tmpdir = tmpdir();

    tmpdir.create("hello.txt").unwrap();
    tmpdir.open("hello.txt").unwrap();
    tmpdir.open("hello.txt.").unwrap();
    tmpdir.open("hello.txt..").unwrap();
    tmpdir.open("hello.txt...").unwrap();
    tmpdir.open("hello.txt ").unwrap();
    tmpdir.open("hello.txt  ").unwrap();
    tmpdir.open("hello.txt   ").unwrap();
    tmpdir.open("hello.txt . . .").unwrap();
    tmpdir.open("hello.txt. . . ").unwrap();
    tmpdir.open("hello.txt*").unwrap_err();
    tmpdir.open("hello.txt*.").unwrap_err();
    tmpdir.open("hello.txt*..").unwrap_err();
    tmpdir.open("hello.txt*...").unwrap_err();
}

/// This test is the same as `trailing_dots` but uses `std::fs`'
/// ambient API instead of `cap_std`. The purpose of this test is to
/// confirm fundamentally OS-specific differences.
#[test]
fn ambient_trailing_dots() {
    use std::fs;

    let dir = tempfile::tempdir().unwrap();

    fs::File::create(dir.path().join("hello.txt")).unwrap();
    fs::File::open(dir.path().join("hello.txt")).unwrap();
    fs::File::open(dir.path().join("hello.txt.")).unwrap();
    fs::File::open(dir.path().join("hello.txt..")).unwrap();
    fs::File::open(dir.path().join("hello.txt...")).unwrap();
    fs::File::open(dir.path().join("hello.txt ")).unwrap();
    fs::File::open(dir.path().join("hello.txt  ")).unwrap();
    fs::File::open(dir.path().join("hello.txt   ")).unwrap();
    fs::File::open(dir.path().join("hello.txt . . .")).unwrap();
    fs::File::open(dir.path().join("hello.txt. . . ")).unwrap();
    fs::File::open(dir.path().join("hello.txt*")).unwrap_err();
    fs::File::open(dir.path().join("hello.txt*.")).unwrap_err();
    fs::File::open(dir.path().join("hello.txt*..")).unwrap_err();
    fs::File::open(dir.path().join("hello.txt*...")).unwrap_err();
}

/// Like `trailing_dots`, but with a directory instead of a file, and adds
/// a few extra cases for mixed trailing dots, spaces, and slashes.
#[test]
fn trailing_dots_dir() {
    let tmpdir = tmpdir();

    tmpdir.create_dir("dir").unwrap();
    tmpdir.open_dir("dir").unwrap();
    tmpdir.open_dir("dir.").unwrap();
    tmpdir.open_dir("dir..").unwrap();
    tmpdir.open_dir("dir...").unwrap();
    tmpdir.open_dir("dir ").unwrap();
    tmpdir.open_dir("dir  ").unwrap();
    tmpdir.open_dir("dir   ").unwrap();
    tmpdir.open_dir("dir . . .").unwrap();
    tmpdir.open_dir("dir. . . ").unwrap();
    tmpdir.open_dir("dir/").unwrap();
    tmpdir.open_dir("dir/.").unwrap();
    tmpdir.open_dir("dir./").unwrap();
    tmpdir.open_dir("dir./.").unwrap();
    tmpdir.open_dir("dir/ ").unwrap();
    tmpdir.open_dir("dir /").unwrap();
    tmpdir.open_dir("dir / ").unwrap();
    tmpdir.open_dir("dir./ ").unwrap();
    tmpdir.open_dir("dir /.").unwrap();
    tmpdir.open_dir("dir/ . . .").unwrap();
    tmpdir.open_dir("dir/. . . ").unwrap();
    tmpdir.open_dir("dir. . . / . . .").unwrap_err();
    tmpdir.open_dir("dir . . ./. . . ").unwrap_err();
    tmpdir.open_dir("dir/ . . ./. . . / . . .").unwrap_err();
    tmpdir.open_dir("dir/. . . / . . ./. . . ").unwrap_err();
    tmpdir
        .open_dir("dir. . . / . . ./. . . / . . .")
        .unwrap_err();
    tmpdir
        .open_dir("dir . . ./. . . / . . ./. . . ")
        .unwrap_err();
    tmpdir.open_dir("dir*").unwrap_err();
    tmpdir.open_dir("dir*.").unwrap_err();
    tmpdir.open_dir("dir*..").unwrap_err();
    tmpdir.open_dir("dir*...").unwrap_err();
}

#[test]
fn ambient_trailing_dots_dir() {
    use std::os::windows::fs::OpenOptionsExt;
    use std::{fs, io, path::Path};
    use winapi::um::winbase::FILE_FLAG_BACKUP_SEMANTICS;

    fn open_dir(s: &Path) -> io::Result<fs::File> {
        fs::OpenOptions::new()
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
            .read(true)
            .open(s)
    }

    let dir = tempfile::tempdir().unwrap();

    fs::create_dir(dir.path().join("dir")).unwrap();
    open_dir(&dir.path().join("dir")).unwrap();
    open_dir(&dir.path().join("dir.")).unwrap();
    open_dir(&dir.path().join("dir..")).unwrap();
    open_dir(&dir.path().join("dir...")).unwrap();
    open_dir(&dir.path().join("dir ")).unwrap();
    open_dir(&dir.path().join("dir  ")).unwrap();
    open_dir(&dir.path().join("dir   ")).unwrap();
    open_dir(&dir.path().join("dir . . .")).unwrap();
    open_dir(&dir.path().join("dir. . . ")).unwrap();
    open_dir(&dir.path().join("dir/")).unwrap();
    open_dir(&dir.path().join("dir/.")).unwrap();
    open_dir(&dir.path().join("dir./")).unwrap();
    open_dir(&dir.path().join("dir./.")).unwrap();
    open_dir(&dir.path().join("dir/ ")).unwrap();
    open_dir(&dir.path().join("dir /")).unwrap();
    open_dir(&dir.path().join("dir / ")).unwrap();
    open_dir(&dir.path().join("dir./ ")).unwrap();
    open_dir(&dir.path().join("dir /.")).unwrap();
    open_dir(&dir.path().join("dir/ . . .")).unwrap();
    open_dir(&dir.path().join("dir/. . . ")).unwrap();
    open_dir(&dir.path().join("dir. . . / . . .")).unwrap_err();
    open_dir(&dir.path().join("dir . . ./. . . ")).unwrap_err();
    open_dir(&dir.path().join("dir/ . . ./. . . / . . .")).unwrap_err();
    open_dir(&dir.path().join("dir/. . . / . . ./. . . ")).unwrap_err();
    open_dir(&dir.path().join("dir. . . / . . ./. . . / . . .")).unwrap_err();
    open_dir(&dir.path().join("dir . . ./. . . / . . ./. . . ")).unwrap_err();
    open_dir(&dir.path().join("dir*")).unwrap_err();
    open_dir(&dir.path().join("dir*.")).unwrap_err();
    open_dir(&dir.path().join("dir*..")).unwrap_err();
    open_dir(&dir.path().join("dir*...")).unwrap_err();
}
