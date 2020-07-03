use async_std::fs;

/// A builder used to create directories in various manners.
///
/// This corresponds to [`std::fs::DirBuilder`].
///
/// Unlike `async_std::fs::DirBuilder`, this API has no `DirBuilder::create`, because
/// creating directories requires a capability. Use
/// [`Dir::create_with_dir_builder`] instead.
///
/// [`std::fs::DirBuilder`]: https://doc.rust-lang.org/std/fs/struct.DirBuilder.html
/// [`Dir::create_with_dir_builder`]: https://doc.rust-lang.org/std/fs/struct.Dir.html#method.create_with_dir_builder
pub struct DirBuilder {
    std: fs::DirBuilder,
}

impl DirBuilder {
    /// Constructs a new instance of `Self` from the given `async_std::fs::File`.
    #[inline]
    pub fn from_std(std: fs::DirBuilder) -> Self {
        Self { std }
    }

    /// Creates a new set of options with default mode/security settings for all platforms and also non-recursive.
    ///
    /// This corresponds to [`std::fs::DirBuilder::new`].
    ///
    /// [`std::fs::DirBuilder::new`]: https://doc.rust-lang.org/std/fs/struct.DirBuilder.html#method.new
    #[allow(clippy::new_without_default)]
    #[inline]
    pub fn new() -> Self {
        Self {
            std: fs::DirBuilder::new(),
        }
    }

    /// Indicates that directories should be created recursively, creating all parent directories.
    ///
    /// This corresponds to [`std::fs::DirBuilder::recursive`].
    ///
    /// [`std::fs::DirBuilder::recursive`]: https://doc.rust-lang.org/std/fs/struct.DirBuilder.html#method.recursive
    #[inline]
    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.std.recursive(recursive);
        self
    }
}

#[cfg(unix)]
impl std::os::unix::fs::DirBuilderExt for DirBuilder {
    #[inline]
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.std.mode(mode);
        self
    }
}

// TODO: impl Debug for DirBuilder? But don't expose DirBuilder's path...
