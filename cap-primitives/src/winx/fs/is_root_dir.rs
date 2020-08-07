use crate::fs::ReadDir;
use std::{fs, io};

pub(crate) fn is_root_dir(_dir: &fs::File, _parent_iter: &ReadDir) -> io::Result<bool> {
    todo!("is_root_dir")
}
