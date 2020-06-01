This crate provides a capability-based version of [`std`]. It provides all the
interfaces you are used to, but in a capability-based version.

[`std`]: https://doc.rust-lang.org/std/index.html

It is a work in progress and many things aren't implemented yet.

The two most interesting classes are `fs::Dir` and `net::Catalog`. Directories
represent capabilities for accessing files beneath them, and "catalogs" (name
TBD) represent capabilities for creating network connections.

There are four potentially interesting ways that we might implement this API:
 - yanix's `openat` etc. on Unix-like hosts
 - winx'`s `openat` etc. emulation on Windows
 - translating directory handles back into paths and calling libstd `open`
 - Calling WASI directly -- skip the CString overhead

Currently, only the yanix one is started.
