//! A capability-based API modeled after `std`.
//!
//! This corresponds to [`std`].
//!
//! Capability-based APIs represent access to external resources as
//! objects which can be passed around between different parts of a
//! program.
//!
//! Two notable features are the [`Dir`] and [`Catalog`] types:
//!  - `Dir` represents an open directory in a filesystem. Instead of
//!    opening files by absolute paths or paths relative to the current
//!    working directory, files are opened via paths relative to a
//!    `Dir`. The concepts of a process-wide "current working directory"
//!    and a single global filesystem namespace are de-emphasized.
//!  - `Catalog` represents a set of network addresses. Instead of
//!    allowing applications to request access to any address and then
//!    applying process-wide filtering rules, filtering rules are
//!    built into catalogs which may be passed through the program.
//!
//! On WASI, use of this library closely reflects the underlying system
//! API, so it avoids compatibility layers.
//!
//! [`std`]: https://doc.rust-lang.org/std/index.html
//! [`Dir`]: fs/struct.Dir.html
//! [`Catalog`]: net/struct.Catalog.html

#![allow(dead_code, unused_variables)] // TODO: When more things are implemented, remove these.
#![deny(missing_docs)]

mod sys;

pub mod fs;
#[cfg(feature = "fs_utf8")]
pub mod fs_utf8;
pub mod net;
pub mod os;

#[cfg(test)]
mod test {
    use super::fs;
    use std::ops::Deref;
    use std::os::unix::io::{FromRawFd, IntoRawFd};
    use std::path::PathBuf;
    use std::{env, io};
    use uuid::Uuid;

    pub struct TempDir {
        abs_path: PathBuf,
        dir: fs::Dir,
    }

    impl TempDir {
        pub fn new() -> io::Result<Self> {
            // TODO support Windows and WASI
            let mut abs_path = env::temp_dir();
            abs_path.push(Uuid::new_v4().to_string());
            std::fs::create_dir(&abs_path)?;
            let dir = std::fs::File::open(&abs_path)?;
            let dir = unsafe { fs::Dir::from_raw_fd(dir.into_raw_fd()) };

            Ok(Self { abs_path, dir })
        }
    }

    impl Deref for TempDir {
        type Target = fs::Dir;

        fn deref(&self) -> &Self::Target {
            &self.dir
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.abs_path).expect("could remove the temp directory")
        }
    }
}
