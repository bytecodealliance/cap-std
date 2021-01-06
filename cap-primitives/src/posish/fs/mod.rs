mod c_str;
mod copy;
mod create_dir_unchecked;
mod dir_entry_inner;
mod dir_options_ext;
mod dir_utils;
#[cfg(not(target_os = "linux"))]
#[cfg(any(test, racy_asserts))]
mod file_path;
mod file_type_ext;
mod hard_link_unchecked;
mod is_read_write_impl;
mod is_root_dir;
#[cfg(racy_asserts)]
mod is_same_file;
mod metadata_ext;
mod oflags;
mod open_options_ext;
mod open_unchecked;
mod permissions_ext;
mod read_dir_inner;
mod read_link_unchecked;
mod remove_dir_all_impl;
mod remove_dir_unchecked;
mod remove_file_unchecked;
mod remove_open_dir_by_searching;
mod rename_unchecked;
#[cfg(not(target_os = "linux"))]
mod set_permissions_impl;
#[cfg(not(target_os = "linux"))]
mod set_times_impl;
mod stat_unchecked;
mod symlink_unchecked;
mod times;

pub(crate) mod errors;

// On Linux, use optimized implementations of `open` and `stat` using `openat2`
// and `O_PATH` when available.
//
// FreeBSD has a similar mechanism in `O_BENEATH`, however it appears to have
// different behavior on absolute and `..` paths in ways that make it unsuitable
// for `cap-std`'s style of sandboxing. For more information, see the bug filed
// upstream: https://bugs.freebsd.org/bugzilla/show_bug.cgi?id=248335
#[cfg(any(target_os = "macos", target_os = "ios"))]
#[cfg(any(test, racy_asserts))]
pub(crate) use crate::posish::darwin::fs::*;
#[cfg(target_os = "linux")]
pub(crate) use crate::posish::linux::fs::*;
#[cfg(not(target_os = "linux"))]
#[rustfmt::skip]
pub(crate) use crate::fs::{
    manually::open_entry as open_entry_impl,
    manually::open as open_impl,
    manually::stat as stat_impl,
    manually::canonicalize as canonicalize_impl,
    via_parent::set_times_nofollow as set_times_nofollow_impl,
};
#[cfg(any(target_os = "macos", target_os = "ios"))]
#[cfg(any(test, racy_asserts))]
pub(super) use file_path::file_path_by_ttyname_or_seaching;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "ios")))]
#[cfg(any(test, racy_asserts))]
pub(crate) use file_path::file_path_by_ttyname_or_seaching as file_path;
#[cfg(not(target_os = "linux"))]
pub(crate) use {set_permissions_impl::set_permissions_impl, set_times_impl::set_times_impl};

#[rustfmt::skip]
pub(crate) use crate::fs::{
    via_parent::hard_link as hard_link_impl,
    via_parent::create_dir as create_dir_impl,
    via_parent::read_link as read_link_impl,
    via_parent::rename as rename_impl,
    via_parent::remove_dir as remove_dir_impl,
    via_parent::symlink as symlink_impl,
    via_parent::remove_file as remove_file_impl,
    remove_open_dir_by_searching as remove_open_dir_impl,
};

#[allow(unused_imports)]
pub(crate) use c_str::c_str;
pub(crate) use copy::*;
pub(crate) use create_dir_unchecked::*;
pub(crate) use dir_entry_inner::*;
pub(crate) use dir_options_ext::*;
pub(crate) use dir_utils::*;
pub(crate) use file_type_ext::*;
pub(crate) use hard_link_unchecked::*;
pub(crate) use is_read_write_impl::*;
pub(crate) use is_root_dir::*;
#[cfg(racy_asserts)]
pub(crate) use is_same_file::*;
pub(crate) use metadata_ext::*;
pub(crate) use open_options_ext::*;
pub(crate) use open_unchecked::*;
pub(crate) use permissions_ext::*;
pub(crate) use read_dir_inner::*;
pub(crate) use read_link_unchecked::*;
pub(crate) use remove_dir_all_impl::*;
pub(crate) use remove_dir_unchecked::*;
pub(crate) use remove_file_unchecked::*;
pub(crate) use remove_open_dir_by_searching::*;
pub(crate) use rename_unchecked::*;
pub(crate) use stat_unchecked::*;
pub(crate) use symlink_unchecked::*;
#[allow(unused_imports)]
pub(crate) use times::{set_times_follow_unchecked, set_times_nofollow_unchecked};

// On Linux, there is a limit of 40 symlink expansions.
// Source: https://man7.org/linux/man-pages/man7/path_resolution.7.html
pub(crate) const MAX_SYMLINK_EXPANSIONS: u8 = 40;

pub(super) use oflags::*;

/// Test that `file_path` works on a tty path.
#[test]
fn tty_path() {
    #[cfg(unix)]
    use std::os::unix::fs::FileTypeExt;

    for path in &["/dev/tty", "/dev/stdin", "/dev/stdout", "/dev/stderr"] {
        // Not all host configurations have these, so only test them if we can
        // open and canonicalize them, and if they're not FIFOs, which some
        // OS's use for stdin/stdout/stderr.
        if let Ok(file) = std::fs::File::open(path) {
            if !file.metadata().unwrap().file_type().is_fifo() {
                if let Ok(canonical) = std::fs::canonicalize(path) {
                    assert_eq!(
                        file_path(&file)
                            .as_ref()
                            .map(std::fs::canonicalize)
                            .map(Result::unwrap),
                        Some(canonical)
                    );
                }
            }
        }
    }
}
