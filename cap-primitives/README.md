<div align="center">
  <h1><code>cap-primitives</code></h1>

  <p>
    <strong>Capability-oriented primitives</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://cirrus-ci.com/github/sunfishcode/cap-std"><img src="https://api.cirrus-ci.com/github/sunfishcode/cap-std.svg" alt="Cirrus CI Status" /></a>
    <a href="https://docs.rs/cap-primitives"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>
</div>

The `cap-primitives` crate provides primitive sandboxing operations that
[`cap-std`] and [`cap-async-std`] are built on.

The filesystem module, [`cap_std::fs`], currently supports Linux, macOS,
FreeBSD, and Windows. WASI support is in development, though not yet usable.

The networking module, `net`, is not yet usable.

[`cap-std`]: https://github.com/sunfishcode/cap-std/blob/main/cap-std/README.md
[`cap-async-std`]: https://github.com/sunfishcode/cap-std/blob/main/cap-async-std/README.md
