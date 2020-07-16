<div align="center">
  <h1><code>cap-async-std</code></h1>

  <p>
    <strong>Capability-based version of `async-std`</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/cap-async-std/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/cap-async-std/workflows/CI/badge.svg" alt="build status" /></a>
  </p>
</div>

This crate provides a capability-based version of [`async-std`]. It provides all the
interfaces you are used to, but in a capability-based version.

This is a very simplistic port of [`cap-std`] to `async-std`. Key `fs` functions
including opening files still use synchronous API calls.

See the [`cap-std` README.md] for more information about capability-based security.

[`async-std`]: https://docs.rs/async_std/
[`cap-std`]: https://docs.rs/cap_std/
[`cap-std` README.md]: https://github.com/sunfishcode/cap-std/blob/main/README.md

At the moment, `cap-async-std` is a very rudimentary translation of `cap-std` to
`async-std`. It doesn't yet have tests or fuzzing, and it hasn't yet been
optimized to make effective use of `async`. It should not be considered mature
or "battle-tested". Use at your own risk.

The filesystem module, `fs`, is known to compile on Linux, macOS, and FreeBSD,
and probably can be easily ported to other modern Unix-family platforms. Ports
to Windows and WASI platforms are in development, though not yet usable.

The networking module, `net`, is not yet usable.
