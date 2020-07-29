<div align="center">
  <h1><code>cap-async-std</code></h1>

  <p>
    <strong>Capability-based version of `async-std`</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://cirrus-ci.com/github/sunfishcode/cap-std"><img src="https://api.cirrus-ci.com/github/sunfishcode/cap-std.svg" alt="Cirrus CI Status" /></a>
  </p>
</div>

This crate provides a capability-based version of [`async-std`]. It provides all the
interfaces you are used to, but in a capability-based version.

This is a very simplistic port of [`cap-std`] to `async-std`. Key `fs` functions
including opening files still use synchronous API calls.

See the [`cap-std` README.md] for more information about capability-based security.

[`async-std`]: https://crates.io/crates/async-std
[`cap-std`]: https://crates.io/crates/cap-std
[`cap-std` README.md]: https://github.com/sunfishcode/cap-std/blob/main/README.md

At the moment, `cap-async-std` is a very rudimentary translation of `cap-std` to
`async-std`. Like `cap-std`, it uses [`cap-primitives`] to perform the underlying
operations. It hasn't yet been optimized to make effective use of `async`.

The filesystem module, `fs`, is known to compile on Linux, macOS, and FreeBSD, and
probably can be easily ported to other modern Unix-family platforms. Ports to
Windows and WASI platforms are in development, though not yet usable.

The networking module, `net`, is not yet usable.

[`cap-primitives`]: https://crates.io/crates/cap-primitives
