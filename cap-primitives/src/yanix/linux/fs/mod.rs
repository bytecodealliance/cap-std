mod canonicalize_impl;
mod ensure_cloexec;
mod file_metadata;
#[cfg(any(test, not(feature = "no_racy_asserts")))]
mod file_path;
mod open_entry_impl;
mod open_impl;
mod procfs;
mod set_permissions_impl;
mod stat_impl;

pub(crate) use canonicalize_impl::*;
pub(crate) use ensure_cloexec::*;
#[cfg(any(test, not(feature = "no_racy_asserts")))]
pub(crate) use file_path::*;
pub(crate) use open_entry_impl::*;
pub(crate) use open_impl::*;
pub(crate) use set_permissions_impl::*;
pub(crate) use stat_impl::*;

use file_metadata::file_metadata;

// In theory we could optimize `link` using `openat2` with `O_PATH` and
// `linkat` with `AT_EMPTY_PATH`, however that requires `CAP_DAC_READ_SEARCH`,
// so it isn't very widely applicable.
