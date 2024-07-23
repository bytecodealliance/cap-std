//! Capability-based temporary directories, with UTF-8 paths.
//!
//! TODO: This whole scheme is still under development.

#[cfg(test)]
use camino::Utf8PathBuf;
use cap_std::fs_utf8::Dir;
#[cfg(test)]
use std::env;
use std::ops::Deref;
use std::{fmt, io, mem};

#[doc(hidden)]
pub use cap_std::ambient_authority_known_at_compile_time;
pub use cap_std::{ambient_authority, AmbientAuthority};

/// A directory in a filesystem that is automatically deleted when it goes out
/// of scope.
///
/// This corresponds to [`tempfile::TempDir`].
///
/// Unlike `tempfile::TempDir`, this API has no `TempDir::path`,
/// `TempDir::into_path`, or `impl AsRef<Utf8Path>`, because absolute paths
/// don't interoperate well with the capability model.
///
/// [`tempfile::TempDir`]: https://docs.rs/tempfile/latest/tempfile/struct.TempDir.html
pub struct TempDir {
    dir: Option<Dir>,
}

impl TempDir {
    // Consume a base/non-UTF8 tempdir instance and return a UTF8 version
    fn from_cap_std(mut td: super::TempDir) -> Self {
        // Take ownership of the underlying Dir instance
        let dir = td.dir.take().map(Dir::from_cap_std);
        Self { dir }
    }

    /// Attempts to make a temporary directory inside of `env::temp_dir()`.
    ///
    /// This corresponds to [`tempfile::TempDir::new`].
    ///
    /// [`tempfile::TempDir::new`]: https://docs.rs/tempfile/latest/tempfile/struct.TempDir.html#method.new
    ///
    /// # Ambient Authority
    ///
    /// This function makes use of ambient authority to access temporary
    /// directories.
    pub fn new(ambient_authority: AmbientAuthority) -> io::Result<Self> {
        // Because a Dir instance doesn't have a name accessible to the calling
        // code, we just delegate to the non-UTF8 base version. In some presumably
        // very unusual situations the tempdir may have a non-UTF8 name,
        // but that's fine.
        super::TempDir::new(ambient_authority).map(Self::from_cap_std)
    }

    /// Create a new temporary directory.
    ///
    /// This corresponds to [`tempfile::TempDir::new_in`].
    ///
    /// [`tempfile::TempDir::new_in`]: https://docs.rs/tempfile/latest/tempfile/fn.tempdir_in.html
    pub fn new_in(dir: &Dir) -> io::Result<Self> {
        super::TempDir::new_in(dir.as_cap_std()).map(Self::from_cap_std)
    }

    /// Closes and removes the temporary directory, returning a `Result`.
    ///
    /// This corresponds to [`tempfile::TempDir::close`].
    ///
    /// [`tempfile::TempDir::close`]: https://docs.rs/tempfile/latest/tempfile/struct.TempDir.html#method.close
    pub fn close(mut self) -> io::Result<()> {
        mem::take(&mut self.dir).unwrap().remove_open_dir_all()
    }
}

impl Deref for TempDir {
    type Target = Dir;

    fn deref(&self) -> &Self::Target {
        self.dir.as_ref().unwrap()
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if let Some(dir) = mem::take(&mut self.dir) {
            dir.remove_open_dir_all().ok();
        }
    }
}

impl fmt::Debug for TempDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.dir.fmt(f)
    }
}

/// Create a new temporary directory.
///
/// This corresponds to [`tempfile::tempdir`].
///
/// [`tempfile::tempdir`]: https://docs.rs/tempfile/latest/tempfile/fn.tempdir.html
///
/// # Ambient Authority
///
/// This function makes use of ambient authority to access temporary
/// directories.
pub fn tempdir(ambient_authority: AmbientAuthority) -> io::Result<TempDir> {
    TempDir::new(ambient_authority)
}

/// Create a new temporary directory.
///
/// This corresponds to [`tempfile::tempdir_in`].
///
/// [`tempfile::tempdir_in`]: https://docs.rs/tempfile/latest/tempfile/fn.tempdir_in.html
pub fn tempdir_in(dir: &Dir) -> io::Result<TempDir> {
    TempDir::new_in(dir)
}

#[test]
fn drop_tempdir() {
    use crate::ambient_authority;

    let t = tempdir(ambient_authority()).unwrap();
    drop(t)
}

#[test]
fn close_tempdir() {
    use crate::ambient_authority;

    let t = tempdir(ambient_authority()).unwrap();
    t.close().unwrap();
}

#[test]
fn drop_tempdir_in() {
    use crate::ambient_authority;

    let temp_dir: Utf8PathBuf = env::temp_dir().try_into().unwrap();
    let dir = Dir::open_ambient_dir(temp_dir, ambient_authority()).unwrap();
    let t = tempdir_in(&dir).unwrap();
    drop(t);
}

#[test]
fn close_tempdir_in() {
    use crate::ambient_authority;

    let temp_dir: Utf8PathBuf = env::temp_dir().try_into().unwrap();
    let dir = Dir::open_ambient_dir(temp_dir, ambient_authority()).unwrap();
    let t = tempdir_in(&dir).unwrap();
    t.close().unwrap();
}

#[test]
fn close_outer() {
    use crate::ambient_authority;

    let t = tempdir(ambient_authority()).unwrap();
    let _s = tempdir_in(&t).unwrap();
    #[cfg(windows)]
    assert!(matches!(
        t.close().unwrap_err().raw_os_error().map(|err| err as _),
        Some(windows_sys::Win32::Foundation::ERROR_SHARING_VIOLATION)
            | Some(windows_sys::Win32::Foundation::ERROR_DIR_NOT_EMPTY)
    ));
    #[cfg(not(windows))]
    t.close().unwrap();
}

#[test]
fn close_inner() {
    use crate::ambient_authority;

    let t = tempdir(ambient_authority()).unwrap();
    let s = tempdir_in(&t).unwrap();
    s.close().unwrap();
}
