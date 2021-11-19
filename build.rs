use std::env::var;
use std::io::Write;

fn main() {
    use_feature_or_nothing("can_vector"); // https://github.com/rust-lang/rust/issues/69941
    use_feature_or_nothing("clamp"); // https://github.com/rust-lang/rust/issues/44095
    use_feature_or_nothing("extend_one"); // https://github.com/rust-lang/rust/issues/72631
    use_feature_or_nothing("io_error_more"); // https://github.com/rust-lang/rust/issues/86442
    use_feature_or_nothing("pattern"); // https://github.com/rust-lang/rust/issues/27721
    use_feature_or_nothing("seek_convenience"); // https://github.com/rust-lang/rust/issues/59359
    use_feature_or_nothing("shrink_to"); // https://github.com/rust-lang/rust/issues/56431
    use_feature_or_nothing("toowned_clone_into"); // https://github.com/rust-lang/rust/issues/41263
    use_feature_or_nothing("try_reserve"); // https://github.com/rust-lang/rust/issues/56431
    use_feature_or_nothing("unix_socket_peek"); // https://github.com/rust-lang/rust/issues/76923
    use_feature_or_nothing("windows_by_handle"); // https://github.com/rust-lang/rust/issues/63010
    use_feature_or_nothing("with_options"); // https://github.com/rust-lang/rust/issues/65439
    use_feature_or_nothing("write_all_vectored"); // https://github.com/rust-lang/rust/issues/70436
                                                  // https://doc.rust-lang.org/unstable-book/library-features/windows-file-type-ext.html
    use_feature_or_nothing("windows_file_type_ext");

    // Don't rerun this on changes other than build.rs, as we only depend on
    // the rustc version.
    println!("cargo:rerun-if-changed=build.rs");
}

fn use_feature_or_nothing(feature: &str) {
    if has_feature(feature) {
        use_feature(feature);
    }
}

fn use_feature(feature: &str) {
    println!("cargo:rustc-cfg={}", feature);
}

/// Test whether the rustc at `var("RUSTC")` supports the given feature.
fn has_feature(feature: &str) -> bool {
    let rustc = var("RUSTC").unwrap();

    #[cfg(not(windows))]
    let dev_null = "/dev/null";
    #[cfg(windows)]
    let dev_null = "NUL";

    let mut child = std::process::Command::new(rustc)
        .arg("--crate-type=rlib") // Don't require `main`.
        .arg("--emit=dep-info") // Do as little as possible.
        .arg("-o")
        .arg(dev_null) // Don't write an output file.
        .arg("-") // Read from stdin.
        .stdin(std::process::Stdio::piped()) // Stdin is a pipe.
        .spawn()
        .unwrap();

    writeln!(child.stdin.take().unwrap(), "#![feature({})]", feature).unwrap();

    child.wait().unwrap().success()
}
