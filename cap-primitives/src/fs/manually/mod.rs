//! Functions that perform path lookup manually, one component
//! at a time, with manual symlink resolution.

mod canonical_path;
mod canonicalize;
mod cow_component;
#[cfg(any(test, not(feature = "no_racy_asserts")))]
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "ios", windows)))]
mod file_path;
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
#[cfg(any(test, not(feature = "no_racy_asserts")))]
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "ios", windows)))]
pub(crate) use file_path::file_path;
pub(crate) use open::{open, stat};
#[cfg(not(windows))]
pub(crate) use open_entry::open_entry;
