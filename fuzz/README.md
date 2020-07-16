# Fuzz targets for `cap-std`

This crate defines various `libFuzzer` targets for `cap-std` which can be run
via `cargo-fuzz` plugin.

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
cargo +nightly fuzz run cap-primitives
```

