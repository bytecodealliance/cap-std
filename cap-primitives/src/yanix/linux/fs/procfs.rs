use super::file_metadata;
use crate::fs::{open_unchecked, readlink_unchecked, FollowSymlinks, OpenOptions};
use std::{
    ffi::CString,
    fs, io,
    mem::MaybeUninit,
    os::unix::{
        ffi::OsStrExt,
        fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
        io::AsRawFd,
    },
    path::{Path, PathBuf},
};

/// Linux's procfs always uses inode 1 for its root directory.
const PROC_ROOT_INO: u64 = 1;

/// The filesystem magic number for procfs.
///
/// This is defined in the `libc` crate for linux-gnu but not for
/// linux-musl, so we define it ourselves.
#[cfg(not(target_env = "musl"))]
const PROC_SUPER_MAGIC: libc::c_long = 0x0000_9fa0;
#[cfg(target_env = "musl")]
const PROC_SUPER_MAGIC: libc::c_ulong = 0x0000_9fa0;

lazy_static! {
    static ref PROC_SELF_FD: io::Result<fs::File> = init_proc_self_fd();
}

fn init_proc_self_fd() -> io::Result<fs::File> {
    // Open "/proc".
    // TODO: Here and below, add O_NOCTTY once yanix has it.
    let proc = fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_PATH | libc::O_DIRECTORY | libc::O_NOFOLLOW)
        .open("/proc")?;

    // Check the filesystem magic.
    confirm_procfs(&proc)?;

    let proc_metadata = file_metadata(&proc)?;

    if !proc_metadata.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "/proc isn't a directory",
        ));
    }

    // Check the root inode number.
    if proc_metadata.ino() != PROC_ROOT_INO {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "unexpected root inode in /proc",
        ));
    }

    // Check that root owns "/proc".
    if proc_metadata.uid() != 0 || proc_metadata.gid() != 0 || proc_metadata.mode() != 0o40555 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "/proc isn't owned by root",
        ));
    }

    // Check that the "/proc" directory isn't empty.
    if proc_metadata.nlink() <= 2 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "/proc appears to not have subdirectories",
        ));
    }

    // Open "/proc/self/fd". Use `read(true) even though we don't need `read`
    // permissions, because Rust's libstd requires an access mode, and Linux
    // ignores `O_RDONLY` with `O_PATH`.
    let proc_self_fd = open_unchecked(
        &proc,
        Path::new("self/fd"),
        OpenOptions::new()
            .read(true)
            .follow(FollowSymlinks::No)
            .custom_flags(libc::O_PATH | libc::O_DIRECTORY),
    )?;

    // Double-check that "/proc/self/fd" is still in procfs.
    confirm_procfs(&proc_self_fd)?;

    // Check that /proc is sane.
    let proc_self_fd_metadata = file_metadata(&proc_self_fd)?;

    if proc_self_fd_metadata.ino() == PROC_ROOT_INO {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "unexpected non-root inode in /proc/self/fd",
        ));
    }

    // Triple-check that we're still in procfs.
    if proc_self_fd_metadata.dev() != proc_metadata.dev() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "/proc/self/fd is on a different filesystem from /proc",
        ));
    }

    // Check that our process owns the "/proc/self/fd" directory.
    if proc_self_fd_metadata.uid() != unsafe { libc::getuid() }
        || proc_self_fd_metadata.gid() != unsafe { libc::getgid() }
        || proc_self_fd_metadata.mode() != 0o40500
    {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "/proc/self/fd isn't owned by the process' owner",
        ));
    }

    // Check that the "/proc/self/fd" directory doesn't have any extraneous
    // links into it (which would include unexpected subdirectories).
    if proc_self_fd_metadata.nlink() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "/proc/self/fd has an unexpected number of links",
        ));
    }

    Ok(proc_self_fd)
}

fn proc_self_fd() -> io::Result<&'static fs::File> {
    PROC_SELF_FD
        .as_ref()
        .map_err(|e| io::Error::new(e.kind(), e.to_string()))
}

fn confirm_procfs(file: &fs::File) -> io::Result<()> {
    let mut statfs = MaybeUninit::<libc::statfs>::uninit();
    cvt_i32(unsafe { libc::fstatfs(file.as_raw_fd(), statfs.as_mut_ptr()) })?;

    let f_type = unsafe { statfs.assume_init() }.f_type;
    if f_type != PROC_SUPER_MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("unexpected filesystem type in /proc ({:#x})", f_type),
        ));
    }

    Ok(())
}

fn cvt_i32(t: i32) -> io::Result<i32> {
    if t == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

fn cstr(path: &Path) -> io::Result<CString> {
    Ok(CString::new(path.as_os_str().as_bytes())?)
}

pub(crate) fn get_path_from_proc_self_fd(file: &fs::File) -> io::Result<PathBuf> {
    readlink_unchecked(proc_self_fd()?, Path::new(&file.as_raw_fd().to_string()))
        .map_err(Into::into)
}

pub(crate) fn set_permissions_through_proc_self_fd(
    file: &fs::File,
    perm: fs::Permissions,
) -> io::Result<()> {
    // We the `fchmodat` below to follow the magiclink, but if that resolves
    // to a symlink, we want it to stop following. So check for the file being
    // a symlink first.
    if file_metadata(file)?.file_type().is_symlink() {
        return Err(io::Error::from_raw_os_error(libc::ENOTSUP));
    }

    // Don't pass `AT_SYMLINK_NOFOLLOW`, because (a) Linux doesn't support it,
    // (b) some Linux libc implementations emulate it using procfs but without
    // the safety checks we do, and (c) we do actually want to follow the first
    // symlink. We don't want to follow any subsequent symlinks, but the check
    // above ensures that the destination of the link isn't a symlink.
    let atflags = 0;

    let fd = proc_self_fd()?.as_raw_fd();
    let mode = perm.mode();
    let cstr = cstr(Path::new(&file.as_raw_fd().to_string()))?;

    cvt_i32(unsafe { libc::fchmodat(fd, cstr.as_ptr(), mode, atflags) })?;

    Ok(())
}
