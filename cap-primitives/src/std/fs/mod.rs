//! Filesystem utilities.

mod canonicalize;
mod canonicalize_manually;
mod file_type;
mod follow_symlinks;
#[cfg(debug_assertions)]
mod get_path;
mod link;
mod link_via_parent;
mod maybe_owned_file;
mod metadata;
mod mkdir;
mod mkdir_via_parent;
mod open;
mod open_manually;
mod open_options;
mod open_parent;
mod permissions;
mod stat;
mod stat_via_parent;
mod unlink;
mod unlink_via_parent;

pub(crate) use canonicalize_manually::*;
#[cfg(debug_assertions)]
pub(crate) use get_path::*;
pub(crate) use link_via_parent::*;
pub(crate) use maybe_owned_file::*;
pub(crate) use mkdir_via_parent::*;
pub(crate) use open_manually::*;
pub(crate) use open_parent::*;
pub(crate) use stat_via_parent::*;
pub(crate) use unlink_via_parent::*;

pub use canonicalize::*;
pub use file_type::*;
pub use follow_symlinks::*;
pub use link::*;
pub use metadata::*;
pub use mkdir::*;
pub use open::*;
pub use open_options::*;
pub use permissions::*;
pub use stat::*;
pub use unlink::*;
