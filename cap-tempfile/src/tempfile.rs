//! Temporary files.

use cap_std::fs::{Dir, File};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::io::{self, Read, Seek, Write};

/// A file in a directory that is by default deleted when it goes out
/// of scope, but may also be written persistently.
///
/// This corresponds most closely to [`tempfile::NamedTempFile`]; however,
/// there are some important differences, so read the below carefully
/// to understand how to port existing code.
///
/// # Name-able, but not necessarily named
///
/// By default, the file does not necessarily have an name until the file is
/// written persistently.
///
/// On some operating systems like Linux, it is possible to create anonymous
/// temporary files that can still be written to disk persistently via
/// `O_TMPFILE`. The advantage of this is that if the process (or operating
/// system) crashes while the file is being written, the temporary space will
/// be automatically cleaned up. For this reason, there is no API to retrieve
/// the name, for either case.
///
/// To more closely match the semantics of [`tempfile::tempfile`], use
/// [`crate::TempFile::new_anonymous`].
///
/// # File permissions
///
/// Unlike the tempfile crate, the default [`TempFile::new`] will use the same
/// permissions as [`File::create_new`] in the Rust standard library.
/// Concretely on Unix systems for example this can (depending on `umask`)
/// result in files that are readable by all users. The rationale for this is
/// to make it more ergonomic and natural to use this API to atomically create
/// new files and replace existing ones. Many cases that want "private" files
/// will prefer [`TempFile::new_anonymous`] to have the file not be accessible
/// at all outside the current process.
///
/// To fully control the permissions of the resulting file, you can use
/// [`File::set_permissions`].
///
/// [`tempfile::tempfile`]: https://docs.rs/tempfile/latest/tempfile/fn.tempfile.html
/// [`tempfile::NamedTempFile`]: https://docs.rs/tempfile/latest/tempfile/struct.NamedTempFile.html
/// [`File::create_new`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new
/// [`File::set_permissions`]: https://docs.rs/cap-std/latest/cap_std/fs/struct.File.html#method.set_permissions
pub struct TempFile<'d> {
    dir: &'d Dir,
    fd: File,
    name: Option<String>,
}

impl<'d> Debug for TempFile<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Manual Debug implementation to omit the file reference and name so
        // we don't leak the path, the same as `cap_std::fs::File`.
        f.debug_struct("TempFile").field("dir", &self.dir).finish()
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn new_tempfile_linux(d: &Dir, anonymous: bool, mode: Option<u32>) -> io::Result<Option<File>> {
    use rustix::fs::{Mode, OFlags};
    // openat's API uses WRONLY. There may be use cases for reading too, so let's
    // support it.
    let mut oflags = OFlags::CLOEXEC | OFlags::TMPFILE | OFlags::RDWR;
    if anonymous {
        oflags |= OFlags::EXCL;
    }
    // We default to 0o666, same as main rust when creating new files; this will be
    // modified by umask: <https://github.com/rust-lang/rust/blob/44628f7273052d0bb8e8218518dacab210e1fe0d/library/std/src/sys/unix/fs.rs#L762>
    let mode = Mode::from(mode.unwrap_or(0o666));

    // Happy path - Linux with O_TMPFILE
    match rustix::fs::openat(d, ".", oflags, mode) {
        Ok(r) => Ok(Some(File::from(r))),
        // See <https://github.com/Stebalien/tempfile/blob/1a40687e06eb656044e3d2dffa1379f04b3ef3fd/src/file/imp/unix.rs#L81>
        Err(rustix::io::Errno::OPNOTSUPP | rustix::io::Errno::ISDIR | rustix::io::Errno::NOENT) => {
            Ok(None)
        }
        Err(e) => Err(e.into()),
    }
}

/// Assign a random name to a currently anonymous O_TMPFILE descriptor.
#[cfg(any(target_os = "android", target_os = "linux"))]
fn generate_name_in(subdir: &Dir, f: &File) -> io::Result<String> {
    use rustix::fd::AsFd;
    use rustix::fs::AtFlags;
    let procself_fd = rustix_linux_procfs::proc_self_fd()?;
    let fdnum = rustix::path::DecInt::from_fd(f.as_fd());
    let fdnum = fdnum.as_c_str();
    super::retry_with_name_ignoring(io::ErrorKind::AlreadyExists, |name| {
        rustix::fs::linkat(procself_fd, fdnum, subdir, name, AtFlags::SYMLINK_FOLLOW)
            .map_err(Into::into)
    })
    .map(|(_, name)| name)
}

/// Create a new temporary file in the target directory, which may or may not
/// have a (randomly generated) name at this point. If anonymous is specified,
/// the file will be deleted
fn new_tempfile(
    d: &Dir,
    anonymous: bool,
    #[cfg(any(target_os = "android", target_os = "linux"))] unnamed_mode: Option<u32>,
    #[cfg(unix)] named_mode: Option<u32>,
) -> io::Result<(File, Option<String>)> {
    // On Linux, try O_TMPFILE
    #[cfg(any(target_os = "android", target_os = "linux"))]
    if let Some(f) = new_tempfile_linux(d, anonymous, unnamed_mode)? {
        return Ok((f, None));
    }
    // Otherwise, fall back to just creating a randomly named file.
    let mut opts = cap_std::fs::OpenOptions::new();
    opts.read(true);
    opts.write(true);
    opts.create_new(true);
    #[cfg(unix)]
    {
        use cap_std::fs::OpenOptionsExt;
        if let Some(mode) = named_mode {
            opts.mode(mode);
        }
    }
    let (f, name) = super::retry_with_name_ignoring(io::ErrorKind::AlreadyExists, |name| {
        d.open_with(name, &opts)
    })?;
    if anonymous {
        d.remove_file(name)?;
        Ok((f, None))
    } else {
        Ok((f, Some(name)))
    }
}

impl<'d> TempFile<'d> {
    /// Create a new temporary file in the provided directory.
    pub fn new(dir: &'d Dir) -> io::Result<Self> {
        let (fd, name) = new_tempfile(
            dir,
            false,
            #[cfg(any(target_os = "android", target_os = "linux"))]
            None,
            #[cfg(unix)]
            None,
        )?;
        Ok(Self { dir, fd, name })
    }

    /// Create a new temporary file in the provided directory, with the provided modes.
    /// `unnamed_mode` is used when the file is unnamed (created with `O_TMPFILE`).
    /// `named_mode` is used when the file is named (fallback case).
    /// Process umask is taken into account for the actual file mode.
    #[cfg(unix)]
    pub fn new_with_modes(
        dir: &'d Dir,
        #[cfg(any(target_os = "android", target_os = "linux"))] unnamed_mode: Option<u32>,
        named_mode: Option<u32>,
    ) -> io::Result<Self> {
        let (fd, name) = new_tempfile(
            dir,
            false,
            #[cfg(any(target_os = "android", target_os = "linux"))]
            unnamed_mode,
            named_mode,
        )?;
        Ok(Self { dir, fd, name })
    }

    /// Create a new temporary file in the provided directory that will not have
    /// a name. This corresponds to [`tempfile::tempfile_in`].
    ///
    /// [`tempfile::tempfile_in`]: https://docs.rs/tempfile/latest/tempfile/fn.tempfile_in.html
    pub fn new_anonymous(dir: &'d Dir) -> io::Result<File> {
        new_tempfile(
            dir,
            true,
            #[cfg(any(target_os = "android", target_os = "linux"))]
            None,
            #[cfg(unix)]
            Some(0o000),
        )
        .map(|v| v.0)
    }

    /// Get a reference to the underlying file.
    pub fn as_file(&self) -> &File {
        &self.fd
    }

    /// Get a mutable reference to the underlying file.
    pub fn as_file_mut(&mut self) -> &mut File {
        &mut self.fd
    }

    /// Returns whether the tempfile is named (visible in the folder)
    pub fn is_named(&self) -> bool {
        self.name.is_some()
    }

    fn impl_replace(mut self, destname: &OsStr) -> io::Result<()> {
        // At this point on Linux if O_TMPFILE is used, we need to give the file a
        // temporary name in order to link it into place. There are patches to
        // add an `AT_LINKAT_REPLACE` API. With that we could skip this and
        // have file-leak-proof atomic file replacement: <https://marc.info/?l=linux-fsdevel&m=158028833007418&w=2>
        #[cfg(any(target_os = "android", target_os = "linux"))]
        let tempname = self
            .name
            .take()
            .map(Ok)
            .unwrap_or_else(|| generate_name_in(self.dir, &self.fd))?;
        // SAFETY: We only support anonymous files on Linux, so the file must have a
        // name here.
        #[cfg(not(any(target_os = "android", target_os = "linux")))]
        let tempname = self.name.take().unwrap();
        // And try the rename into place.
        self.dir.rename(&tempname, self.dir, destname).map_err(|e| {
            // But, if we catch an error here, then move ownership back into self,
            // which means the Drop invocation will clean it up.
            self.name = Some(tempname);
            e
        })
    }

    /// Write the file to the target directory with the provided name.
    /// Any existing file will be replaced.
    ///
    /// The file permissions will default to read-only.
    pub fn replace(self, destname: impl AsRef<OsStr>) -> io::Result<()> {
        let destname = destname.as_ref();
        self.impl_replace(destname)
    }
}

impl<'d> Read for TempFile<'d> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_file_mut().read(buf)
    }
}

impl<'d> Write for TempFile<'d> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.as_file_mut().write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.as_file_mut().flush()
    }
}

impl<'d> Seek for TempFile<'d> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.as_file_mut().seek(pos)
    }
}

impl<'d> Drop for TempFile<'d> {
    fn drop(&mut self) {
        if let Some(name) = self.name.take() {
            let _ = self.dir.remove_file(name);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// On Unix, calling `umask()` actually *mutates* the process global state.
    /// This uses Linux `/proc` to read the current value.
    #[cfg(any(target_os = "android", target_os = "linux"))]
    fn get_process_umask() -> io::Result<u32> {
        use io::BufRead;
        let status = std::fs::File::open("/proc/self/status")?;
        let bufr = io::BufReader::new(status);
        for line in bufr.lines() {
            let line = line?;
            let l = if let Some(v) = line.split_once(':') {
                v
            } else {
                continue;
            };
            let (k, v) = l;
            if k != "Umask" {
                continue;
            }
            return Ok(u32::from_str_radix(v.trim(), 8).unwrap());
        }
        panic!("Could not determine process umask")
    }

    /// Older Windows versions don't support removing open files
    fn os_supports_unlinked_tmp(d: &Dir) -> bool {
        if cfg!(not(windows)) {
            return true;
        }
        let name = "testfile";
        let _f = d.create(name).unwrap();
        d.remove_file(name).and_then(|_| d.create(name)).is_ok()
    }

    #[test]
    fn test_tempfile() -> io::Result<()> {
        use crate::ambient_authority;

        let td = crate::tempdir(ambient_authority())?;

        // Base case, verify we clean up on drop
        let tf = TempFile::new(&td).unwrap();
        drop(tf);
        assert_eq!(td.entries()?.into_iter().count(), 0);

        let mut tf = TempFile::new(&td)?;
        // Test that we created with the right permissions
        #[cfg(any(target_os = "android", target_os = "linux"))]
        {
            use cap_std::fs::MetadataExt;
            let umask = get_process_umask()?;
            let mode = tf.as_file().metadata()?.mode();
            assert_eq!(0o666 & !umask, mode & 0o777);
        }
        // And that we can write
        tf.write_all(b"hello world")?;
        drop(tf);
        assert_eq!(td.entries()?.into_iter().count(), 0);

        let mut tf = TempFile::new(&td)?;
        tf.write_all(b"hello world")?;
        tf.replace("testfile").unwrap();
        assert_eq!(td.entries()?.into_iter().count(), 1);

        assert_eq!(td.read("testfile")?, b"hello world");

        if os_supports_unlinked_tmp(&td) {
            let mut tf = TempFile::new_anonymous(&td).unwrap();
            tf.write_all(b"hello world, I'm anonymous").unwrap();
            tf.seek(std::io::SeekFrom::Start(0)).unwrap();
            let mut buf = String::new();
            tf.read_to_string(&mut buf).unwrap();
            assert_eq!(&buf, "hello world, I'm anonymous");
        } else if cfg!(windows) {
            eprintln!("notice: Detected older Windows");
        }

        // Test that we can create with 0o000 mode
        #[cfg(any(target_os = "android", target_os = "linux"))]
        {
            use cap_std::fs::MetadataExt;
            let mut tf = TempFile::new_with_modes(&td, Some(0o000), Some(0o000))?;
            assert_eq!(tf.as_file().metadata()?.mode() & 0o777, 0o000);
            tf.write_all(b"mode 0")?;
            tf.replace("testfile")?;
            let metadata = td.metadata("testfile")?;
            assert_eq!(metadata.len(), 6);
            assert_eq!(metadata.mode() & 0o777, 0o000);
        }

        // Test that mode is limited by umask
        #[cfg(any(target_os = "android", target_os = "linux"))]
        {
            use cap_std::fs::MetadataExt;
            let tf = TempFile::new_with_modes(&td, Some(0o777), Some(0o777))?;
            let umask = get_process_umask()?;
            assert_ne!(umask & 0o777, 0o000);
            assert_eq!(tf.as_file().metadata()?.mode() & 0o777, 0o777 & !umask);
        }

        td.close()
    }
}
