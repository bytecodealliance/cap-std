use crate::fs::Permissions;
use std::fs;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct PermissionsExt {
    mode: libc::mode_t,
}

impl PermissionsExt {
    /// Constructs a new instance of `Self` from the given `std::fs::Permissions`.
    #[inline]
    pub(crate) fn from_std(std: fs::Permissions) -> Self {
        use std::os::unix::fs::PermissionsExt;
        Self {
            mode: std.mode() as libc::mode_t,
        }
    }

    /// Constructs a new instance of `FileType` from the given `libc::mode_t`.
    #[inline]
    pub(crate) const fn from_libc(mode: libc::mode_t) -> Permissions {
        Permissions {
            readonly: Self::readonly(mode),
            ext: Self { mode: mode & 0o777 },
        }
    }

    /// Test whether the given `libc::mode_t` lacks write permissions.
    #[inline]
    pub(crate) const fn readonly(mode: libc::mode_t) -> bool {
        mode & 0o222 == 0
    }
}

impl std::os::unix::fs::PermissionsExt for PermissionsExt {
    fn mode(&self) -> u32 {
        self.mode as u32
    }

    fn set_mode(&mut self, mode: u32) {
        self.mode = mode as libc::mode_t
    }

    fn from_mode(mode: u32) -> Self {
        Self {
            mode: mode as libc::mode_t,
        }
    }
}
