This crate provides a capability-based version of [`async-std`]. It provides all the
interfaces you are used to, but in a capability-based version.

This is a very simplistic port of [`cap-std`] to async-std. Key `fs` functions
including opening files still use synchronous API calls. Quite a few comments still
talk about `std` rather than `async-std`.

[`async-std`]: https://docs.rs/async_std/
[`cap-std`]: https://docs.rs/cap_std/

It is a work in progress and many things aren't implemented yet.

The two most interesting features are `fs::Dir` and `net::Catalog` (name TBD).
Dirs represent capabilities for accessing files beneath them, and "catalogs"
represent capabilities for creating network connections.

This library has two potential uses in the WASI ecosystem. First, an implementation
abstraction within wasi-common, abstracting over some [yanix]/[winx] differences. And
second, a user library, for writing applications that use std-like APIs but that
don't require a preopen-like layer.

Things to think about:
 - Should `try_clone` and other methods that consume resources require
   a capability?
 - Should we provide a capability-oriented [`std::process::Command`]?
 - Rust's `Path` has several ambient-authority methods: `metadata`,
   `read_link`, `read_dir`, `symlink_metadata`, `canonicalize`. Is it
   worth having our own version of `Path` just to exclude those? Such a
   thing could also exclude absolute paths.
 - utf8-cap-async-std (with [arf strings])?
 - Should we provide any of Rust's Unix-specific APIs on Windows, using
   winx and emulation?
 - Should we propose adding things to Rust's libstd which would help streamline this library?
    - A way to construct an arbitrary [`std::fs::FileType`] and [`std::fs::Metadata`]?
    - A way to read the options out of a [`std::fs::OpenOptions`] and [`std::fs::DirBuilder`]?

[arf strings]: https://github.com/bytecodealliance/arf-strings/
[`std::process::Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`std::fs::FileType`]: https://doc.rust-lang.org/std/fs/struct.FileType.html
[`std::fs::Metadata`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html
[`std::fs::DirBuilder`]: https://doc.rust-lang.org/std/fs/struct.DirBuilder.html
[`std::fs::OpenOptions`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
[yanix]: https://docs.rs/yanix
[winx]: https://docs.rs/winx
