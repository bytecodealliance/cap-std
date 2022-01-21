//! Capability-based temporary directories.

#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.ico"
)]

use cap_std::fs::Dir;
use std::ops::Deref;
use std::{env, fmt, fs, io, mem};
#[cfg(not(target_os = "emscripten"))]
use uuid::Uuid;

#[cfg(feature = "fs_utf8")]
pub mod utf8;

#[doc(hidden)]
pub use cap_std::ambient_authority_known_at_compile_time;
pub use cap_std::{ambient_authority, AmbientAuthority};

/// A directory in a filesystem that is automatically deleted when it goes out
/// of scope.
///
/// This corresponds to [`tempfile::TempDir`].
///
/// Unlike `tempfile::TempDir`, this API has no `TempDir::path`,
/// `TempDir::into_path`, or `impl AsRef<Path>`, because absolute paths don't
/// interoperate well with the capability model.
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
    /// # Ambient Authority
    ///
    /// This function makes use of ambient authority to access temporary
    /// directories.
    pub fn new(ambient_authority: AmbientAuthority) -> io::Result<Self> {
        let system_tmp = env::temp_dir();
        for _ in 0..Self::num_iterations() {
            let name = system_tmp.join(&Self::new_name());
            match fs::create_dir(&name) {
                Ok(()) => {
                    let dir = match Dir::open_ambient_dir(&name, ambient_authority) {
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
        #[cfg(not(target_os = "emscripten"))]
        {
            Uuid::new_v4().to_string()
        }

        // Uuid doesn't support Emscripten yet, but Emscripten isn't multi-user
        // or multi-process yet, so we can do something simple.
        #[cfg(target_os = "emscripten")]
        {
            use rand::RngCore;
            let mut r = rand::thread_rng();
            format!("cap-primitives.{}", r.next_u32())
        }
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

    let dir = Dir::open_ambient_dir(env::temp_dir(), ambient_authority()).unwrap();
    let t = tempdir_in(&dir).unwrap();
    drop(t);
}

#[test]
fn close_tempdir_in() {
    use crate::ambient_authority;

    let dir = Dir::open_ambient_dir(env::temp_dir(), ambient_authority()).unwrap();
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
        t.close().unwrap_err().raw_os_error() as _,
        Some(winapi::shared::winerror::ERROR_SHARING_VIOLATION)
            | Some(winapi::shared::winerror::ERROR_DIR_NOT_EMPTY)
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
