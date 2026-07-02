use rustix::fs::{renameat_with, RenameFlags};
use std::path::Path;
use std::{fs, io};

pub(crate) fn rename_excl_unchecked(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    renameat_with(
        old_start,
        old_path,
        new_start,
        new_path,
        RenameFlags::NOREPLACE,
    )
    .map_err(Into::into)
}
