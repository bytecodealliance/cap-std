// Implementation derived from `copy` in Rust's
// library/std/src/sys/unix/fs.rs at revision
// 108e90ca78f052c0c1c49c42a22c85620be19712.

#[cfg(any(target_os = "macos", target_os = "ios"))]
use super::{cstr, cvt_i32};
use crate::fs::{open, OpenOptions};
#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios"
))]
use std::os::unix::io::AsRawFd;
#[cfg(any(target_os = "linux", target_os = "android"))]
use std::ptr;
use std::{fs, io, path::Path};

#[cfg(any(target_os = "macos", target_os = "ios"))]
macro_rules! syscall {
    (fn $name:ident($($arg_name:ident: $t:ty),*) -> $ret:ty) => (
        unsafe fn $name($($arg_name: $t),*) -> $ret {
            weak! { fn $name($($t),*) -> $ret }

            if let Some(fun) = $name.get() {
                fun($($arg_name),*)
            } else {
                errno::set_errno(errno::Errno(libc::ENOSYS));
                -1
            }
        }
    )
}

fn open_from(start: &fs::File, path: &Path) -> io::Result<(fs::File, fs::Metadata)> {
    let reader = open(start, path, OpenOptions::new().read(true))?;
    let metadata = reader.metadata()?;
    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "the source path is not an existing regular file",
        ));
    }
    Ok((reader, metadata))
}

fn open_to_and_set_permissions(
    start: &fs::File,
    path: &Path,
    reader_metadata: fs::Metadata,
) -> io::Result<(fs::File, fs::Metadata)> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

    let perm = reader_metadata.permissions();
    let writer = open(
        start,
        path,
        OpenOptions::new()
            // create the file with the correct mode right away
            .mode(perm.mode())
            .write(true)
            .create(true)
            .truncate(true),
    )?;
    let writer_metadata = writer.metadata()?;
    if writer_metadata.is_file() {
        // Set the correct file permissions, in case the file already existed.
        // Don't set the permissions on already existing non-files like
        // pipes/FIFOs or device nodes.
        writer.set_permissions(perm)?;
    }
    Ok((writer, writer_metadata))
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios"
)))]
pub(crate) fn copy_impl(
    from_start: &fs::File,
    from_path: &Path,
    to_start: &fs::File,
    to_path: &Path,
) -> io::Result<u64> {
    let (mut reader, reader_metadata) = open_from(from_start, from_path)?;
    let (mut writer, _) = open_to_and_set_permissions(to_start, to_path, reader_metadata)?;

    io::copy(&mut reader, &mut writer)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) fn copy_impl(
    from_start: &fs::File,
    from_path: &Path,
    to_start: &fs::File,
    to_path: &Path,
) -> io::Result<u64> {
    use std::{
        cmp,
        sync::atomic::{AtomicBool, Ordering},
    };

    // Kernel prior to 4.5 don't have copy_file_range
    // We store the availability in a global to avoid unnecessary syscalls
    static HAS_COPY_FILE_RANGE: AtomicBool = AtomicBool::new(true);

    unsafe fn copy_file_range(
        fd_in: libc::c_int,
        off_in: *mut libc::loff_t,
        fd_out: libc::c_int,
        off_out: *mut libc::loff_t,
        len: libc::size_t,
        flags: libc::c_uint,
    ) -> libc::c_long {
        libc::syscall(
            libc::SYS_copy_file_range,
            fd_in,
            off_in,
            fd_out,
            off_out,
            len,
            flags,
        )
    }

    let (mut reader, reader_metadata) = open_from(from_start, from_path)?;
    let len = reader_metadata.len();
    let (mut writer, _) = open_to_and_set_permissions(to_start, to_path, reader_metadata)?;

    let has_copy_file_range = HAS_COPY_FILE_RANGE.load(Ordering::Relaxed);
    let mut written = 0_u64;
    while written < len {
        let copy_result = if has_copy_file_range {
            let bytes_to_copy = cmp::min(len - written, usize::MAX as u64) as usize;
            let copy_result = unsafe {
                // We actually don't have to adjust the offsets,
                // because copy_file_range adjusts the file offset automatically
                let result = copy_file_range(
                    reader.as_raw_fd(),
                    ptr::null_mut(),
                    writer.as_raw_fd(),
                    ptr::null_mut(),
                    bytes_to_copy,
                    0,
                );
                #[cfg(target_pointer_width = "64")]
                {
                    crate::fs::cvt_i64(result)
                }
                #[cfg(target_pointer_width = "32")]
                {
                    crate::fs::cvt_i32(result)
                }
            };
            if let Err(ref copy_err) = copy_result {
                match copy_err.raw_os_error() {
                    Some(libc::ENOSYS) | Some(libc::EPERM) => {
                        HAS_COPY_FILE_RANGE.store(false, Ordering::Relaxed);
                    }
                    _ => {}
                }
            }
            copy_result
        } else {
            Err(io::Error::from_raw_os_error(libc::ENOSYS))
        };
        match copy_result {
            Ok(ret) => written += ret as u64,
            Err(err) => {
                match err.raw_os_error() {
                    Some(os_err)
                        if os_err == libc::ENOSYS
                            || os_err == libc::EXDEV
                            || os_err == libc::EINVAL
                            || os_err == libc::EPERM =>
                    {
                        // Try fallback io::copy if either:
                        // - Kernel version is < 4.5 (ENOSYS)
                        // - Files are mounted on different fs (EXDEV)
                        // - copy_file_range is disallowed, for example by seccomp (EPERM)
                        // - copy_file_range cannot be used with pipes or device nodes (EINVAL)
                        assert_eq!(written, 0);
                        return io::copy(&mut reader, &mut writer);
                    }
                    _ => return Err(err),
                }
            }
        }
    }
    Ok(written)
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[allow(non_upper_case_globals)]
pub(crate) fn copy_impl(
    from_start: &fs::File,
    from_path: &Path,
    to_start: &fs::File,
    to_path: &Path,
) -> io::Result<u64> {
    use std::sync::atomic::{AtomicBool, Ordering};

    const COPYFILE_ACL: u32 = 1 << 0;
    const COPYFILE_STAT: u32 = 1 << 1;
    const COPYFILE_XATTR: u32 = 1 << 2;
    const COPYFILE_DATA: u32 = 1 << 3;

    const COPYFILE_SECURITY: u32 = COPYFILE_STAT | COPYFILE_ACL;
    const COPYFILE_METADATA: u32 = COPYFILE_SECURITY | COPYFILE_XATTR;
    const COPYFILE_ALL: u32 = COPYFILE_METADATA | COPYFILE_DATA;

    const COPYFILE_STATE_COPIED: u32 = 8;

    #[allow(non_camel_case_types)]
    type copyfile_state_t = *mut libc::c_void;
    #[allow(non_camel_case_types)]
    type copyfile_flags_t = u32;

    extern "C" {
        fn fcopyfile(
            from: libc::c_int,
            to: libc::c_int,
            state: copyfile_state_t,
            flags: copyfile_flags_t,
        ) -> libc::c_int;
        fn copyfile_state_alloc() -> copyfile_state_t;
        fn copyfile_state_free(state: copyfile_state_t) -> libc::c_int;
        fn copyfile_state_get(
            state: copyfile_state_t,
            flag: u32,
            dst: *mut libc::c_void,
        ) -> libc::c_int;
    }

    struct FreeOnDrop(copyfile_state_t);
    impl Drop for FreeOnDrop {
        fn drop(&mut self) {
            // The code below ensures that `FreeOnDrop` is never a null pointer
            unsafe {
                // `copyfile_state_free` returns -1 if the `to` or `from` files
                // cannot be closed. However, this is not considered this an
                // error.
                copyfile_state_free(self.0);
            }
        }
    }

    // MacOS prior to 10.12 don't support `fclonefileat`
    // We store the availability in a global to avoid unnecessary syscalls
    static HAS_FCLONEFILEAT: AtomicBool = AtomicBool::new(true);
    syscall! {
        fn fclonefileat(
            srcfd: libc::c_int,
            dst_dirfd: libc::c_int,
            dst: *const libc::c_char,
            flags: libc::c_int
        ) -> libc::c_int
    }

    let (reader, reader_metadata) = open_from(from_start, from_path)?;

    // Opportunistically attempt to create a copy-on-write clone of `from_path`
    // using `fclonefileat`.
    if HAS_FCLONEFILEAT.load(Ordering::Relaxed) {
        let to_path = cstr(to_path)?;
        let clonefile_result = cvt_i32(unsafe {
            fclonefileat(
                reader.as_raw_fd(),
                to_start.as_raw_fd(),
                to_path.as_ptr(),
                0,
            )
        });
        match clonefile_result {
            Ok(_) => return Ok(reader_metadata.len()),
            Err(err) => match err.raw_os_error() {
                // `fclonefileat` will fail on non-APFS volumes, if the
                // destination already exists, or if the source and destination
                // are on different devices. In all these cases `fcopyfile`
                // should succeed.
                Some(libc::ENOTSUP) | Some(libc::EEXIST) | Some(libc::EXDEV) => (),
                Some(libc::ENOSYS) => HAS_FCLONEFILEAT.store(false, Ordering::Relaxed),
                _ => return Err(err),
            },
        }
    }

    // Fall back to using `fcopyfile` if `fclonefileat` does not succeed.
    let (writer, writer_metadata) =
        open_to_and_set_permissions(to_start, to_path, reader_metadata)?;

    // We ensure that `FreeOnDrop` never contains a null pointer so it is
    // always safe to call `copyfile_state_free`
    let state = unsafe {
        let state = copyfile_state_alloc();
        if state.is_null() {
            return Err(std::io::Error::last_os_error());
        }
        FreeOnDrop(state)
    };

    let flags = if writer_metadata.is_file() {
        COPYFILE_ALL
    } else {
        COPYFILE_DATA
    };

    cvt_i32(unsafe { fcopyfile(reader.as_raw_fd(), writer.as_raw_fd(), state.0, flags) })?;

    let mut bytes_copied: libc::off_t = 0;
    cvt_i32(unsafe {
        copyfile_state_get(
            state.0,
            COPYFILE_STATE_COPIED,
            &mut bytes_copied as *mut libc::off_t as *mut libc::c_void,
        )
    })?;
    Ok(bytes_copied as u64)
}
