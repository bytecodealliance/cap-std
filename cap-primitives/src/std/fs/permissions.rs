#[cfg(any(unix, target_os = "vxworks"))]
use crate::fs::PermissionsExt;
use std::fs;

/// Representation of the various permissions on a file.
///
/// This corresponds to [`std::fs::Permissions`].
///
/// [`std::fs::Permissions`]: https://doc.rust-lang.org/std/fs/struct.Permissions.html
///
/// <details>
/// We need to define our own version because the libstd `Permissions` doesn't have
/// a public constructor that we can use.
/// </details>
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Permissions {
    pub(crate) readonly: bool,

    #[cfg(any(unix, target_os = "vxworks"))]
    pub(crate) ext: PermissionsExt,
}

impl Permissions {
    /// Constructs a new instance of `Self` from the given `std::fs::Permissions`.
    #[inline]
    pub(crate) fn from_std(std: fs::Permissions) -> Self {
        Self {
            readonly: std.readonly(),

            #[cfg(any(unix, target_os = "vxworks"))]
            ext: PermissionsExt::from_std(std),
        }
    }

    /// Returns `true` if these permissions describe a readonly (unwritable) file.
    ///
    /// This corresponds to [`Permissions::readonly`].
    ///
    /// [`std::fs::Permissions::readonly`]: https://doc.rust-lang.org/std/fs/struct.Permissions.html#method.readonly
    #[inline]
    pub fn readonly(&self) -> bool {
        self.readonly
    }

    /// Modifies the readonly flag for this set of permissions.
    ///
    /// This corresponds to [`Permissions::set_readonly`].
    ///
    /// [`std::fs::Permissions::set_readonly`]: https://doc.rust-lang.org/std/fs/struct.Permissions.html#method.set_readonly
    #[inline]
    pub fn set_readonly(&mut self, readonly: bool) {
        self.readonly = readonly
    }
}

#[cfg(unix)]
impl std::os::unix::fs::PermissionsExt for Permissions {
    #[inline]
    fn mode(&self) -> u32 {
        self.ext.mode()
    }

    #[inline]
    fn set_mode(&mut self, mode: u32) {
        self.ext.set_mode(mode)
    }

    #[inline]
    fn from_mode(mode: u32) -> Self {
        Self {
            readonly: PermissionsExt::readonly(mode as libc::mode_t),
            ext: PermissionsExt::from_mode(mode),
        }
    }
}

#[cfg(target_os = "vxworks")]
impl std::os::unix::fs::PermissionsExt for Permissions {
    #[inline]
    fn mode(&self) -> u32 {
        self.ext.mode()
    }

    #[inline]
    fn set_mode(&mut self, mode: u32) {
        self.ext.set_mode(mode)
    }

    #[inline]
    fn from_mode(mode: u32) -> Self {
        Self {
            readonly: PermissionsExt::readonly(mode),
            ext: PermissionsExt::from(mode),
        }
    }
}
