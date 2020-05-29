use std::{io, path::Path};

/// A builder used to create directories in various manners.
///
/// This corresponds to [`std::fs::DirBuilder`].
///
/// [`std::fs::DirBuilder`]: https://doc.rust-lang.org/std/fs/struct.DirBuilder.html
pub struct DirBuilder {}

impl DirBuilder {
    /// Creates a new set of options with default mode/security settings for all platforms and also non-recursive.
    ///
    /// This corresponds to [`std::fs::DirBuilder::new`].
    ///
    /// [`std::fs::DirBuilder::new`]: https://doc.rust-lang.org/std/fs/struct.DirBuilder.html#method.new
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        unimplemented!("DirBuilder::new");
    }

    /// Indicates that directories should be created recursively, creating all parent directories.
    ///
    /// This corresponds to [`std::fs::DirBuilder::recursive`].
    ///
    /// [`std::fs::DirBuilder::recursive`]: https://doc.rust-lang.org/std/fs/struct.DirBuilder.html#method.recursive
    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        unimplemented!("DirBuilder::recursive");
    }

    /// Creates the specified directory with the options configured in this builder.
    ///
    /// This corresponds to [`std::fs::DirBuilder::create`].
    ///
    /// [`std::fs::DirBuilder::create`]: https://doc.rust-lang.org/std/fs/struct.DirBuilder.html#method.create
    pub fn create<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        unimplemented!("DirBuilder::create");
    }
}

// TODO: functions from DirBuilderExt?

// TODO: impl Debug for DirBuilder? But don't expose DirBuilder's path...
