# Contributing to `cap-std`

`cap-std` follows common [PR]-oriented development, and uses common Rust tooling,
including [`rustfmt`].

[PR]: https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/about-pull-requests
[rustfmt]: https://github.com/rust-lang/rustfmt#quick-start

## Tests

`cargo test`.

## Fuzzing

There is a simple fuzzer for the `cap-primitives` crate which constructs
random paths and attempts random filesystem operations on them.

Caution is recommended when running this fuzzer, since it is a filesystem
fuzzer which in a critical path may result in data loss.

For more details on our fuzzer, see [fuzz/README.md].

[fuzz/README.md]: https://github.com/sunfishcode/cap-std/blob/main/fuzz/README.md

## Benchmarking

There are several micro-benchmarks for the `cap-std` crate which stress-test
specific API features. As micro-benchmarks, they aren't representative of
real-world use, but they are useful for development of `cap-std`.

To run the `cap-std` benchmarks, run:

```
cargo +nightly bench
```
