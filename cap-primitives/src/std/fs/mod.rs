//! Filesystem utilities.

mod file_type;
mod follow_symlinks;
#[cfg(debug_assertions)]
mod get_path;
mod maybe_owned_file;
mod metadata;
mod open;
mod open_manually;
mod open_options;
mod open_parent;
mod permissions;
mod stat;
mod stat_via_parent;

#[cfg(debug_assertions)]
pub(crate) use get_path::*;
pub(crate) use maybe_owned_file::*;
pub(crate) use open_manually::*;
pub(crate) use open_parent::*;
pub(crate) use stat_via_parent::*;

pub use file_type::*;
pub use follow_symlinks::*;
pub use metadata::*;
pub use open::*;
pub use open_options::*;
pub use permissions::*;
pub use stat::*;
