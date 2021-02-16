use super::get_path::concatenate_or_return_absolute;
use crate::fs::{open, FollowSymlinks, OpenOptions, Permissions};
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `set_permissions`, but which does not
/// perform sandboxing.
pub(crate) fn set_permissions_unchecked(
    start: &fs::File,
    path: &Path,
    perm: Permissions,
) -> io::Result<()> {
    // According to [Rust's documentation], `fs::set_permissions` uses
    // `SetFileAttributes`, and according to [Windows' documentation]
    // `SetFileAttributes` does not follow symbolic links.
    //
    // [Windows' documentation]: https://docs.microsoft.com/en-us/windows/win32/fileio/symbolic-link-effects-on-file-systems-functions#setfileattributes
    // [Rust's documentation]: https://doc.rust-lang.org/std/fs/fn.set_permissions.html#platform-specific-behavior
    let (out_path, enforce_dir) = concatenate_or_return_absolute(start, path)?;
    if enforce_dir {
        let mut opts = OpenOptions::new();
        opts.follow(FollowSymlinks::No);
        let opened = open(start, path, &opts)?;
        let std_perm = perm.into_std(&opened)?;
        opened.set_permissions(std_perm)
    } else {
        fs::set_permissions(out_path, perm.into_std(start)?)
    }
}
