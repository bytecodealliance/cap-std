use super::get_path::get_path;
use std::{fs, io};

pub(crate) fn remove_open_dir_impl(dir: fs::File) -> io::Result<()> {
    let path = get_path(&dir)?;
    fs::remove_dir(path)
}
