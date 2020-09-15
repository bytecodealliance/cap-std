#[macro_use]
mod sys_common;

use cap_dir_ext::DirExt;
use cap_std::time::{SystemClock, SystemTime};
use sys_common::io::tmpdir;

fn modified_time(meta: cap_std::fs::Metadata) -> SystemTime {
    meta.modified().unwrap()
}

#[test]
fn basic_times() {
    let tmpdir = tmpdir();
    check!(tmpdir.create("file"));
    check!(tmpdir.create_dir("dir"));
    check!(tmpdir.symlink_file("file", "file_symlink_file"));
    check!(tmpdir.symlink_dir("dir", "dir_symlink_dir"));

    let file_time = SystemClock::UNIX_EPOCH;
    check!(tmpdir.set_times("file", None, Some(file_time.into())));
    assert_eq!(modified_time(check!(tmpdir.metadata("file"))), file_time);
    assert_eq!(
        modified_time(check!(tmpdir.metadata("file_symlink_file"))),
        file_time
    );

    let dir_time = SystemClock::UNIX_EPOCH;
    check!(tmpdir.set_times("dir", None, Some(dir_time.into())));
    assert_eq!(modified_time(check!(tmpdir.metadata("dir"))), dir_time);
    assert_eq!(
        modified_time(check!(tmpdir.metadata("dir_symlink_dir"))),
        dir_time
    );
}

#[test]
fn symlink_times() {
    let tmpdir = tmpdir();
    check!(tmpdir.create("file"));
    check!(tmpdir.create_dir("dir"));
    check!(tmpdir.symlink_file("file", "file_symlink_file"));
    check!(tmpdir.symlink_dir("dir", "dir_symlink_dir"));

    let file_time = SystemClock::UNIX_EPOCH;
    check!(tmpdir.set_times("file_symlink_file", None, Some(file_time.into())));
    assert_eq!(modified_time(check!(tmpdir.metadata("file"))), file_time);
    assert_eq!(
        modified_time(check!(tmpdir.metadata("file_symlink_file"))),
        file_time
    );
    assert_eq!(
        modified_time(check!(tmpdir.symlink_metadata("file"))),
        file_time
    );
    assert_ne!(
        modified_time(check!(tmpdir.symlink_metadata("file_symlink_file"))),
        file_time
    );

    let dir_time = SystemClock::UNIX_EPOCH;
    dbg!("test");
    check!(tmpdir.set_times("dir_symlink_dir", None, Some(file_time.into())));
    dbg!("test");
    assert_eq!(modified_time(check!(tmpdir.metadata("dir"))), dir_time);
    assert_eq!(
        modified_time(check!(tmpdir.metadata("dir_symlink_dir"))),
        dir_time
    );
    assert_eq!(
        modified_time(check!(tmpdir.symlink_metadata("dir"))),
        dir_time
    );
    assert_ne!(
        modified_time(check!(tmpdir.symlink_metadata("dir_symlink_dir"))),
        dir_time
    );
}

#[test]
fn symlink_itself_times() {
    let tmpdir = tmpdir();
    check!(tmpdir.create("file"));
    check!(tmpdir.create_dir("dir"));
    check!(tmpdir.symlink_file("file", "file_symlink_file"));
    check!(tmpdir.symlink_dir("dir", "dir_symlink_dir"));

    let file_time = SystemClock::UNIX_EPOCH;
    check!(tmpdir.set_symlink_times("file_symlink_file", None, Some(file_time.into())));
    assert_ne!(modified_time(check!(tmpdir.metadata("file"))), file_time);
    assert_ne!(
        modified_time(check!(tmpdir.metadata("file_symlink_file"))),
        file_time
    );
    assert_ne!(
        modified_time(check!(tmpdir.symlink_metadata("file"))),
        file_time
    );
    assert_eq!(
        modified_time(check!(tmpdir.symlink_metadata("file_symlink_file"))),
        file_time
    );

    let dir_time = SystemClock::UNIX_EPOCH;
    check!(tmpdir.set_symlink_times("dir_symlink_dir", None, Some(file_time.into())));
    assert_ne!(modified_time(check!(tmpdir.metadata("dir"))), dir_time);
    assert_ne!(
        modified_time(check!(tmpdir.metadata("dir_symlink_dir"))),
        dir_time
    );
    assert_ne!(
        modified_time(check!(tmpdir.symlink_metadata("dir"))),
        dir_time
    );
    assert_eq!(
        modified_time(check!(tmpdir.symlink_metadata("dir_symlink_dir"))),
        dir_time
    );
}
