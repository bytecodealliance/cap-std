mod ensure_cloexec;
mod open_impl;
mod stat_impl;
mod open_entry_impl;

pub(crate) use ensure_cloexec::*;
pub(crate) use open_impl::*;
pub(crate) use open_entry_impl::*;
pub(crate) use stat_impl::*;

// In theory we could optimize `link` using `openat2` with `O_PATH` and
// `linkat` with `AT_EMPTY_PATH`, however that requires `CAP_DAC_READ_SEARCH`,
// so it isn't very widely applicable.
