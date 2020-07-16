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
random paths and attempts random filesystem operations on them.

Caution is recommended when running this fuzzer, since it is a filesystem
fuzzer which in a critical path may result in data loss.

For more details on our fuzzer, see [fuzz/README.md].

[fuzz/README.md]: https://github.com/sunfishcode/cap-std/blob/main/fuzz/README.md

To run the `cap-primitives` fuzzer, run:

```
cargo +nightly fuzz run cap-primitives
```

