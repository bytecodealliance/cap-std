mod common;
#[cfg(target_os = "linux")]
mod linux;

pub(crate) mod fs {
    pub(crate) use super::common::fs::*;

    #[cfg(target_os = "linux")]
    pub(crate) use super::linux::fs::*;

    #[cfg(not(target_os = "linux"))]
    pub(crate) use crate::fs::open_manually_wrapper as open_impl;

    pub(crate) use crate::fs::stat_via_parent as stat_impl;
    pub(crate) use crate::fs::mkdir_via_parent as mkdir_impl;
    pub(crate) use crate::fs::unlink_via_parent as unlink_impl;
}
