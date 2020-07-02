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

    pub(crate) use crate::fs::stat_via_parent as stat_impl;
    pub(crate) use crate::fs::mkdir_via_parent as mkdir_impl;
    pub(crate) use crate::fs::unlink_via_parent as unlink_impl;
    pub(crate) use crate::fs::link_via_parent as link_impl;
}
