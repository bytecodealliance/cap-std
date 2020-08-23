//! Filesystem utilities.

mod canonicalize;
mod copy;
mod dir_builder;
mod dir_entry;
mod dir_options;
mod file_type;
mod flags;
mod follow_symlinks;
mod link;
mod maybe_owned_file;
mod metadata;
mod mkdir;
mod open;
mod open_dir;
mod open_options;
mod open_unchecked_error;
mod permissions;
mod read_dir;
mod readlink;
mod remove_dir_all;
mod remove_open_dir;
mod rename;
mod rmdir;
mod set_permissions;
mod stat;
mod symlink;
mod unlink;

pub(crate) mod errors;
pub(crate) mod manually;
pub(crate) mod via_parent;

use maybe_owned_file::MaybeOwnedFile;

pub(crate) use open_unchecked_error::*;

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

/// Test that `file_path` works on a few miscelleanous directory paths.
#[test]
fn dir_paths() {
    for path in &[std::env::current_dir().unwrap(), std::env::temp_dir()] {
        let dir = unsafe { open_ambient_dir(&path).unwrap() };
        assert_eq!(
            file_path(&dir)
                .as_ref()
                .map(std::fs::canonicalize)
                .map(Result::unwrap),
            Some(std::fs::canonicalize(path).unwrap())
        );
    }
}
