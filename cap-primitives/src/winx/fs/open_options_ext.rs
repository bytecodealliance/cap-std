#[derive(Debug, Clone)]
pub(crate) struct OpenOptionsExt {
    pub(super) access_mode: Option<u32>,
    pub(super) share_mode: u32,
    pub(super) custom_flags: u32,
    pub(super) attributes: u32,
    pub(super) security_qos_flags: u32,
}

impl OpenOptionsExt {
    pub(crate) fn new() -> Self {
        use winapi::um::winnt;
        Self {
            access_mode: None,
            share_mode: winnt::FILE_SHARE_READ | winnt::FILE_SHARE_WRITE | winnt::FILE_SHARE_DELETE,
            custom_flags: 0,
            attributes: 0,
            security_qos_flags: 0,
        }
    }
}

impl std::os::windows::fs::OpenOptionsExt for OpenOptionsExt {
    fn access_mode(&mut self, mode: u32) -> &mut Self {
        self.access_mode = Some(mode);
        self
    }

    fn share_mode(&mut self, share: u32) -> &mut Self {
        self.share_mode = share;
        self
    }

    fn custom_flags(&mut self, flags: u32) -> &mut Self {
        self.custom_flags = flags;
        self
    }

    fn attributes(&mut self, attributes: u32) -> &mut Self {
        self.attributes = attributes;
        self
    }

    /// Re-enable this once https://github.com/rust-lang/rust/pull/74074 is in stable.
    #[cfg(feature = "windows_security_qos_flags")]
    fn security_qos_flags(&mut self, flags: u32) -> &mut Self {
        self.security_qos_flags = flags;
        self
    }

    #[cfg(not(feature = "windows_security_qos_flags"))]
    fn security_qos_flags(&mut self, _flags: u32) -> &mut std::fs::OpenOptions {
        panic!("OpenOptionsExt::security_qos_flags requires the \"nightly\" feature")
    }
}
