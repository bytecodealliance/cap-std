use rustix::fs::OFlags;
use std::{fs, io};

#[inline]
pub(crate) fn is_file_read_write_impl(file: &fs::File) -> io::Result<(bool, bool)> {
    let mode = rustix::fs::fcntl_getfl(file)?;

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

    // Use `RWMODE` rather than `ACCMODE` as `ACCMODE` may include `O_PATH`.
    // We handled `O_PATH` above.
    match mode & OFlags::RWMODE {
        OFlags::RDONLY => Ok((true, false)),
        OFlags::RDWR => Ok((true, true)),
        OFlags::WRONLY => Ok((false, true)),
        _ => unreachable!(),
    }
}
