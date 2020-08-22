//! In many operations, the last component of a path is special. For example,
//! in `mkdir`, the last component names the path to be created, while the
//! rest of the components just name the place to create it in.

mod link;
mod mkdir;
mod open_parent;
#[cfg(not(windows))] // doesn't work on windows; use a windows-specific impl
mod readlink;
mod rename;
mod rmdir;
#[cfg(not(target_os = "linux"))] // doesn't work reliably on linux
mod set_permissions;
mod symlink;
mod unlink;

use open_parent::open_parent;

pub(crate) use link::link;
pub(crate) use mkdir::mkdir;
#[cfg(not(windows))] // doesn't work on windows; use a windows-specific impl
pub(crate) use readlink::readlink;
pub(crate) use rename::rename;
pub(crate) use rmdir::rmdir;
#[cfg(not(target_os = "linux"))] // doesn't work reliably on linux
pub(crate) use set_permissions::set_permissions;
#[cfg(not(windows))]
pub(crate) use symlink::symlink;
#[cfg(windows)]
pub(crate) use symlink::{symlink_dir, symlink_file};
pub(crate) use unlink::unlink;
