# Fuzz targets for `cap-std`

This crate defines various `libFuzzer` targets for `cap-std` which can be run
via `cargo-fuzz` plugin.

Currently, there is only a simple fuzzer for the `cap-primitives` crate which
constructs random paths and attempts random filesystem operations on them. If
`cap-primitives`' sandbox is working as intended, these operations either stay
within a temporary directory or fail. Many of the operations in `cap-primitives`
have backup checks in `cfg(racy_asserts)` builds, to diagnose sandbox escapes.

Caution is recommended when running this fuzzer, since it is a filesystem
fuzzer, and if it should find a way to escape the sandbox and avoid the backup
checks, it could cause data loss.

## Example: fuzzing `cap-primitives`

1. Install `cargo-fuzz` plugin:

```
cargo install cargo-fuzz
```

2. Install `nightly` toolchain:

```
rustup toolchain add nightly
```

3. Fuzz away:

```
env 'RUSTFLAGS=--cfg racy_asserts' cargo +nightly fuzz run cap-primitives
```
