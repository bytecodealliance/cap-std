<div align="center">
  <h1><code>cap-std</code></h1>

  <p>
    <strong>Capability-oriented version of the Rust standard library</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://cirrus-ci.com/github/sunfishcode/cap-std"><img src="https://api.cirrus-ci.com/github/sunfishcode/cap-std.svg" alt="Cirrus CI Status" /></a>
    <a href="https://docs.rs/cap-std"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>
</div>

This crate provides a capability-oriented version of [`std`]. See the
[toplevel README.md] for more information about capability-oriented security.

The filesystem module, [`cap_std::fs`], is known to compile on Linux, macOS, and
FreeBSD, and probably can be easily ported to other modern Unix-family platforms.
Windows support is in development; basic functionality works, but not all features
are implemented yet. WASI support is in development, though not yet usable.

The networking module, `net`, is not yet usable.

[`std`]: https://doc.rust-lang.org/std/
[toplevel README.md]: https://github.com/sunfishcode/cap-std/blob/main/README.md
[`cap_std::fs`]: https://docs.rs/cap-std/latest/cap_std/fs/index.html
