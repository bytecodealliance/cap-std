//! Following [`std`], we don't carry workarounds for Linux versions
//! older than 2.6.32.
//!
//! [`std`]: https://github.com/rust-lang/rust/pull/74163

pub(crate) mod fs;

/// Test whether the version of the Linux kernel underneath us is at least the
/// given major and minor version.
#[cfg(target_os = "android")]
pub(crate) fn linux_version_at_least(major: u32, minor: u32) -> bool {
    let uname = rustix::process::uname();
    let release = uname.release().to_bytes();
    if let Some((current_major, current_minor)) = linux_major_minor(release) {
        if major > current_major || (major == current_major && minor >= current_minor) {
            return true;
        }
    }
    false
}

/// Extract the major and minor values from a Linux `release` string.
#[cfg(target_os = "android")]
fn linux_major_minor(release: &[u8]) -> Option<(u32, u32)> {
    let mut parts = release.split(|b| *b == b'.');
    if let Some(major) = parts.next() {
        if let Ok(major) = std::str::from_utf8(major) {
            if let Ok(major) = major.parse::<u32>() {
                if let Some(minor) = parts.next() {
                    if let Ok(minor) = std::str::from_utf8(minor) {
                        if let Ok(minor) = minor.parse::<u32>() {
                            return Some((major, minor));
                        }
                    }
                }
            }
        }
    }

    None
}

#[cfg(target_os = "android")]
#[test]
fn test_linux_major_minor() {
    assert_eq!(linux_major_minor(b"5.11.0-5489-something"), Some((5, 11)));
    assert_eq!(linux_major_minor(b"5.10.0-9-whatever"), Some((5, 10)));
    assert_eq!(linux_major_minor(b"5.0.0"), Some((5, 0)));
    assert_eq!(linux_major_minor(b"4.11.0"), Some((4, 11)));
    assert_eq!(linux_major_minor(b"4.10.99"), Some((4, 10)));
    assert_eq!(linux_major_minor(b"4.10.0"), Some((4, 10)));
    assert_eq!(linux_major_minor(b"6.0.0"), Some((6, 0)));
    assert_eq!(linux_major_minor(b"5.6.0"), Some((5, 6)));
    assert_eq!(linux_major_minor(b"5.5.99"), Some((5, 5)));
    assert_eq!(linux_major_minor(b"5.5.0"), Some((5, 5)));
    assert_eq!(linux_major_minor(b"2.6.34"), Some((2, 6)));
    assert_eq!(linux_major_minor(b""), None);
    assert_eq!(linux_major_minor(b"linux-2.6.32"), None);
}
