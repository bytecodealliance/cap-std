#![allow(dead_code)] // TODO: When more things are implemented, remove these.

use crate::fs::{Dir, DirEntry};
use async_std::io;

/// Iterator over the entries in a directory.
///
/// This corresponds to [`std::fs::ReadDir`].
///
/// Unlike `async_std::fs::ReadDir`, this API has a lifetime parameter.
///
/// TODO: The lifetime parameter is here because `ReadDir` needs to return
/// `DirEntry`s which have paths
///
/// Note that there is no `from_std` method, as `async_std::fs::ReadDir` doesn't
/// provide a way to construct a `ReadDir` without opening directories by
/// ambient paths.
///
/// [`std::fs::ReadDir`]: https://doc.rust-lang.org/std/fs/struct.ReadDir.html
pub struct ReadDir<'dir> {
    dir: &'dir Dir,
}

impl<'dir> Iterator for ReadDir<'dir> {
    type Item = io::Result<DirEntry<'dir>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        todo!("ReadDir::next()")
    }
}

// TODO: impl Debug for ReadDir? But don't expose ReadDir's path...
