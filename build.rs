fn main() {
    if let rustc_version::Channel::Nightly = rustc_version::version_meta()
        .expect("query rustc release channel")
        .channel
    {
        for feature in &[
            "can_vector",                // https://github.com/rust-lang/rust/issues/69941
            "clamp",                     // https://github.com/rust-lang/rust/issues/44095
            "extend_one",                // https://github.com/rust-lang/rust/issues/72631
            "open_options_ext_as_flags", // https://github.com/rust-lang/rust/issues/76801
            "pattern",                   // https://github.com/rust-lang/rust/issues/27721
            "read_initializer",          // https://github.com/rust-lang/rust/issues/42788
            "seek_convenience",          // https://github.com/rust-lang/rust/issues/59359
            "shrink_to",                 // https://github.com/rust-lang/rust/issues/56431
            "toowned_clone_into",        // https://github.com/rust-lang/rust/issues/41263
            "try_reserve",               // https://github.com/rust-lang/rust/issues/56431
            "unix_socket_peek",          // https://github.com/rust-lang/rust/issues/76923
            "windows_by_handle",         // https://github.com/rust-lang/rust/issues/63010
            "with_options",              // https://github.com/rust-lang/rust/issues/65439
            "write_all_vectored",        // https://github.com/rust-lang/rust/issues/70436
            // https://doc.rust-lang.org/unstable-book/library-features/windows-file-type-ext.html
            "windows_file_type_ext",
        ] {
            println!("cargo:rustc-cfg={}", feature);
        }
    }

    // Don't rerun this on changes other than build.rs, as we only depend on
    // the rustc version.
    println!("cargo:rerun-if-changed=build.rs");
}
