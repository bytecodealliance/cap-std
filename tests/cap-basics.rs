mod sys_common;

use sys_common::io::tmpdir;

#[test]
fn cap_smoke_test() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir_all("dir/inner"));
    check!(tmpdir.write("red.txt", b"hello world\n"));
    check!(tmpdir.write("dir/green.txt", b"goodmight moon\n"));
    check!(tmpdir.write("dir/inner/blue.txt", b"hey mars\n"));

    let inner = check!(tmpdir.open_dir("dir/inner"));

    check!(tmpdir.open("red.txt"));
    error!(tmpdir.open("blue.txt"), "No such file");
    error!(tmpdir.open("green.txt"), "No such file");

    check!(tmpdir.open("./red.txt"));
    error!(tmpdir.open("./blue.txt"), "No such file");
    error!(tmpdir.open("./green.txt"), "No such file");

    error!(tmpdir.open("dir/red.txt"), "No such file");
    check!(tmpdir.open("dir/green.txt"));
    error!(tmpdir.open("dir/blue.txt"), "No such file");

    error!(tmpdir.open("dir/inner/red.txt"), "No such file");
    error!(tmpdir.open("dir/inner/green.txt"), "No such file");
    check!(tmpdir.open("dir/inner/blue.txt"));

    check!(tmpdir.open("dir/../red.txt"));
    check!(tmpdir.open("dir/inner/../../red.txt"));
    check!(tmpdir.open("dir/inner/../inner/../../red.txt"));

    error!(inner.open("red.txt"), "No such file");
    error!(inner.open("green.txt"), "No such file");
    error!(
        inner.open("../inner/blue.txt"),
        "a path led outside of the filesystem"
    );
    error!(
        inner.open("../inner/red.txt"),
        "a path led outside of the filesystem"
    );

    check!(inner.open_dir(""));
    error!(inner.open_dir("/"), "a path led outside of the filesystem");
    error!(
        inner.open_dir("/etc/services"),
        "a path led outside of the filesystem"
    );
    check!(inner.open_dir("."));
    check!(inner.open_dir("./"));
    check!(inner.open_dir("./."));
    error!(inner.open_dir(".."), "a path led outside of the filesystem");
    error!(
        inner.open_dir("../"),
        "a path led outside of the filesystem"
    );
    error!(
        inner.open_dir("../."),
        "a path led outside of the filesystem"
    );
    error!(
        inner.open_dir("./.."),
        "a path led outside of the filesystem"
    );
}

#[test]
#[ignore] // symlinks not yet implemented
fn symlinks() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir_all("dir/next/inner"));
    check!(tmpdir.write("red.txt", b"hello world\n"));
    check!(tmpdir.write("dir/green.txt", b"goodmight moon\n"));
    check!(tmpdir.write("dir/inner/blue.txt", b"hey mars\n"));

    let inner = check!(tmpdir.open_dir("dir/next"));

    check!(tmpdir.symlink("dir", "link"));
    check!(tmpdir.symlink("does_not_exist", "badlink"));

    check!(tmpdir.open("link/../red.txt"));
    check!(tmpdir.open("link/green.txt"));
    check!(tmpdir.open("link/inner/blue.txt"));
    error!(tmpdir.open("link/red.txt"), "No such file");
    error!(tmpdir.open("link/../green.txt"), "No such file");

    check!(tmpdir.open("./dir/.././/link/..///./red.txt"));
    error!(
        tmpdir.open("./dir/.././/link/..///./not.txt"),
        "No such file"
    );
    check!(tmpdir.open("link/inner/../inner/../../red.txt"));
    check!(inner.open("../inner/../inner/../../link/other.txt"));

    check!(tmpdir.open("link/other.txt"));
    error!(tmpdir.open("badlink/../red.txt"), "No such file");
}

#[test]
#[ignore] // symlinks not yet implemented
fn symlink_loop() {
    let tmpdir = tmpdir();
    check!(tmpdir.symlink("link", "link"));
    error!(tmpdir.open("link"), "No such file");
}

#[cfg(linux)]
#[test]
fn proc_self_fd() {
    let fd = check!(File::open("/proc/self/fd"));
    let dir = cap_std::fs::Dir::from_std_file(fd);
    error!(dir.open("0"), "No such file");
}
