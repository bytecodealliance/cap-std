mod sys_common;

use std::path::Path;
use sys_common::io::tmpdir;

#[test]
fn canonicalize_edge_cases() {
    let tmpdir = tmpdir();
    assert_eq!(check!(tmpdir.canonicalize(".")), Path::new(""));
    assert_eq!(check!(tmpdir.canonicalize("./")), Path::new(""));
    assert_eq!(check!(tmpdir.canonicalize("./.")), Path::new(""));
    assert_eq!(check!(tmpdir.canonicalize("")), Path::new(""));
    error_contains!(
        tmpdir.canonicalize("/"),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("/."),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("/.."),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("/./"),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("/../"),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("/../."),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("/./."),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("/./.."),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("/./../"),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("/../.."),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("/../../"),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize(".."),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("../"),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("../."),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("./.."),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("../.."),
        "a path led outside of the filesystem"
    );

    check!(tmpdir.create_dir_all("foo/bar"));
    assert_eq!(check!(tmpdir.canonicalize("foo")), Path::new("foo"));
    assert_eq!(check!(tmpdir.canonicalize("foo/")), Path::new("foo"));
    assert_eq!(check!(tmpdir.canonicalize("foo/")).to_str(), Some("foo"));
    assert_eq!(check!(tmpdir.canonicalize("foo/.")), Path::new("foo"));
    assert_eq!(check!(tmpdir.canonicalize("foo/./")), Path::new("foo"));
    assert_eq!(check!(tmpdir.canonicalize("foo/./")).to_str(), Some("foo"));
    assert_eq!(check!(tmpdir.canonicalize("foo/..")), Path::new(""));
    assert_eq!(check!(tmpdir.canonicalize("foo/../")), Path::new(""));
    assert_eq!(check!(tmpdir.canonicalize("foo/../.")), Path::new(""));
    assert_eq!(check!(tmpdir.canonicalize("foo/bar")), Path::new("foo/bar"));
    assert_eq!(
        check!(tmpdir.canonicalize("foo/bar/")),
        Path::new("foo/bar")
    );
    assert_eq!(
        check!(tmpdir.canonicalize("foo/bar/")).to_str(),
        Some("foo/bar")
    );
    assert_eq!(
        check!(tmpdir.canonicalize("foo/../foo/bar")),
        Path::new("foo/bar")
    );
    assert_eq!(
        check!(tmpdir.canonicalize("foo/../foo/bar/")),
        Path::new("foo/bar")
    );
    assert_eq!(
        check!(tmpdir.canonicalize("foo/../foo/bar/")).to_str(),
        Some("foo/bar")
    );
    error_contains!(
        tmpdir.canonicalize("foo/../.."),
        "a path led outside of the filesystem"
    );
    error_contains!(
        tmpdir.canonicalize("foo/../../"),
        "a path led outside of the filesystem"
    );
}
