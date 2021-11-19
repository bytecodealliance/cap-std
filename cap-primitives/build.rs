use std::env::var;
use std::io::Write;

fn main() {
    use_feature_or_nothing("windows_by_handle"); // https://github.com/rust-lang/rust/issues/63010
                                                 // https://doc.rust-lang.org/unstable-book/library-features/windows-file-type-ext.html
    use_feature_or_nothing("windows_file_type_ext");
    use_feature_or_nothing("io_error_more"); // https://github.com/rust-lang/rust/issues/86442

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
    let out_dir = var("OUT_DIR").unwrap();
    let rustc = var("RUSTC").unwrap();

    let mut child = std::process::Command::new(rustc)
        .arg("--crate-type=rlib") // Don't require `main`.
        .arg("--emit=metadata") // Do as little as possible but still parse.
        .arg("--out-dir")
        .arg(out_dir) // Put the output somewhere inconsequential.
        .arg("-") // Read from stdin.
        .stdin(std::process::Stdio::piped()) // Stdin is a pipe.
        .spawn()
        .unwrap();

    writeln!(child.stdin.take().unwrap(), "#![feature({})]", feature).unwrap();

    child.wait().unwrap().success()
}
