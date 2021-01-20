//! Filesystem utilities.

#[cfg(racy_asserts)]
#[macro_use]
pub(crate) mod assert_same_file;

mod canonicalize;
mod copy;
mod create_dir;
mod dir_builder;
mod dir_entry;
mod dir_options;
#[cfg(not(any(target_os = "android", target_os = "linux", windows)))]
mod file_path_by_searching;
mod file_type;
mod follow_symlinks;
mod hard_link;
mod is_read_write;
mod maybe_owned_file;
mod metadata;
mod open;
mod open_dir;
mod open_options;
mod open_unchecked_error;
mod permissions;
mod read_dir;
mod read_link;
mod remove_dir;
mod remove_dir_all;
mod remove_file;
mod remove_open_dir;
mod rename;
mod reopen;
mod set_permissions;
mod set_times;
mod stat;
mod symlink;
mod system_time_spec;

pub(crate) mod errors;
pub(crate) mod manually;
pub(crate) mod via_parent;

use maybe_owned_file::MaybeOwnedFile;

#[cfg(not(any(target_os = "android", target_os = "linux", windows)))]
pub(crate) use file_path_by_searching::file_path_by_searching;
pub(crate) use open_unchecked_error::*;

#[cfg(not(windows))]
pub(crate) use super::posish::fs::*;
#[cfg(windows)]
pub(crate) use super::windows::fs::*;

pub use canonicalize::*;
pub use copy::*;
pub use create_dir::*;
pub use dir_builder::*;
pub use dir_entry::*;
pub use dir_options::*;
pub use file_type::*;
pub use follow_symlinks::*;
pub use hard_link::*;
pub use is_read_write::is_read_write;
pub use metadata::*;
pub use open::*;
pub use open_dir::*;
pub use open_options::*;
pub use permissions::*;
pub use read_dir::*;
pub use read_link::*;
pub use remove_dir::*;
pub use remove_dir_all::*;
pub use remove_file::*;
pub use remove_open_dir::*;
pub use rename::*;
pub use reopen::reopen;
pub use set_permissions::*;
pub use set_times::*;
pub use stat::*;
pub use symlink::*;
pub use system_time_spec::*;

#[cfg(racy_asserts)]
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
