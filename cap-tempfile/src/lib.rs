//! Capability-oriented temporary directories.

#![deny(missing_docs)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/sunfishcode/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sunfishcode/cap-std/main/media/cap-std.ico"
)]

use cap_std::fs::Dir;
use std::{env, fmt, fs, io, mem, ops::Deref};
use uuid::Uuid;

/// A directory in a filesystem that is automatically deleted when it goes out of scope.
///
/// This corresponds to [`tempfile::TempDir`].
///
/// Unlike `tempfile::TempDir`, this API has no `TempDir::path`, `TempDir::into_path`,
/// or `impl AsRef<Path>`, because absolute paths don't interoperate well with the capability
/// model.
///
/// [`tempfile::TempDir`]: https://docs.rs/tempfile/latest/tempfile/struct.TempDir.html
pub struct TempDir {
    dir: Option<Dir>,
}

impl TempDir {
    /// Attempts to make a temporary directory inside of `env::temp_dir()`.
    ///
    /// This corresponds to [`tempfile::TempDir::new`].
    ///
    /// [`tempfile::TempDir::new`]: https://docs.rs/tempfile/latest/tempfile/struct.TempDir.html#method.new
    ///
    /// # Safety
    ///
    /// This function is unsafe because it makes use of ambient authority to
    /// access temporary directories, which doesn't uphold the invariant of
    /// the rest of the API. It is otherwise safe to use.
    pub unsafe fn new() -> io::Result<Self> {
        let system_tmp = env::temp_dir();
        for _ in 0..Self::num_iterations() {
            let name = system_tmp.join(&Self::new_name());
            match fs::create_dir(&name) {
                Ok(()) => {
                    let dir = match Dir::open_ambient_dir(&name) {
                        Ok(dir) => dir,
                        Err(e) => {
                            fs::remove_dir(name).ok();
                            return Err(e);
                        }
                    };
                    return Ok(Self { dir: Some(dir) });
                }
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e),
            }
        }
        Err(Self::already_exists())
    }

    /// Create a new temporary directory.
    ///
    /// This corresponds to [`tempfile::TempDir::new_in`].
    ///
    /// [`tempfile::TempDir::new_in`]: https://docs.rs/tempfile/latest/tempfile/fn.tempdir_in.html
    pub fn new_in(dir: &Dir) -> io::Result<Self> {
        for _ in 0..Self::num_iterations() {
            let name = &Self::new_name();
            match dir.create_dir(&name) {
                Ok(()) => {
                    let dir = match dir.open_dir(&name) {
                        Ok(dir) => dir,
                        Err(e) => {
                            dir.remove_dir(name).ok();
                            return Err(e);
                        }
                    };
                    return Ok(Self { dir: Some(dir) });
                }
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e),
            }
        }
        Err(Self::already_exists())
    }

    /// Closes and removes the temporary directory, returning a `Result`.
    ///
    /// This corresponds to [`tempfile::TempDir::close`].
    ///
    /// [`tempfile::TempDir::close`]: https://docs.rs/tempfile/latest/tempfile/struct.TempDir.html#method.close
    pub fn close(mut self) -> io::Result<()> {
        mem::take(&mut self.dir).unwrap().remove_open_dir_all()
    }

    fn new_name() -> String {
        Uuid::new_v4().to_string()
    }

    const fn num_iterations() -> i32 {
        i32::MAX
    }

    fn already_exists() -> io::Error {
        io::Error::new(
            io::ErrorKind::AlreadyExists,
            "too many temporary files exist",
        )
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
/// [`tempfile::tempdir`]: https://docs.rs/tempfile/3.1.0/tempfile/fn.tempdir.html
///
/// # Safety
///
/// This function is unsafe because it makes use of ambient authority to
/// access temporary directories, which doesn't uphold the invariant of
/// the rest of the API. It is otherwise safe to use.
pub unsafe fn tempdir() -> io::Result<TempDir> {
    TempDir::new()
}

/// Create a new temporary directory.
///
/// This corresponds to [`tempfile::tempdir_in`].
///
/// [`tempfile::tempdir`]: https://docs.rs/tempfile/3.1.0/tempfile/fn.tempdir_in.html
pub fn tempdir_in(dir: &Dir) -> io::Result<TempDir> {
    TempDir::new_in(dir)
}

#[test]
fn drop_tempdir() {
    let t = unsafe { tempdir().unwrap() };
    drop(t)
}

#[test]
fn close_tempdir() {
    let t = unsafe { tempdir().unwrap() };
    t.close().unwrap();
}

#[test]
fn drop_tempdir_in() {
    let dir = unsafe { Dir::open_ambient_dir(env::temp_dir()).unwrap() };
    let t = tempdir_in(&dir).unwrap();
    drop(t);
}

#[test]
fn close_tempdir_in() {
    let dir = unsafe { Dir::open_ambient_dir(env::temp_dir()).unwrap() };
    let t = tempdir_in(&dir).unwrap();
    t.close().unwrap();
}

#[test]
fn close_outer() {
    let t = unsafe { tempdir().unwrap() };
    let _s = tempdir_in(&t).unwrap();
    #[cfg(windows)]
    assert_eq!(
        t.close().unwrap_err().raw_os_error(),
        Some(winapi::shared::winerror::ERROR_SHARING_VIOLATION as i32)
    );
    #[cfg(not(windows))]
    t.close().unwrap();
}

#[test]
fn close_inner() {
    let t = unsafe { tempdir().unwrap() };
    let s = tempdir_in(&t).unwrap();
    s.close().unwrap();
}
