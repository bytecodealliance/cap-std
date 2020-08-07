use super::get_path::get_path;
use crate::fs::ReadDir;
use std::{fs, io};

pub(crate) fn is_root_dir(dir: &fs::File, _parent_iter: &ReadDir) -> io::Result<bool> {
    Ok(get_path(dir)?.file_name().is_none())
}
