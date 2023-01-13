use std::env::var;
use std::io::Write;

fn main() {
    use_feature_or_nothing("can_vector"); // https://github.com/rust-lang/rust/issues/69941
    use_feature_or_nothing("clamp"); // https://github.com/rust-lang/rust/issues/44095
    use_feature_or_nothing("extend_one"); // https://github.com/rust-lang/rust/issues/72631
    use_feature_or_nothing("io_error_more"); // https://github.com/rust-lang/rust/issues/86442
    use_feature_or_nothing("io_error_uncategorized");
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
    can_compile(&format!(
        "#![allow(stable_features)]\n#![feature({})]",
        feature
    ))
}

/// Test whether the rustc at `var("RUSTC")` can compile the given code.
fn can_compile<T: AsRef<str>>(test: T) -> bool {
    use std::process::Stdio;

    let out_dir = var("OUT_DIR").unwrap();
    let rustc = var("RUSTC").unwrap();
    let target = var("TARGET").unwrap();

    let mut cmd = if let Ok(wrapper) = var("CARGO_RUSTC_WRAPPER") {
        let mut cmd = std::process::Command::new(wrapper);
        // The wrapper's first argument is supposed to be the path to rustc.
        cmd.arg(rustc);
        cmd
    } else {
        std::process::Command::new(rustc)
    };

    cmd.arg("--crate-type=rlib") // Don't require `main`.
        .arg("--emit=metadata") // Do as little as possible but still parse.
        .arg("--target")
        .arg(target)
        .arg("--out-dir")
        .arg(out_dir); // Put the output somewhere inconsequential.

    // If Cargo wants to set RUSTFLAGS, use that.
    if let Ok(rustflags) = var("CARGO_ENCODED_RUSTFLAGS") {
        if !rustflags.is_empty() {
            for arg in rustflags.split('\x1f') {
                cmd.arg(arg);
            }
        }
    }

    let mut child = cmd
        .arg("-") // Read from stdin.
        .stdin(Stdio::piped()) // Stdin is a pipe.
        .stderr(Stdio::null()) // Errors from feature detection aren't interesting and can be confusing.
        .spawn()
        .unwrap();

    writeln!(child.stdin.take().unwrap(), "{}", test.as_ref()).unwrap();

    child.wait().unwrap().success()
}
