<div align="center">
  <h1><code>cap-tempdir</code></h1>

  <p>
    <strong>Capability-based standard directories for config, cache and other data</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/cap-std/workflows/CI/badge.svg" alt="build status" /></a>
  </p>
</div>

`cap-directories` crate provides utilities for accessing standard directories
via the [`directories`] crate, but which provide [`Dir`]s instead of `Path`s.

[`directories`]: https://crates.io/crates/directories
[`Dir`]: https://docs.rs/cap-std/latest/cap_std/fs/struct.Dir.html
