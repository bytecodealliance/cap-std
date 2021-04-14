use cap_tempfile::tempdir;

pub use cap_tempfile::TempDir;

#[allow(unused)]
pub fn tmpdir() -> TempDir {
    use cap_tempfile::ambient_authority;

    // It's ok to call `ambient_authority()` here, rather than take an
    // `AmbientAuthority` argument, because this function is only used
    // by tests.
    tempdir(ambient_authority()).expect("expected to be able to create a temporary directory")
}
