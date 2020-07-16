mod dir_entry_inner;
mod dir_options_ext;
mod dir_utils;
mod file_type_ext;
mod flags;
mod is_same_file;
mod link_unchecked;
mod metadata_ext;
mod mkdir_unchecked;
mod open_options_ext;
mod open_unchecked;
mod permissions_ext;
mod read_dir_inner;
mod readlink_one;
mod readlink_unchecked;
mod remove_dir_all_impl;
mod rename_unchecked;
mod rmdir_unchecked;
mod stat_unchecked;
mod symlink_unchecked;
mod unlink_unchecked;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub(crate) use crate::yanix::linux::fs::*;
    } else {
        pub(crate) use crate::fs::open_manually_wrapper as open_impl;
    }
}

pub(crate) use crate::fs::{
    canonicalize_manually_and_follow as canonicalize_impl, link_via_parent as link_impl,
    mkdir_via_parent as mkdir_impl, readlink_via_parent as readlink_impl,
    rename_via_parent as rename_impl, rmdir_via_parent as rmdir_impl, stat_via_parent as stat_impl,
    symlink_via_parent as symlink_impl, unlink_via_parent as unlink_impl,
};

pub(crate) mod errors;

pub(crate) use dir_entry_inner::*;
pub(crate) use dir_options_ext::*;
pub(crate) use dir_utils::*;
pub(crate) use file_type_ext::*;
pub(crate) use flags::*;
pub(crate) use is_same_file::*;
pub(crate) use link_unchecked::*;
pub(crate) use metadata_ext::*;
pub(crate) use mkdir_unchecked::*;
pub(crate) use open_options_ext::*;
pub(crate) use open_unchecked::*;
pub(crate) use permissions_ext::*;
pub(crate) use read_dir_inner::*;
pub(crate) use readlink_one::*;
pub(crate) use readlink_unchecked::*;
pub(crate) use remove_dir_all_impl::*;
pub(crate) use rename_unchecked::*;
pub(crate) use rmdir_unchecked::*;
pub(crate) use stat_unchecked::*;
pub(crate) use symlink_unchecked::*;
pub(crate) use unlink_unchecked::*;
