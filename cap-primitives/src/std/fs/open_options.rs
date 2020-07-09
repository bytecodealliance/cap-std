use crate::fs::OpenOptionsExt;

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
    pub(crate) nofollow: bool,

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
            nofollow: false,

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

    /// Sets the option to suppress following of symlinks.
    #[inline]
    pub(crate) fn nofollow(&mut self, nofollow: bool) -> &mut Self {
        self.nofollow = nofollow;
        self
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
        todo!("OpenOptionsExt::access_mode for Windows")
    }

    #[inline]
    fn share_mode(&mut self, val: u32) -> &mut Self {
        todo!("OpenOptionsExt::share_mode for Windows")
    }

    #[inline]
    fn custom_flags(&mut self, flags: u32) -> &mut Self {
        todo!("OpenOptionsExt::custom_flags for Windows")
    }

    #[inline]
    fn attributes(&mut self, val: u32) -> &mut Self {
        todo!("OpenOptionsExt::attributes for Windows")
    }

    #[inline]
    fn security_qos_flags(&mut self, flags: u32) -> &mut fs::OpenOptions {
        todo!(
            "we need to change the return type of OpenOptionsExt::security_qos_flags in libstd before we can implement this"
        )
    }
}
