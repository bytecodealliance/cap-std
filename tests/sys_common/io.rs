use cap_tempfile::tempdir;

pub use cap_tempfile::TempDir;

pub fn tmpdir() -> TempDir {
    tempdir().expect("expected to be able to create a temporary directory")
}
