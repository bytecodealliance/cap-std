<div align="center">
  <h1><code>cap-primitives</code></h1>

  <p>
    <strong>Capability-oriented primitives</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/cap-primitives/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/cap-primitives/workflows/CI/badge.svg" alt="build status" /></a>
  </p>
</div>

`cap-primitives` crate provides primitive sandboxing operations that [`cap-std`]
and [`cap-async-std`] are built on.

[`cap-std`]: https://crates.io/crates/cap-std
[`cap-async-std`]: https://crates.io/crates/cap-async-std

The filesystem module, `fs`, is known to work on Linux, macOS, and FreeBSD, and
probably can be easily ported to other modern Unix-family platforms. Ports to
Windows and WASI platforms are in development, though not yet usable.

The networking module, `net`, is not yet usable.
