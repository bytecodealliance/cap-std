// This file is derived from src/libstd/sys_common/io.rs in the Rust repository
// at revision 7e11379f3b4c376fbb9a6c4d44f3286ccc28d149.
//
// Note that we use plain `std` here, because this is infrastructure for running
// tests.

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
        use winx::file::Flags;
        Ok(Self(
            cap_std::fs::Dir::from_std_file(
                fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .attributes(Flags::FILE_FLAG_BACKUP_SEMANTICS.bits())
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
