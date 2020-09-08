<div align="center">
  <h1><code>cap-std</code></h1>

  <p>
    <strong>Capability-oriented version of the Rust standard library</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://cirrus-ci.com/github/sunfishcode/cap-std"><img src="https://api.cirrus-ci.com/github/sunfishcode/cap-std.svg" alt="Cirrus CI Status" /></a>
    <a href="https://travis-ci.com/sunfishcode/cap-std"><img src="https://travis-ci.com/sunfishcode/cap-std.svg?branch=main" alt="Travis CI Status" /></a>
    <a href="https://crates.io/crates/cap-std"><img src="https://img.shields.io/crates/v/cap-std.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/cap-std"><img src="https://docs.rs/cap-std/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

This crate provides a capability-oriented version of [`std`]. See the
[toplevel README.md] for more information about capability-oriented security.

The filesystem module [`cap_std::fs`] and the time module [`cap_std::time`]
currently support Linux, macOS, FreeBSD, and Windows. WASI support is in
development, though not yet usable.

The networking module, `net`, is not yet usable.

[`std`]: https://doc.rust-lang.org/std/
[toplevel README.md]: https://github.com/sunfishcode/cap-std/blob/main/README.md
[`cap_std::fs`]: https://docs.rs/cap-std/latest/cap_std/fs/index.html
[`cap_std::time`]: https://docs.rs/cap-std/latest/cap_std/time/index.html
