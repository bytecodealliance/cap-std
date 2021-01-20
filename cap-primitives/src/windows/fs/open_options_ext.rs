use winapi::um::{winbase, winnt};

#[derive(Debug, Clone)]
pub(crate) struct OpenOptionsExt {
    pub(super) access_mode: Option<u32>,
    pub(super) share_mode: u32,
    pub(super) custom_flags: u32,
    pub(super) attributes: u32,
    pub(super) security_qos_flags: u32,
}

impl OpenOptionsExt {
    pub(crate) const fn new() -> Self {
        Self {
            access_mode: None,
            share_mode: winnt::FILE_SHARE_READ | winnt::FILE_SHARE_WRITE | winnt::FILE_SHARE_DELETE,
            custom_flags: 0,
            attributes: 0,
            security_qos_flags: 0,
        }
    }

    pub(crate) fn access_mode(&mut self, mode: u32) -> &mut Self {
        self.access_mode = Some(mode);
        self
    }

    pub(crate) fn share_mode(&mut self, share: u32) -> &mut Self {
        self.share_mode = share;
        self
    }

    pub(crate) fn custom_flags(&mut self, flags: u32) -> &mut Self {
        self.custom_flags = flags;
        self
    }

    pub(crate) fn attributes(&mut self, attributes: u32) -> &mut Self {
        self.attributes = attributes;
        self
    }

    pub(crate) fn security_qos_flags(&mut self, flags: u32) -> &mut Self {
        self.security_qos_flags = flags | winbase::SECURITY_SQOS_PRESENT;
        self
    }
}
