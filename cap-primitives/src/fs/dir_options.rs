#[cfg(any(unix, target_os = "vxworks"))]
use crate::fs::DirOptionsExt;

/// Options and flags which can be used to configure how a directory is created.
///
/// This is to `create_dir` what to `OpenOptions` is to `open`.
#[derive(Debug, Clone)]
pub struct DirOptions {
    #[cfg(any(unix, target_os = "vxworks"))]
    pub(crate) ext: DirOptionsExt,
}

impl DirOptions {
    /// Creates a blank new set of options ready for configuration.
    #[allow(clippy::new_without_default)]
    #[inline]
    pub const fn new() -> Self {
        Self {
            #[cfg(any(unix, target_os = "vxworks"))]
            ext: DirOptionsExt::new(),
        }
    }
}

#[cfg(unix)]
impl std::os::unix::fs::DirBuilderExt for DirOptions {
    #[inline]
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.ext.mode(mode);
        self
    }
}

#[cfg(target_os = "vxworks")]
impl std::os::vxworks::fs::DirBuilderExt for DirOptions {
    #[inline]
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.ext.mode(mode);
        self
    }
}

#[cfg(feature = "arbitrary")]
impl arbitrary::Arbitrary for DirOptions {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        #[cfg(any(unix, target_os = "vxworks"))]
        use std::os::unix::fs::DirBuilderExt;

        let mut dir_options = Self::new();

        #[cfg(any(unix, target_os = "vxworks"))]
        dir_options.mode(u.int_in_range(0..=0o777)?);

        Ok(dir_options)
    }
}
