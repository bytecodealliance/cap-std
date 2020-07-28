//! Capability-oriented temporary directories.

#![deny(missing_docs)]
#![doc(html_logo_url = "https://github.com/sunfishcode/cap-std/tree/main/media/cap-std.svg")]

use cap_std::fs::Dir;
use std::{io, ops::Deref};

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
    inner: tempfile::TempDir,
    dir: Dir,
}

impl TempDir {
    /// Attempts to make a temporary directory inside of `env::temp_dir()`.
    ///
    /// This corresponds to [`tempfile::TempDir::new`].
    ///
    /// [`tempfile::TempDir::new`]: https://docs.rs/tempfile/latest/tempfile/struct.TempDir.html#method.new
    pub fn new() -> io::Result<Self> {
        let inner = tempfile::TempDir::new()?;
        let dir = unsafe { Dir::open_ambient_dir(inner.path())? };
        Ok(Self { inner, dir })
    }

    // TODO: `new_in`, but take a `Dir` instead of a `Path`?

    /// Closes and removes the temporary directory, returing a `Result`.
    ///
    /// This corresponds to [`tempfile::TempDir::close`].
    ///
    /// [`tempfile::TempDir::close`]: https://docs.rs/tempfile/latest/tempfile/struct.TempDir.html#method.close
    pub fn close(self) -> io::Result<()> {
        self.inner.close()
    }
}

impl Deref for TempDir {
    type Target = Dir;

    fn deref(&self) -> &Self::Target {
        &self.dir
    }
}

/// Create a new temporary directory.
///
/// This corresponds to [`tempfile::tempdir`].
///
/// [`tempfile::tempdir`]: https://docs.rs/tempfile/3.1.0/tempfile/fn.tempdir.html
pub fn tempdir() -> io::Result<TempDir> {
    TempDir::new()
}
