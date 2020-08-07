<div align="center">
  <h1><code>cap-async-std</code></h1>

  <p>
    <strong>Capability-oriented version of `async-std`</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://cirrus-ci.com/github/sunfishcode/cap-std"><img src="https://api.cirrus-ci.com/github/sunfishcode/cap-std.svg" alt="Cirrus CI Status" /></a>
    <a href="https://docs.rs/cap-async-std"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>
</div>

This crate provides a capability-oriented version of [`async-std`]. See the
[toplevel README.md] for more information about capability-oriented security.

At the moment, `cap-async-std` is a very rudimentary translation of [`cap-std`] to
`async-std`. It hasn't yet been optimized to make effective use of `async`.

[`async-std`]: https://crates.io/crates/async-std
[`cap-std`]: https://github.com/sunfishcode/cap-std/blob/main/cap-std/README.md
[toplevel README.md]: https://github.com/sunfishcode/cap-std/blob/main/README.md
