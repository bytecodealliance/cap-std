use crate::fs::manually;

mod copy;
mod dir_entry_inner;
mod dir_options_ext;
mod dir_utils;
mod file_type_ext;
mod flags_impl;
mod get_path;
#[cfg(feature = "windows_by_handle")]
mod is_same_file;
mod link_unchecked;
mod metadata_ext;
mod mkdir_unchecked;
mod oflags;
mod open_options_ext;
mod open_unchecked;
mod read_dir_inner;
mod readlink_impl;
mod readlink_unchecked;
mod remove_dir_all_impl;
mod remove_open_dir_impl;
mod rename_unchecked;
mod rmdir_unchecked;
mod set_permissions_unchecked;
mod stat_unchecked;
mod symlink_unchecked;
mod unlink_unchecked;

pub(crate) mod errors;

#[rustfmt::skip]
pub(crate) use crate::fs::{
    manually::canonicalize as canonicalize_impl,
    via_parent::link as link_impl,
    via_parent::mkdir as mkdir_impl,
    via_parent::rename as rename_impl,
    via_parent::rmdir as rmdir_impl,
    via_parent::set_permissions as set_permissions_impl,
    manually::stat as stat_impl,
    via_parent::symlink_dir as symlink_dir_impl,
    via_parent::symlink_file as symlink_file_impl,
    via_parent::unlink as unlink_impl,
};

pub(crate) use copy::*;
pub(crate) use dir_entry_inner::*;
pub(crate) use dir_options_ext::*;
pub(crate) use dir_utils::*;
pub(crate) use file_type_ext::*;
pub(crate) use flags_impl::*;
#[cfg(feature = "windows_by_handle")]
#[allow(unused_imports)]
pub(crate) use is_same_file::*;
pub(crate) use link_unchecked::*;
pub(crate) use manually::open as open_impl;
pub(crate) use metadata_ext::*;
pub(crate) use mkdir_unchecked::*;
pub(crate) use open_options_ext::*;
pub(crate) use open_unchecked::*;
pub(crate) use read_dir_inner::*;
pub(crate) use readlink_impl::*;
pub(crate) use readlink_unchecked::*;
pub(crate) use remove_dir_all_impl::*;
pub(crate) use remove_open_dir_impl::*;
pub(crate) use rename_unchecked::*;
pub(crate) use rmdir_unchecked::*;
pub(crate) use set_permissions_unchecked::*;
pub(crate) use stat_unchecked::*;
pub(crate) use symlink_unchecked::*;
pub(crate) use unlink_unchecked::*;

// On Windows, there is a limit of 63 reparse points on any given path.
// https://docs.microsoft.com/en-us/windows/win32/fileio/reparse-points
pub(crate) const MAX_SYMLINK_EXPANSIONS: u8 = 63;

#[cfg(any(test, not(feature = "no_racy_asserts")))]
pub(crate) fn file_path(file: &std::fs::File) -> Option<std::path::PathBuf> {
    get_path::get_path(file).ok()
}

pub(super) use oflags::*;
