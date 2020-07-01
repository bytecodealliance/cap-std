#[derive(Debug, Clone)]
pub(crate) struct OpenOptionsExt {
    pub(crate) mode: libc::mode_t,
    pub(crate) custom_flags: i32,
}

impl OpenOptionsExt {
    pub(crate) fn new() -> Self {
        Self {
            mode: 0o666,
            custom_flags: 0,
        }
    }

    pub(crate) fn mode(&mut self, mode: libc::mode_t) -> &mut Self {
        self.mode = mode;
        self
    }

    pub(crate) fn custom_flags(&mut self, flags: i32) -> &mut Self {
        self.custom_flags = flags;
        self
    }
}
