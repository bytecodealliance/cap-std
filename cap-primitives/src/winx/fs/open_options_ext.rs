#[derive(Debug, Clone)]
pub(crate) struct OpenOptionsExt {
    pub(crate) access_mode: u32,
    pub(crate) share_mode: u32,
    pub(crate) custom_flags: u32,
    pub(crate) attributes: u32,
    pub(crate) security_qos_flags: u32,
}

impl OpenOptionsExt {
    pub(crate) fn new() -> Self {
        use winapi::um::winnt;
        // TODO figure out the defaults
        Self {
            access_mode: 0,
            share_mode: winnt::FILE_SHARE_READ | winnt::FILE_SHARE_WRITE | winnt::FILE_SHARE_DELETE,
            custom_flags: 0,
            attributes: 0,
            security_qos_flags: 0,
        }
    }

    // TODO all of the below can safely be deleted when `OpenOptionsExt` trait
    // is fixed in upstream.
    // See #74074
    pub fn access_mode(&mut self, mode: u32) -> &mut Self {
        self.access_mode = mode;
        self
    }

    pub fn share_mode(&mut self, share: u32) -> &mut Self {
        self.share_mode = share;
        self
    }

    pub fn custom_flags(&mut self, flags: u32) -> &mut Self {
        self.custom_flags = flags;
        self
    }

    pub fn attributes(&mut self, attributes: u32) -> &mut Self {
        self.attributes = attributes;
        self
    }

    pub fn security_qos_flags(&mut self, flags: u32) -> &mut Self {
        self.security_qos_flags = flags;
        self
    }
}

// TODO re-enable when [#74074] is resolved.
// [#74074]: https://github.com/rust-lang/rust/pull/74074
// impl std::os::windows::fs::OpenOptionsExt for OpenOptionsExt {
//     fn access_mode(&mut self, mode: u32) -> &mut Self {
//         self.access_mode = mode;
//         self
//     }

//     fn share_mode(&mut self, share: u32) -> &mut Self {
//         self.share_mode = share;
//         self
//     }

//     fn custom_flags(&mut self, flags: u32) -> &mut Self {
//         self.custom_flags = flags;
//         self
//     }

//     fn attributes(&mut self, attributes: u32) -> &mut Self {
//         self.attributes = attributes;
//         self
//     }

//     fn security_qos_flags(&mut self, flags: u32) -> &mut Self {
//         self.security_qos_flags = flags;
//         self
//     }
// }
