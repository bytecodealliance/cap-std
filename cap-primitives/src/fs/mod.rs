//! Filesystem utilities.

mod canonicalize;
mod canonicalize_manually;
mod dir_builder;
mod dir_entry;
mod dir_options;
mod file_type;
mod follow_symlinks;
#[cfg(not(feature = "no_racy_asserts"))]
mod get_path;
mod link;
mod link_via_parent;
mod maybe_owned_file;
mod metadata;
mod mkdir;
mod mkdir_via_parent;
mod open;
mod open_dir;
mod open_entry_manually;
mod open_manually;
mod open_options;
mod open_parent;
mod permissions;
mod read_dir;
mod readlink;
mod readlink_one;
mod readlink_via_parent;
mod remove_dir_all;
mod remove_open_dir;
mod remove_open_dir_by_searching;
mod rename;
mod rename_via_parent;
mod rmdir;
mod rmdir_via_parent;
mod stat;
mod stat_via_parent;
mod symlink;
mod symlink_via_parent;
mod unlink;
mod unlink_via_parent;

pub(crate) mod errors;

pub(crate) use canonicalize_manually::*;
#[cfg(not(feature = "no_racy_asserts"))]
pub(crate) use get_path::*;
pub(crate) use link_via_parent::*;
pub(crate) use maybe_owned_file::*;
pub(crate) use mkdir_via_parent::*;
pub(crate) use open_entry_manually::*;
pub(crate) use open_manually::*;
pub(crate) use open_parent::*;
pub(crate) use readlink_one::*;
pub(crate) use readlink_via_parent::*;
pub(crate) use remove_open_dir_by_searching::*;
pub(crate) use rename_via_parent::*;
pub(crate) use rmdir_via_parent::*;
pub(crate) use stat_via_parent::*;
pub(crate) use symlink_via_parent::*;
pub(crate) use unlink_via_parent::*;

cfg_if::cfg_if! {
    if #[cfg(any(unix, target_os = "fuchsia"))] {
        pub(crate) use super::yanix::fs::*;
    } else if #[cfg(windows)] {
        pub(crate) use super::winx::fs::*;
    } else if #[cfg(not(target_os = "wasi"))] {
        compile_error!("cap-std doesn't compile for this platform yet");
    }
}

pub use canonicalize::*;
pub use dir_builder::*;
pub use dir_entry::*;
pub use dir_options::*;
pub use file_type::*;
pub use follow_symlinks::*;
pub use link::*;
pub use metadata::*;
pub use mkdir::*;
pub use open::*;
pub use open_dir::*;
pub use open_options::*;
pub use permissions::*;
pub use read_dir::*;
pub use readlink::*;
pub use remove_dir_all::*;
pub use remove_open_dir::*;
pub use rename::*;
pub use rmdir::*;
pub use stat::*;
pub use symlink::*;
pub use unlink::*;

#[cfg(not(feature = "no_racy_asserts"))]
fn map_result<T: Clone>(result: &std::io::Result<T>) -> Result<T, (std::io::ErrorKind, String)> {
    match result {
        Ok(t) => Ok(t.clone()),
        Err(e) => Err((e.kind(), e.to_string())),
    }
}
