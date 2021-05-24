<div align="center">
  <h1><code>cap-async-std</code></h1>

  <p>
    <strong>Capability-based version of `async-std`</strong>
  </p>

  <p>
    <a href="https://github.com/bytecodealliance/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/cap-async-std"><img src="https://img.shields.io/crates/v/cap-async-std.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/cap-async-std"><img src="https://docs.rs/cap-async-std/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

This crate provides a capability-based version of [`async-std`]. See the
[toplevel README.md] for more information about capability-based security.

At the moment, `cap-async-std` is a very rudimentary translation of [`cap-std`] to
`async-std`. It hasn't yet been optimized to make effective use of `async`.

[`async-std`]: https://crates.io/crates/async-std
[`cap-std`]: https://github.com/bytecodealliance/cap-std/blob/main/cap-std/README.md
[toplevel README.md]: https://github.com/bytecodealliance/cap-std/blob/main/README.md
