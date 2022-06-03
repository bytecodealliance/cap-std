// This file contains tests for `cap_fs_ext::MetatadaExt`.

#[macro_use]
mod sys_common;

use cap_fs_ext::{DirExt, MetadataExt};
use sys_common::io::tmpdir;
use sys_common::symlink_supported;

#[test]
fn test_metadata_ext() {
    let tmpdir = tmpdir();
    let a = check!(tmpdir.create("a"));
    let b = check!(tmpdir.create("b"));
    let tmpdir_metadata = check!(tmpdir.dir_metadata());
    let a_metadata = check!(a.metadata());
    let b_metadata = check!(b.metadata());
    let a_dir_metadata = check!(tmpdir.metadata("a"));
    let b_dir_metadata = check!(tmpdir.metadata("b"));
    let a_symlink_metadata = check!(tmpdir.symlink_metadata("a"));
    let b_symlink_metadata = check!(tmpdir.symlink_metadata("b"));

    // The directory and files inside it should be on the same device.
    assert_eq!(tmpdir_metadata.dev(), a_metadata.dev());
    assert_eq!(a_metadata.dev(), b_metadata.dev());

    // They should all have distinct inodes.
    assert_ne!(tmpdir_metadata.ino(), a_metadata.ino());
    assert_ne!(tmpdir_metadata.ino(), b_metadata.ino());
    assert_ne!(a_metadata.ino(), b_metadata.ino());

    // The files should start with just one link.
    assert_eq!(a_metadata.nlink(), 1);
    assert_eq!(b_metadata.nlink(), 1);

    // Add another link and check for it.
    check!(tmpdir.hard_link("b", &tmpdir, "c"));
    let b_metadata = check!(b.metadata());
    assert_eq!(b_metadata.nlink(), 2);

    // Check that the metadata has dev/nlink/ino.
    tmpdir_metadata.dev();
    tmpdir_metadata.nlink();
    tmpdir_metadata.ino();
    a_metadata.dev();
    a_metadata.nlink();
    a_metadata.ino();
    b_metadata.dev();
    b_metadata.nlink();
    b_metadata.ino();
    a_dir_metadata.dev();
    a_dir_metadata.nlink();
    a_dir_metadata.ino();
    b_dir_metadata.dev();
    b_dir_metadata.nlink();
    b_dir_metadata.ino();
    a_symlink_metadata.dev();
    a_symlink_metadata.nlink();
    a_symlink_metadata.ino();
    b_symlink_metadata.dev();
    b_symlink_metadata.nlink();
    b_symlink_metadata.ino();

    if symlink_supported() {
        check!(DirExt::symlink_file(&*tmpdir, "b", "d"));
        let d_metadata = check!(tmpdir.metadata("d"));
        let d_symlink_metadata = check!(tmpdir.symlink_metadata("d"));

        d_metadata.dev();
        d_metadata.nlink();
        d_metadata.ino();
        d_symlink_metadata.dev();
        d_symlink_metadata.nlink();
        d_symlink_metadata.ino();

        assert_ne!(
            (d_symlink_metadata.ino(), d_symlink_metadata.dev()),
            (b_metadata.ino(), b_metadata.dev())
        );
        assert_eq!(
            (d_metadata.ino(), d_metadata.dev()),
            (b_metadata.ino(), b_metadata.dev())
        );
    }
}

#[test]
fn test_metadata_ext_created() {
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

    // If the standard library supports file creation times, then cap-std
    // should too.
    if let Ok(expected) = std_dir.created() {
        println!("std::fs supports file created times");
        assert_eq!(expected, check!(cap_std_dir.created()).into_std());
    } else {
        println!("std::fs doesn't support file created times");
    }
    if let Ok(expected) = std_file.created() {
        assert_eq!(expected, check!(cap_std_file.created()).into_std());
        assert_eq!(expected, check!(cap_std_dir_entry.created()).into_std());
    }
}
