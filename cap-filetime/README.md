<div align="center">
  <h1><code>cap-filetime</code></h1>

  <p>
    <strong>Capability-oriented filesystem timestamps</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://cirrus-ci.com/github/sunfishcode/cap-std"><img src="https://api.cirrus-ci.com/github/sunfishcode/cap-std.svg" alt="Cirrus CI Status" /></a>
    <a href="https://docs.rs/cap-filetime"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>
</div>

The `cap-filetime` crate provides utilities for working with file timestamps
similar to the [`filetime`] crate, but which works with [`Dir`]s instead of
`Path`s.

[`filetime`]: https://crates.io/crates/filetime
[`Dir`]: https://docs.rs/cap-std/latest/cap_std/fs/struct.Dir.html
