// This file is derived from src/libstd/sys_common/io.rs in the Rust repository at revision
// 50fc24d8a172a853b5dfe40702d6550e3b8562ba.
//
// Note that we use `std`, because this is infrastructure for running tests.

use rand::RngCore;
use std::{env, fs, ops::Deref, path::PathBuf};

pub struct TempDir(cap_std::fs::Dir, PathBuf);

impl Drop for TempDir {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.1).unwrap();
    }
}

impl Deref for TempDir {
    type Target = cap_std::fs::Dir;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn tmpdir() -> TempDir {
    let p = env::temp_dir();
    let mut r = rand::thread_rng();
    let ret = p.join(&format!("rust-{}", r.next_u32()));
    fs::create_dir(&ret).unwrap();
    TempDir(
        cap_std::fs::Dir::from_std_file(
            fs::File::open(&ret).expect("expected to be able to open temporary directory"),
        ),
        ret,
    )
}
