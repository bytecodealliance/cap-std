fn main() {
    if let rustc_version::Channel::Nightly = rustc_version::version_meta()
        .expect("query rustc release channel")
        .channel
    {
        for feature in &[
            "can_vector",         // https://github.com/rust-lang/rust/issues/69941
            "read_initializer",   // https://github.com/rust-lang/rust/issues/42788
            "seek_convenience",   // https://github.com/rust-lang/rust/issues/59359
            "with_options",       // https://github.com/rust-lang/rust/issues/65439
            "write_all_vectored", // https://github.com/rust-lang/rust/issues/70436
            "windows_by_handle",  // https://github.com/rust-lang/rust/issues/63010
            "windows_file_type_ext",
        ] {
            println!("cargo:rustc-cfg={}", feature);
        }
    }
}
