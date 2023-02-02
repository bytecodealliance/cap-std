//! Windows implementation of `openat` functionality.

#![allow(unsafe_code)]

use super::create_file_at_w::CreateFileAtW;
use super::{open_options_to_std, prepare_open_options_for_open};
use crate::fs::{
    errors, get_access_mode, get_creation_mode, get_flags_and_attributes, FollowSymlinks,
    OpenOptions, OpenUncheckedError, SymlinkKind,
};
use crate::{ambient_authority, AmbientAuthority};
use std::convert::TryInto;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::fs::MetadataExt;
use std::os::windows::io::{AsRawHandle, OwnedHandle};
use std::path::Path;
use std::{fs, io, ptr};
use windows_sys::Win32::Foundation::{
    self, GetLastError, SetLastError, ERROR_INSUFFICIENT_BUFFER, HANDLE,
};
use windows_sys::Win32::Storage::FileSystem::GetFullPathNameW;
use windows_sys::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_DIRECTORY, FILE_FLAG_OPEN_REPARSE_POINT,
};

/// *Unsandboxed* function similar to `open`, but which does not perform
/// sandboxing.
pub(crate) fn open_unchecked(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> Result<fs::File, OpenUncheckedError> {
    let _ = ambient_authority;

    // We have the final `OpenOptions`; now prepare it for an `open`.
    let mut prepared_opts = options.clone();
    let manually_trunc = prepare_open_options_for_open(&mut prepared_opts);

    handle_open_result(open_at(&start, path, &prepared_opts), &options, manually_trunc)
}

// The following is derived from Rust's library/std/src/sys/windows/fs.rs
// at revision 56888c1e9b4135b511abd2d8e907099003d12281, except with a
// directory `start` parameter added and using `CreateFileAtW` instead of
// `CreateFileW`.

fn open_at(start: &fs::File, path: &Path, opts: &OpenOptions) -> io::Result<fs::File> {
    let path = maybe_verbatim(path)?;
    let handle = unsafe {
        CreateFileAtW(
            start.as_raw_handle() as HANDLE,
            path.as_ptr(),
            get_access_mode(opts)?,
            opts.ext.share_mode,
            opts.ext.security_attributes,
            get_creation_mode(opts)?,
            get_flags_and_attributes(opts),
            0 as HANDLE,
        )
    };
    if let Ok(handle) = handle.try_into() {
        Ok(<fs::File as From<OwnedHandle>>::from(handle))
    } else {
        Err(io::Error::last_os_error())
    }
}

// The following is derived from Rust's library/std/src/sys/windows/path.rs
// at revision 0fe54d46509abbbe54292d0ff85f8429301be002.

/// Returns a UTF-16 encoded path capable of bypassing the legacy `MAX_PATH`
/// limits.
///
/// This path may or may not have a verbatim prefix.
pub(crate) fn maybe_verbatim(path: &Path) -> io::Result<Vec<u16>> {
    // Normally the MAX_PATH is 260 UTF-16 code units (including the NULL).
    // However, for APIs such as CreateDirectory[1], the limit is 248.
    //
    // [1]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createdirectorya#parameters
    const LEGACY_MAX_PATH: usize = 248;
    // UTF-16 encoded code points, used in parsing and building UTF-16 paths.
    // All of these are in the ASCII range so they can be cast directly to `u16`.
    const SEP: u16 = b'\\' as _;
    const ALT_SEP: u16 = b'/' as _;
    const QUERY: u16 = b'?' as _;
    const COLON: u16 = b':' as _;
    const DOT: u16 = b'.' as _;
    const U: u16 = b'U' as _;
    const N: u16 = b'N' as _;
    const C: u16 = b'C' as _;

    // \\?\
    const VERBATIM_PREFIX: &[u16] = &[SEP, SEP, QUERY, SEP];
    // \??\
    const NT_PREFIX: &[u16] = &[SEP, QUERY, QUERY, SEP];
    // \\?\UNC\
    const UNC_PREFIX: &[u16] = &[SEP, SEP, QUERY, SEP, U, N, C, SEP];

    let mut path = to_u16s(path)?;
    if path.starts_with(VERBATIM_PREFIX) || path.starts_with(NT_PREFIX) || path == &[0] {
        // Early return for paths that are already verbatim or empty.
        return Ok(path);
    } else if path.len() < LEGACY_MAX_PATH {
        // Early return if an absolute path is less < 260 UTF-16 code units.
        // This is an optimization to avoid calling `GetFullPathNameW` unnecessarily.
        match path.as_slice() {
            // Starts with `D:`, `D:\`, `D:/`, etc.
            // Does not match if the path starts with a `\` or `/`.
            [drive, COLON, 0] | [drive, COLON, SEP | ALT_SEP, ..]
                if *drive != SEP && *drive != ALT_SEP =>
            {
                return Ok(path);
            }
            // Starts with `\\`, `//`, etc
            [SEP | ALT_SEP, SEP | ALT_SEP, ..] => return Ok(path),
            _ => {}
        }
    }

    // Firstly, get the absolute path using `GetFullPathNameW`.
    // https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfullpathnamew
    let lpfilename = path.as_ptr();
    fill_utf16_buf(
        // SAFETY: `fill_utf16_buf` ensures the `buffer` and `size` are valid.
        // `lpfilename` is a pointer to a null terminated string that is not
        // invalidated until after `GetFullPathNameW` returns successfully.
        |buffer, size| unsafe { GetFullPathNameW(lpfilename, size, buffer, ptr::null_mut()) },
        |mut absolute| {
            path.clear();

            // Secondly, add the verbatim prefix. This is easier here because we know the
            // path is now absolute and fully normalized (e.g. `/` has been changed to
            // `\`).
            let prefix = match absolute {
                // C:\ => \\?\C:\
                [_, COLON, SEP, ..] => VERBATIM_PREFIX,
                // \\.\ => \\?\
                [SEP, SEP, DOT, SEP, ..] => {
                    absolute = &absolute[4..];
                    VERBATIM_PREFIX
                }
                // Leave \\?\ and \??\ as-is.
                [SEP, SEP, QUERY, SEP, ..] | [SEP, QUERY, QUERY, SEP, ..] => &[],
                // \\ => \\?\UNC\
                [SEP, SEP, ..] => {
                    absolute = &absolute[2..];
                    UNC_PREFIX
                }
                // Anything else we leave alone.
                _ => &[],
            };

            path.reserve_exact(prefix.len() + absolute.len() + 1);
            path.extend_from_slice(prefix);
            path.extend_from_slice(absolute);
            path.push(0);
        },
    )?;
    Ok(path)
}

// The following is derived from Rust's library/std/src/sys/windows/mod.rs
// at revision a9e5c1a309df80434ebc4c1f6bfaa5cb119b465d, except with the
// optimization in f50f8782fe5d6f617d9c5b20115a7639dc7521bc reverted, to
// avoid depending on Rust nightly features.

pub fn unrolled_find_u16s(needle: u16, haystack: &[u16]) -> Option<usize> {
    let ptr = haystack.as_ptr();
    let mut start = &haystack[..];

    // For performance reasons unfold the loop eight times.
    while start.len() >= 8 {
        macro_rules! if_return {
            ($($n:literal,)+) => {
                $(
                    if start[$n] == needle {
                        return Some(((&start[$n] as *const u16) as usize - ptr as usize) / 2);
                    }
                )+
            }
        }

        if_return!(0, 1, 2, 3, 4, 5, 6, 7,);

        start = &start[8..];
    }

    for c in start {
        if *c == needle {
            return Some(((c as *const u16) as usize - ptr as usize) / 2);
        }
    }
    None
}

pub fn to_u16s<S: AsRef<OsStr>>(s: S) -> std::io::Result<Vec<u16>> {
    fn inner(s: &OsStr) -> std::io::Result<Vec<u16>> {
        // Most paths are ASCII, so reserve capacity for as much as there are bytes
        // in the OsStr plus one for the null-terminating character. We are not
        // wasting bytes here as paths created by this function are primarily used
        // in an ephemeral fashion.
        let mut maybe_result = Vec::with_capacity(s.len() + 1);
        maybe_result.extend(s.encode_wide());

        if unrolled_find_u16s(0, &maybe_result).is_some() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "strings passed to WinAPI cannot contain NULs",
            ));
        }
        maybe_result.push(0);
        Ok(maybe_result)
    }
    inner(s.as_ref())
}

// Many Windows APIs follow a pattern of where we hand a buffer and then they
// will report back to us how large the buffer should be or how many bytes
// currently reside in the buffer. This function is an abstraction over these
// functions by making them easier to call.
//
// The first callback, `f1`, is yielded a (pointer, len) pair which can be
// passed to a syscall. The `ptr` is valid for `len` items (u16 in this case).
// The closure is expected to return what the syscall returns which will be
// interpreted by this function to determine if the syscall needs to be invoked
// again (with more buffer space).
//
// Once the syscall has completed (errors bail out early) the second closure is
// yielded the data which has been read from the syscall. The return value
// from this closure is then the return value of the function.
fn fill_utf16_buf<F1, F2, T>(mut f1: F1, f2: F2) -> std::io::Result<T>
where
    F1: FnMut(*mut u16, u32) -> u32,
    F2: FnOnce(&[u16]) -> T,
{
    // Start off with a stack buf but then spill over to the heap if we end up
    // needing more space.
    //
    // This initial size also works around `GetFullPathNameW` returning
    // incorrect size hints for some short paths:
    // https://github.com/dylni/normpath/issues/5
    let mut stack_buf = [0u16; 512];
    let mut heap_buf = Vec::new();
    unsafe {
        let mut n = stack_buf.len();
        loop {
            let buf = if n <= stack_buf.len() {
                &mut stack_buf[..]
            } else {
                let extra = n - heap_buf.len();
                heap_buf.reserve(extra);
                heap_buf.set_len(n);
                &mut heap_buf[..]
            };

            // This function is typically called on windows API functions which
            // will return the correct length of the string, but these functions
            // also return the `0` on error. In some cases, however, the
            // returned "correct length" may actually be 0!
            //
            // To handle this case we call `SetLastError` to reset it to 0 and
            // then check it again if we get the "0 error value". If the "last
            // error" is still 0 then we interpret it as a 0 length buffer and
            // not an actual error.
            SetLastError(0);
            let k = match f1(buf.as_mut_ptr(), n as u32) {
                0 if GetLastError() == 0 => 0,
                0 => return Err(std::io::Error::last_os_error()),
                n => n,
            } as usize;
            if k == n && GetLastError() == ERROR_INSUFFICIENT_BUFFER {
                n += 2;
            } else if k > n {
                n = k;
            } else if k == n {
                // It is impossible to reach this point.
                // On success, k is the returned string length excluding the null.
                // On failure, k is the required buffer length including the null.
                // Therefore k never equals n.
                unreachable!();
            } else {
                return Ok(f2(&buf[..k]));
            }
        }
    }
}

/// *Unsandboxed* function similar to `open_unchecked`, but which just operates
/// on a bare path, rather than starting with a handle.
pub(crate) fn open_ambient_impl(
    path: &Path,
    options: &OpenOptions,
    ambient_authority: AmbientAuthority,
) -> Result<fs::File, OpenUncheckedError> {
    let _ = ambient_authority;
    let (std_opts, manually_trunc) = open_options_to_std(options);
    handle_open_result(std_opts.open(path), &options, manually_trunc)
}

fn handle_open_result(result: io::Result<fs::File>, options: &OpenOptions, manually_trunc: bool
) -> Result<fs::File, OpenUncheckedError> {
    match result {
        Ok(f) => {
            let enforce_dir = options.dir_required;
            let enforce_nofollow = options.follow == FollowSymlinks::No
                && (options.ext.custom_flags & FILE_FLAG_OPEN_REPARSE_POINT) == 0;

            if enforce_dir || enforce_nofollow {
                let metadata = f.metadata().map_err(OpenUncheckedError::Other)?;

                if enforce_dir {
                    // Require a directory. It may seem possible to eliminate
                    // this `metadata()` call by appending a slash to the path
                    // before opening it so that the OS requires a directory
                    // for us, however on Windows in some circumstances this
                    // leads to "The filename, directory name, or volume label
                    // syntax is incorrect." errors.
                    //
                    // We check `file_attributes()` instead of using `is_dir()`
                    // since the latter returns false if we're looking at a
                    // directory symlink.
                    if metadata.file_attributes() & FILE_ATTRIBUTE_DIRECTORY == 0 {
                        return Err(OpenUncheckedError::Other(errors::is_not_directory()));
                    }
                }

                if enforce_nofollow {
                    // Windows doesn't have a way to return errors like
                    // `O_NOFOLLOW`, so if we're not following symlinks and
                    // we're not using `FILE_FLAG_OPEN_REPARSE_POINT` manually
                    // to open a symlink itself, check for symlinks and report
                    // them as a distinct error.
                    if metadata.file_type().is_symlink() {
                        return Err(OpenUncheckedError::Symlink(
                            io::Error::from_raw_os_error(
                                Foundation::ERROR_STOPPED_ON_SYMLINK as i32,
                            ),
                            if metadata.file_attributes() & FILE_ATTRIBUTE_DIRECTORY
                                == FILE_ATTRIBUTE_DIRECTORY
                            {
                                SymlinkKind::Dir
                            } else {
                                SymlinkKind::File
                            },
                        ));
                    }
                }
            }

            // Windows truncates symlinks into normal files, so truncation
            // may be disabled above; do it manually if needed.
            if manually_trunc {
                // Unwrap is ok because 0 never overflows, and we'll only
                // have `manually_trunc` set when the file is opened for
                // writing.
                f.set_len(0).unwrap();
            }
            Ok(f)
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Err(OpenUncheckedError::NotFound(e)),
        Err(e) => match e.raw_os_error() {
            Some(code) => match code as u32 {
                Foundation::ERROR_FILE_NOT_FOUND | Foundation::ERROR_PATH_NOT_FOUND => {
                    Err(OpenUncheckedError::NotFound(e))
                }
                _ => Err(OpenUncheckedError::Other(e)),
            },
            None => Err(OpenUncheckedError::Other(e)),
        },
    }
}
