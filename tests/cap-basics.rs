mod sys_common;

use sys_common::io::tmpdir;

#[test]
fn cap_smoke_test() {
    let tmpdir = tmpdir();
    check!(tmpdir.create_dir_all("dir/inner"));
    check!(tmpdir.write_file("red.txt", b"hello world\n"));
    check!(tmpdir.write_file("dir/green.txt", b"goodmight moon\n"));
    check!(tmpdir.write_file("dir/inner/blue.txt", b"hey mars\n"));

    let inner = check!(tmpdir.open_dir("dir/inner"));

    check!(tmpdir.open_file("red.txt"));
    error!(tmpdir.open_file("blue.txt"), "No such file");
    error!(tmpdir.open_file("green.txt"), "No such file");

    check!(tmpdir.open_file("./red.txt"));
    error!(tmpdir.open_file("./blue.txt"), "No such file");
    error!(tmpdir.open_file("./green.txt"), "No such file");

    error!(tmpdir.open_file("dir/red.txt"), "No such file");
    check!(tmpdir.open_file("dir/green.txt"));
    error!(tmpdir.open_file("dir/blue.txt"), "No such file");

    error!(tmpdir.open_file("dir/inner/red.txt"), "No such file");
    error!(tmpdir.open_file("dir/inner/green.txt"), "No such file");
    check!(tmpdir.open_file("dir/inner/blue.txt"));

    check!(tmpdir.open_file("dir/../red.txt"));
    check!(tmpdir.open_file("dir/inner/../../red.txt"));
    check!(tmpdir.open_file("dir/inner/../inner/../../red.txt"));

    error!(inner.open_file("red.txt"), "No such file");
    error!(inner.open_file("green.txt"), "No such file");
    error!(
        inner.open_file("../inner/blue.txt"),
        "a path led outside of the filesystem"
    );
    error!(
        inner.open_file("../inner/red.txt"),
        "a path led outside of the filesystem"
    );

    check!(inner.open_dir(""));
    error!(
        inner.open_dir("/"),
        "an absolute path could not be resolved"
    );
    error!(
        inner.open_dir("/etc/services"),
        "an absolute path could not be resolved"
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
    check!(tmpdir.write_file("red.txt", b"hello world\n"));
    check!(tmpdir.write_file("dir/green.txt", b"goodmight moon\n"));
    check!(tmpdir.write_file("dir/inner/blue.txt", b"hey mars\n"));

    let inner = check!(tmpdir.open_dir("dir/next"));

    check!(tmpdir.symlink("dir", "link"));
    check!(tmpdir.symlink("does_not_exist", "badlink"));

    check!(tmpdir.open_file("link/../red.txt"));
    check!(tmpdir.open_file("link/green.txt"));
    check!(tmpdir.open_file("link/inner/blue.txt"));
    error!(tmpdir.open_file("link/red.txt"), "No such file");
    error!(tmpdir.open_file("link/../green.txt"), "No such file");

    check!(tmpdir.open_file("./dir/.././/link/..///./red.txt"));
    error!(
        tmpdir.open_file("./dir/.././/link/..///./not.txt"),
        "No such file"
    );
    check!(tmpdir.open_file("link/inner/../inner/../../red.txt"));
    check!(inner.open_file("../inner/../inner/../../link/other.txt"));

    check!(tmpdir.open_file("link/other.txt"));
    error!(tmpdir.open_file("badlink/../red.txt"), "No such file");
}

#[test]
#[ignore] // symlinks not yet implemented
fn symlink_loop() {
    let tmpdir = tmpdir();
    check!(tmpdir.symlink("link", "link"));
    error!(tmpdir.open_file("link"), "No such file");
}

#[cfg(linux)]
#[test]
fn proc_self_fd() {
    let fd = check!(File::open("/proc/self/fd"));
    let dir = cap_std::fs::Dir::from_std_file(fd);
    error!(dir.open_file("0"), "No such file");
}
