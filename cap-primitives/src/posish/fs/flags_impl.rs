use super::accmode;
use posish::fs::{getfl, OFlags};
use std::{fs, io};

pub(crate) fn flags_impl(file: &fs::File) -> io::Result<(bool, bool)> {
    let mode = getfl(file)?;

    // Check for `O_PATH`.
    #[cfg(any(
        target_os = "android",
        target_os = "fuchsia",
        target_os = "linux",
        target_os = "emscripten"
    ))]
    if mode.contains(OFlags::PATH) {
        return Ok((false, false));
    }

    match mode & accmode() {
        OFlags::RDONLY => Ok((true, false)),
        OFlags::RDWR => Ok((true, true)),
        OFlags::WRONLY => Ok((false, true)),
        _ => unreachable!(),
    }
}
