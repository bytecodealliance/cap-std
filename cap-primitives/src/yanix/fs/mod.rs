mod file_type_ext;
mod flags;
mod is_same_file;
mod link_unchecked;
mod metadata_ext;
mod mkdir_unchecked;
mod open_options_ext;
mod open_unchecked;
mod permissions_ext;
mod readlink_unchecked;
mod rename_unchecked;
mod resolve_symlink_at;
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
    canonicalize_manually as canonicalize_impl, link_via_parent as link_impl,
    mkdir_via_parent as mkdir_impl, readlink_via_parent as readlink_impl,
    rename_via_parent as rename_impl, stat_via_parent as stat_impl,
    symlink_via_parent as symlink_impl, unlink_via_parent as unlink_impl,
};

pub(crate) mod errors;

pub(crate) use file_type_ext::*;
pub(crate) use flags::*;
pub(crate) use is_same_file::*;
pub(crate) use link_unchecked::*;
pub(crate) use metadata_ext::*;
pub(crate) use mkdir_unchecked::*;
pub(crate) use open_options_ext::*;
pub(crate) use open_unchecked::*;
pub(crate) use permissions_ext::*;
pub(crate) use readlink_unchecked::*;
pub(crate) use rename_unchecked::*;
pub(crate) use resolve_symlink_at::*;
pub(crate) use stat_unchecked::*;
pub(crate) use symlink_unchecked::*;
pub(crate) use unlink_unchecked::*;
