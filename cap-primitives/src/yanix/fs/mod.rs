mod copy;
mod dir_entry_inner;
mod dir_options_ext;
mod dir_utils;
mod file_type_ext;
mod flags_impl;
mod is_root_dir;
#[cfg(not(feature = "no_racy_asserts"))]
mod is_same_file;
mod link_unchecked;
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
#[cfg(not(target_os = "linux"))] // doesn't work reliably on linux
mod set_permissions_unchecked;
mod stat_unchecked;
mod symlink_unchecked;
mod unlink_unchecked;

pub(crate) mod errors;

// On Linux, use optimized implementations of `open` and `stat` using `openat2`
// and `O_PATH` when available.
//
// FreeBSD has a similar mechanism in `O_BENEATH`, however it appears to have
// different behavior on absolute and `..` paths in ways that make it unsuitable
// for `cap-std`'s style of sandboxing. For more information, see the bug filed
// upstream: https://bugs.freebsd.org/bugzilla/show_bug.cgi?id=248335
#[cfg(target_os = "linux")]
pub(crate) use crate::yanix::linux::fs::*;
#[cfg(not(target_os = "linux"))]
#[rustfmt::skip]
pub(crate) use crate::fs::{
    open_entry_manually as open_entry_impl,
    open_manually_wrapper as open_impl,
    stat_via_parent as stat_impl,
    canonicalize_manually_and_follow as canonicalize_impl,
    set_permissions_via_parent as set_permissions_impl,
};

#[rustfmt::skip]
pub(crate) use crate::fs::{
    link_via_parent as link_impl,
    mkdir_via_parent as mkdir_impl,
    readlink_via_parent as readlink_impl,
    rename_via_parent as rename_impl,
    rmdir_via_parent as rmdir_impl,
    symlink_via_parent as symlink_impl,
    unlink_via_parent as unlink_impl,
    remove_open_dir_by_searching as remove_open_dir_impl,
};

pub(crate) use copy::*;
pub(crate) use dir_entry_inner::*;
pub(crate) use dir_options_ext::*;
pub(crate) use dir_utils::*;
pub(crate) use file_type_ext::*;
pub(crate) use flags_impl::*;
pub(crate) use is_root_dir::*;
#[cfg(not(feature = "no_racy_asserts"))]
pub(crate) use is_same_file::*;
pub(crate) use link_unchecked::*;
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
#[cfg(not(target_os = "linux"))] // doesn't work reliably on linux
pub(crate) use set_permissions_unchecked::*;
pub(crate) use stat_unchecked::*;
pub(crate) use symlink_unchecked::*;
pub(crate) use unlink_unchecked::*;

// On Linux, there is a limit of 40 symlink expansions.
// Source: https://man7.org/linux/man-pages/man7/path_resolution.7.html
pub(crate) const MAX_SYMLINK_EXPANSIONS: u8 = 40;

pub(super) use oflags::*;
