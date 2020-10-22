<div align="center">
  <h1><code>cap-tempfile</code></h1>

  <p>
    <strong>Capability-oriented temporary directories</strong>
  </p>

  <p>
    <a href="https://github.com/bytecodealliance/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://cirrus-ci.com/github/bytecodealliance/cap-std"><img src="https://api.cirrus-ci.com/github/bytecodealliance/cap-std.svg" alt="Cirrus CI Status" /></a>
    <a href="https://travis-ci.com/bytecodealliance/cap-std"><img src="https://travis-ci.com/bytecodealliance/cap-std.svg?branch=main" alt="Travis CI Status" /></a>
    <a href="https://crates.io/crates/cap-tempfile"><img src="https://img.shields.io/crates/v/cap-tempfile.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/cap-tempfile"><img src="https://docs.rs/cap-tempfile/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

The `cap-tempfile` crate provides utilities for creating temporary directories
via the [`tempfile`] crate, but which provide [`Dir`]s instead of `Path`s.

[`tempfile`]: https://crates.io/crates/tempfile
[`Dir`]: https://docs.rs/cap-std/latest/cap_std/fs/struct.Dir.html
