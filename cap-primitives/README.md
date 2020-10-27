<div align="center">
  <h1><code>cap-primitives</code></h1>

  <p>
    <strong>Capability-oriented primitives</strong>
  </p>

  <p>
    <a href="https://github.com/bytecodealliance/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/cap-permissions"><img src="https://img.shields.io/crates/v/cap-permissions.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/cap-permissions"><img src="https://docs.rs/cap-permissions/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

The `cap-primitives` crate provides primitive sandboxing operations that
[`cap-std`] and [`cap-async-std`] are built on.

The filesystem module [`cap_primitives::fs`] and time module
[`cap_primitives::time`] currently support Linux, macOS, FreeBSD, and Windows.
WASI support is in development, though not yet usable.

The networking module, `net`, is not yet usable.

[`cap-std`]: https://github.com/bytecodealliance/cap-std/blob/main/cap-std/README.md
[`cap-async-std`]: https://github.com/bytecodealliance/cap-std/blob/main/cap-async-std/README.md
[`cap_primitives::fs`]: https://docs.rs/cap-primitives/current/cap_primitives/fs/index.html
[`cap_primitives::time`]: https://docs.rs/cap-primitives/current/cap_primitives/time/index.html
