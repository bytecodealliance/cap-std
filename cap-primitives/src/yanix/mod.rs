mod common;
#[cfg(target_os = "linux")]
mod linux;

pub(crate) mod fs {
    use cfg_if::cfg_if;

    pub(crate) use super::common::fs::*;

    cfg_if! {
        if #[cfg(target_os = "linux")] {
            pub(crate) use super::linux::fs::*;
        } else {
            pub(crate) use crate::fs::open_manually_wrapper as open_impl;
        }
    }

    pub(crate) use crate::fs::{
        canonicalize_manually as canonicalize_impl, link_via_parent as link_impl,
        mkdir_via_parent as mkdir_impl, stat_via_parent as stat_impl,
        symlink_via_parent as symlink_impl, unlink_via_parent as unlink_impl,
        rename_via_parent as rename_impl
    };
}
