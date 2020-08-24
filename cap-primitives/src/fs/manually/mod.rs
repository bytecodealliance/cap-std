//! Functions that perform path lookup manually, one component
//! at a time, with manual symlink resolution.

mod canonical_path;
mod canonicalize;
mod cow_component;
mod open;
#[cfg(not(windows))]
mod open_entry;
mod readlink_one;

use canonical_path::CanonicalPath;
use cow_component::CowComponent;
use open::internal_open;
use readlink_one::readlink_one;

#[cfg(not(feature = "no_racy_asserts"))]
pub(super) use canonicalize::canonicalize_with;

pub(crate) use canonicalize::canonicalize;
pub(crate) use open::{open, stat};
#[cfg(not(windows))]
pub(crate) use open_entry::open_entry;
