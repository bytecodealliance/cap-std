use crate::fs::{FollowSymlinks, OpenOptionsExt};

/// Options and flags which can be used to configure how a file is opened.
///
/// This corresponds to [`std::fs::OpenOptions`].
///
/// Note that this `OpenOptions` has no `open` method. To open a file with
/// an `OptionOptions`, you must first obtain a [`Dir`] containing the path, and
/// then call [`Dir::open_with`].
///
/// [`std::fs::OpenOptions`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
/// [`Dir`]: struct.Dir.html
/// [`Dir::open_with`]: struct.Dir.html#method.open_with
///
/// <details>
/// We need to define our own version because the libstd `OpenOptions` doesn't have
/// public accessors that we can use.
/// </details>
#[derive(Debug, Clone)]
pub struct OpenOptions {
    pub(crate) read: bool,
    pub(crate) write: bool,
    pub(crate) append: bool,
    pub(crate) truncate: bool,
    pub(crate) create: bool,
    pub(crate) create_new: bool,
    pub(crate) dir_required: bool,
    pub(crate) readdir_required: bool,
    pub(crate) follow: FollowSymlinks,

    #[cfg(any(unix, windows, target_os = "vxworks"))]
    pub(crate) ext: OpenOptionsExt,
}

impl OpenOptions {
    /// Creates a blank new set of options ready for configuration.
    ///
    /// This corresponds to [`std::fs::OpenOptions::new`].
    ///
    /// [`std::fs::OpenOptions::new`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.new
    #[allow(clippy::new_without_default)]
    #[inline]
    pub fn new() -> Self {
        Self {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
            dir_required: false,
            readdir_required: false,
            follow: FollowSymlinks::Yes,

            #[cfg(any(unix, windows, target_os = "vxworks"))]
            ext: OpenOptionsExt::new(),
        }
    }

    /// Sets the option for read access.
    ///
    /// This corresponds to [`std::fs::OpenOptions::read`].
    ///
    /// [`std::fs::OpenOptions::read`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.read
    #[inline]
    pub fn read(&mut self, read: bool) -> &mut Self {
        self.read = read;
        self
    }

    /// Sets the option for write access.
    ///
    /// This corresponds to [`std::fs::OpenOptions::write`].
    ///
    /// [`std::fs::OpenOptions::write`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.write
    #[inline]
    pub fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        self
    }

    /// Sets the option for the append mode.
    ///
    /// This corresponds to [`std::fs::OpenOptions::append`].
    ///
    /// [`std::fs::OpenOptions::append`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.append
    #[inline]
    pub fn append(&mut self, append: bool) -> &mut Self {
        self.append = append;
        self
    }

    /// Sets the option for truncating a previous file.
    ///
    /// This corresponds to [`std::fs::OpenOptions::truncate`].
    ///
    /// [`std::fs::OpenOptions::truncate`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.truncate
    #[inline]
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        self
    }

    /// Sets the option to create a new file.
    ///
    /// This corresponds to [`std::fs::OpenOptions::create`].
    ///
    /// [`std::fs::OpenOptions::create`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create
    #[inline]
    pub fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }

    /// Sets the option to always create a new file.
    ///
    /// This corresponds to [`std::fs::OpenOptions::create_new`].
    ///
    /// [`std::fs::OpenOptions::create_new`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new
    #[inline]
    pub fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.create_new = create_new;
        self
    }

    /// Sets the option to enable or suppress following of symlinks.
    #[inline]
    pub(crate) fn follow(&mut self, follow: FollowSymlinks) -> &mut Self {
        self.follow = follow;
        self
    }

    /// Sets the option to enable an error if the opened object is not a directory.
    #[inline]
    pub(crate) fn dir_required(&mut self, dir_required: bool) -> &mut Self {
        self.dir_required = dir_required;
        self
    }

    /// Sets the option to request the ability to read directory entries.
    #[inline]
    pub(crate) fn readdir_required(&mut self, readdir_required: bool) -> &mut Self {
        self.readdir_required = readdir_required;
        self
    }

    /// Wrapper to allow `follow` to be exposed by the `cap-fs-ext` crate.
    ///
    /// # Safety
    ///
    /// This is hidden from the main API since this functionality isn't present in `std`.
    /// Use `cap_fs_ext::OpenOptionsFollowExt` instead of calling this directly.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn _cap_fs_ext_follow(&mut self, follow: FollowSymlinks) -> &mut Self {
        self.follow(follow)
    }
}

#[cfg(unix)]
impl std::os::unix::fs::OpenOptionsExt for OpenOptions {
    #[inline]
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.ext.mode(mode);
        self
    }

    #[inline]
    fn custom_flags(&mut self, flags: i32) -> &mut Self {
        self.ext.custom_flags(flags);
        self
    }

    #[cfg(open_options_ext_as_flags)]
    fn as_flags(&self) -> io::Result<libc::c_int> {
        Ok(crate::posish::fs::oflags::get_access_mode(self)
            | crate::posish::fs::oflags::get_creation_mode(self)
            | self.custom_flags)
    }
}

#[cfg(target_os = "vxworks")]
impl std::os::vxworks::fs::OpenOptionsExt for OpenOptions {
    #[inline]
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.ext.mode(mode);
        self
    }

    #[inline]
    fn custom_flags(&mut self, flags: i32) -> &mut Self {
        self.ext.custom_flags(flags);
        self
    }
}

#[cfg(windows)]
impl std::os::windows::fs::OpenOptionsExt for OpenOptions {
    #[inline]
    fn access_mode(&mut self, access: u32) -> &mut Self {
        self.ext.access_mode(access);
        self
    }

    /// To prevent race conditions on Windows, handles for directories must be opened
    /// without `FILE_SHARE_DELETE`.
    #[inline]
    fn share_mode(&mut self, val: u32) -> &mut Self {
        self.ext.share_mode(val);
        self
    }

    #[inline]
    fn custom_flags(&mut self, flags: u32) -> &mut Self {
        self.ext.custom_flags(flags);
        self
    }

    #[inline]
    fn attributes(&mut self, val: u32) -> &mut Self {
        self.ext.attributes(val);
        self
    }

    #[inline]
    fn security_qos_flags(&mut self, flags: u32) -> &mut Self {
        self.ext.security_qos_flags(flags);
        self
    }
}

#[cfg(feature = "arbitrary")]
impl arbitrary::Arbitrary for OpenOptions {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        use arbitrary::Arbitrary;
        let (read, write) = match u.int_in_range(0..=2)? {
            0 => (true, false),
            1 => (false, true),
            2 => (true, true),
            _ => panic!(),
        };
        // TODO: `OpenOptionsExt` options.
        Ok(Self::new()
            .read(read)
            .write(write)
            .create(<bool as Arbitrary>::arbitrary(u)?)
            .append(<bool as Arbitrary>::arbitrary(u)?)
            .truncate(<bool as Arbitrary>::arbitrary(u)?)
            .create(<bool as Arbitrary>::arbitrary(u)?)
            .create_new(<bool as Arbitrary>::arbitrary(u)?)
            .dir_required(<bool as Arbitrary>::arbitrary(u)?)
            .readdir_required(<bool as Arbitrary>::arbitrary(u)?)
            .follow(<FollowSymlinks as Arbitrary>::arbitrary(u)?)
            .clone())
    }
}
