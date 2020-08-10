mod dir_entry_inner;
mod dir_options_ext;
mod dir_utils;
#[cfg(feature = "windows_file_type_ext")]
mod file_type_ext;
mod flags_impl;
mod get_path;
#[cfg(feature = "windows_file_type_ext")]
mod is_root_dir;
#[cfg(feature = "windows_by_handle")]
mod is_same_file;
mod link_unchecked;
#[cfg(feature = "windows_file_type_ext")]
mod metadata_ext;
mod mkdir_unchecked;
mod oflags;
mod open_options_ext;
mod open_unchecked;
mod read_dir_inner;
mod readlink_unchecked;
mod remove_dir_all_impl;
mod remove_open_dir_impl;
mod rename_unchecked;
mod rmdir_unchecked;
mod stat_unchecked;
mod symlink_unchecked;
mod unlink_unchecked;

pub(crate) mod errors;

#[rustfmt::skip]
pub(crate) use crate::fs::{
    canonicalize_manually_and_follow as canonicalize_impl,
    link_via_parent as link_impl,
    mkdir_via_parent as mkdir_impl,
    readlink_via_parent as readlink_impl,
    rename_via_parent as rename_impl,
    rmdir_via_parent as rmdir_impl,
    stat_via_parent as stat_impl,
    symlink_dir_via_parent as symlink_dir_impl,
    symlink_file_via_parent as symlink_file_impl,
    unlink_via_parent as unlink_impl,
};

pub(crate) use crate::fs::open_manually_wrapper as open_impl;
pub(crate) use dir_entry_inner::*;
pub(crate) use dir_options_ext::*;
pub(crate) use dir_utils::*;
#[cfg(feature = "windows_file_type_ext")]
pub(crate) use file_type_ext::*;
pub(crate) use flags_impl::*;
#[allow(unused_imports)]
pub(crate) use get_path::get_path as get_path_impl;
#[cfg(feature = "windows_file_type_ext")]
pub(crate) use is_root_dir::*;
#[cfg(feature = "windows_by_handle")]
#[allow(unused_imports)]
pub(crate) use is_same_file::*;
pub(crate) use link_unchecked::*;
#[cfg(feature = "windows_file_type_ext")]
pub(crate) use metadata_ext::*;
pub(crate) use mkdir_unchecked::*;
pub(crate) use open_options_ext::*;
pub(crate) use open_unchecked::*;
pub(crate) use read_dir_inner::*;
pub(crate) use readlink_unchecked::*;
pub(crate) use remove_dir_all_impl::*;
pub(crate) use remove_open_dir_impl::*;
pub(crate) use rename_unchecked::*;
pub(crate) use rmdir_unchecked::*;
pub(crate) use stat_unchecked::*;
pub(crate) use symlink_unchecked::*;
pub(crate) use unlink_unchecked::*;

// On Windows, there is a limit of 63 reparse points on any given path.
// https://docs.microsoft.com/en-us/windows/win32/fileio/reparse-points
pub(crate) const MAX_SYMLINK_EXPANSIONS: u8 = 63;

pub(super) use oflags::*;
