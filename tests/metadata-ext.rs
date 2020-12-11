// This file contains tests for `cap_fs_ext::MetatadaExt`.

#[macro_use]
mod sys_common;

use cap_fs_ext::MetadataExt;
use sys_common::io::tmpdir;

#[test]
fn test_cap_fs_ext() {
    let tmpdir = tmpdir();
    let a = check!(tmpdir.create("a"));
    let b = check!(tmpdir.create("b"));
    let tmpdir_metadata = check!(tmpdir.dir_metadata());
    let a_metadata = check!(a.metadata());
    let b_metadata = check!(b.metadata());

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
}
