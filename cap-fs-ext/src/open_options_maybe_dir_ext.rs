/// Extension trait for `cap_primitives::fs::OpenOptions` which adds
/// `maybe_dir`, a function for controlling whether an open should
/// attempt to succeed on a directory. On Posix-ish platforms, opening
/// a directory always succeeds, but on Windows, opening a directory
/// needs this option.
pub trait OpenOptionsMaybeDirExt {
    /// Sets the option for disabling an error that might be generated
    /// by the opened object being a directory.
    fn maybe_dir(&mut self, maybe_dir: bool) -> &mut Self;
}

impl OpenOptionsMaybeDirExt for cap_primitives::fs::OpenOptions {
    #[inline]
    fn maybe_dir(&mut self, maybe_dir: bool) -> &mut Self {
        // `maybe_dir` functionality is implemented within `cap_primitives`; we're
        // just exposing it here since `OpenOptions` is re-exported by `cap_std`
        // etc. and `maybe_dir` isn't in `std`.
        unsafe { self._cap_fs_ext_maybe_dir(maybe_dir) }
    }
}
