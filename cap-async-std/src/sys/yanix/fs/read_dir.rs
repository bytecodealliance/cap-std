use crate::fs::{Dir, DirEntry};
use async_std::io;

pub(crate) struct ReadDir<'dir> {
    dir: &'dir Dir,
    // ... add traversal state here
}

impl<'dir> Iterator for ReadDir<'dir> {
    type Item = io::Result<DirEntry<'dir>>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!("ReadDir::next()")
    }
}
