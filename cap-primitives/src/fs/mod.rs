//! Filesystem utilities.

mod canonical_path;
mod canonicalize;
mod canonicalize_manually;
mod copy;
mod cow_component;
mod dir_builder;
mod dir_entry;
mod dir_options;
mod file_type;
mod flags;
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
#[cfg(not(windows))] // not needed on windows
mod open_entry_manually;
mod open_manually;
mod open_options;
mod open_parent;
mod open_unchecked_error;
mod permissions;
mod read_dir;
mod readlink;
mod readlink_one;
#[cfg(not(windows))] // doesn't work on windows; use a windows-specific impl
mod readlink_via_parent;
mod remove_dir_all;
mod remove_open_dir;
mod rename;
mod rename_via_parent;
mod rmdir;
mod rmdir_via_parent;
mod set_permissions;
#[cfg(not(target_os = "linux"))] // doesn't work reliably on linux
mod set_permissions_via_parent;
mod stat;
mod symlink;
mod symlink_via_parent;
mod unlink;
mod unlink_via_parent;

pub(crate) mod errors;

use canonical_path::CanonicalPath;
use cow_component::{to_borrowed_component, to_owned_component, CowComponent};
use maybe_owned_file::MaybeOwnedFile;
use open_parent::open_parent;
use readlink_one::readlink_one;

pub(crate) use canonicalize_manually::*;
#[cfg(not(feature = "no_racy_asserts"))]
pub(crate) use get_path::*;
pub(crate) use link_via_parent::*;
pub(crate) use mkdir_via_parent::*;
#[cfg(not(windows))] // not needed on windows
pub(crate) use open_entry_manually::*;
pub(crate) use open_manually::*;
pub(crate) use open_unchecked_error::*;
#[cfg(not(windows))] // doesn't work on windows; use a windows-specific impl
pub(crate) use readlink_via_parent::*;
pub(crate) use rename_via_parent::*;
pub(crate) use rmdir_via_parent::*;
#[cfg(not(target_os = "linux"))] // doesn't work reliably on linux
pub(crate) use set_permissions_via_parent::*;
pub(crate) use symlink_via_parent::*;
pub(crate) use unlink_via_parent::*;

#[cfg(windows)]
pub(crate) use super::winx::fs::*;
#[cfg(not(windows))]
pub(crate) use super::yanix::fs::*;

pub use canonicalize::*;
pub use copy::*;
pub use dir_builder::*;
pub use dir_entry::*;
pub use dir_options::*;
pub use file_type::*;
pub use flags::*;
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
pub use set_permissions::*;
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
