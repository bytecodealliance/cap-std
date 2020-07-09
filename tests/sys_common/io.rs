// This file is derived from src/libstd/sys_common/io.rs in the Rust repository at revision
// 50fc24d8a172a853b5dfe40702d6550e3b8562ba.
//
// Note that we use `std`, because this is infrastructure for running tests.

use std::{
    env, fs, io,
    ops::Deref,
    path::{Path, PathBuf},
};
use uuid::Uuid;

pub struct TempDir(cap_std::fs::Dir, PathBuf);

impl TempDir {
    #[cfg(target_family = "unix")]
    fn from_std(path: &Path) -> io::Result<Self> {
        Ok(Self(
            cap_std::fs::Dir::from_std_file(fs::File::open(&path)?),
            path.to_owned(),
        ))
    }

    #[cfg(windows)]
    fn from_std(path: &Path) -> io::Result<Self> {
        use std::os::windows::fs::OpenOptionsExt;
        use winapi::um::winbase::FILE_FLAG_BACKUP_SEMANTICS;
        Ok(Self(
            cap_std::fs::Dir::from_std_file(
                fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .attributes(FILE_FLAG_BACKUP_SEMANTICS)
                    .open(&path)?,
            ),
            path.to_owned(),
        ))
    }
}

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
    let ret = p.join(&Uuid::new_v4().to_string());
    fs::create_dir(&ret).unwrap();
    TempDir::from_std(&ret).expect("expected to be able to open temporary directory")
}
