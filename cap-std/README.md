<div align="center">
  <h1><code>cap-std</code></h1>

  <p>
    <strong>Capability-oriented version of the Rust standard library</strong>
  </p>

  <p>
    <a href="https://github.com/bytecodealliance/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/cap-std"><img src="https://img.shields.io/crates/v/cap-std.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/cap-std"><img src="https://docs.rs/cap-std/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

This crate provides a capability-oriented version of [`std`]. See the
[toplevel README.md] for more information about capability-oriented security.

The filesystem module [`cap_std::fs`], the networking module [`cap_std::net`],
and the time module [`cap_std::time`] currently support Linux, macOS, FreeBSD,
and Windows. WASI support is in development, though not yet usable.

[`std`]: https://doc.rust-lang.org/std/
[toplevel README.md]: https://github.com/bytecodealliance/cap-std/blob/main/README.md
[`cap_std::fs`]: https://docs.rs/cap-std/latest/cap_std/fs/index.html
[`cap_std::net`]: https://docs.rs/cap-std/latest/cap_std/net/index.html
[`cap_std::time`]: https://docs.rs/cap-std/latest/cap_std/time/index.html
