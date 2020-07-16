# Contributing to `cap-std`

`cap-std` follows common [PR]-oriented development, and uses common Rust tooling,
including [`rustfmt`].

[PR]: https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/about-pull-requests
[rustfmt]: https://github.com/rust-lang/rustfmt#quick-start

## Tests

Of course `cargo test` works, though there are additional options
that are useful to add:

 - `--workspace` - This additionally runs tests in the `cap-primitives` and
   `cap-async-std` packages.

 - `--all-features` - This additionally runs tests in the `fs_utf8` modules.

## Fuzzing

There is a simple fuzzer for the `cap-primitives` crate which constructs
random paths and attempts random filesystem operations on them. If
`cap-primitives`' sandbox is working as intended, these operations either
stay within a temporary directory or fail. Many of the operations in
`cap-primitives` have backup checks in `debug_assertions` builds, to
diagnose sandbox escapes.

Caution is recommended when running this fuzzer, since it is a filesystem
fuzzer, and if it should find a way to escape the sandbox and avoid the
backup checks, it could cause data loss.

To run the `cap-primitives` fuzzer, run:

```
cargo +nightly fuzz run cap-primitives
```
