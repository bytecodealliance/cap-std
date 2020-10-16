mod c_str;
mod copy;
mod dir_entry_inner;
mod dir_options_ext;
mod dir_utils;
mod file_type_ext;
mod hard_link_unchecked;
mod is_read_write_impl;
mod is_root_dir;
#[cfg(racy_asserts)]
mod is_same_file;
mod metadata_ext;
mod mkdir_unchecked;
mod oflags;
mod open_options_ext;
mod open_unchecked;
mod permissions_ext;
mod read_dir_inner;
mod readlink_unchecked;
mod remove_dir_all_impl;
mod remove_open_dir_by_searching;
mod rename_unchecked;
mod rmdir_unchecked;
#[cfg(not(target_os = "linux"))]
mod set_permissions_impl;
#[cfg(not(target_os = "linux"))]
mod set_times_impl;
mod stat_unchecked;
mod symlink_unchecked;
mod times;
mod unlink_unchecked;

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
    dir_options as dir_path_options,
};
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "ios")))]
#[cfg(any(test, racy_asserts))]
pub(crate) use crate::fs::file_path_by_searching as file_path;
#[cfg(not(target_os = "linux"))]
pub(crate) use {set_permissions_impl::set_permissions_impl, set_times_impl::set_times_impl};

#[rustfmt::skip]
pub(crate) use crate::fs::{
    via_parent::hard_link as hard_link_impl,
    via_parent::mkdir as mkdir_impl,
    via_parent::readlink as readlink_impl,
    via_parent::rename as rename_impl,
    via_parent::rmdir as rmdir_impl,
    via_parent::symlink as symlink_impl,
    via_parent::unlink as unlink_impl,
    remove_open_dir_by_searching as remove_open_dir_impl,
};

#[allow(unused_imports)]
pub(crate) use c_str::c_str;
pub(crate) use copy::*;
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
pub(crate) use mkdir_unchecked::*;
pub(crate) use open_options_ext::*;
pub(crate) use open_unchecked::*;
pub(crate) use permissions_ext::*;
pub(crate) use read_dir_inner::*;
pub(crate) use readlink_unchecked::*;
pub(crate) use remove_dir_all_impl::*;
pub(crate) use remove_open_dir_by_searching::*;
pub(crate) use rename_unchecked::*;
pub(crate) use rmdir_unchecked::*;
pub(crate) use stat_unchecked::*;
pub(crate) use symlink_unchecked::*;
#[allow(unused_imports)]
pub(crate) use times::{set_times_follow_unchecked, set_times_nofollow_unchecked};
pub(crate) use unlink_unchecked::*;

// On Linux, there is a limit of 40 symlink expansions.
// Source: https://man7.org/linux/man-pages/man7/path_resolution.7.html
pub(crate) const MAX_SYMLINK_EXPANSIONS: u8 = 40;

pub(super) use oflags::*;
