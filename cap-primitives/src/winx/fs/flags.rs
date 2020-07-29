use crate::fs::{FollowSymlinks, OpenOptions};
use std::{fs, os::windows::fs::OpenOptionsExt};
use winx::file::Flags;

pub(super) fn open_options_to_std(opts: &OpenOptions) -> fs::OpenOptions {
    let custom_flags = match opts.follow {
        FollowSymlinks::Yes => opts.ext.custom_flags,
        FollowSymlinks::No => opts.ext.custom_flags | Flags::FILE_FLAG_OPEN_REPARSE_POINT.bits(),
    };
    let mut std_opts = fs::OpenOptions::new();
    std_opts
        .read(opts.read)
        .write(opts.write)
        .append(opts.append)
        .truncate(opts.truncate)
        .create(opts.create)
        .create_new(opts.create_new)
        .share_mode(opts.ext.share_mode)
        .custom_flags(custom_flags)
        .attributes(opts.ext.attributes)
        .security_qos_flags(opts.ext.security_qos_flags);

    if let Some(access_mode) = opts.ext.access_mode {
        std_opts.access_mode(access_mode);
    }

    std_opts
}
