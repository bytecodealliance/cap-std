use std::io;

#[cold]
pub(crate) fn readlink_not_symlink() -> io::Error {
    todo!("reading_not_symlink")
}

#[cold]
pub(crate) fn rename_path_in_use() -> io::Error {
    todo!("rename_path_in_use")
}
