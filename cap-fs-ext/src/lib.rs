//! Extension traits for `Dir`

#![deny(missing_docs)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.ico"
)]

mod dir_ext;
mod metadata_ext;
mod open_options_follow_ext;

#[cfg(all(any(feature = "std", feature = "async_std"), feature = "fs_utf8"))]
pub use dir_ext::DirExtUtf8;
pub use dir_ext::{DirExt, SystemTimeSpec};
pub use metadata_ext::MetadataExt;
pub use open_options_follow_ext::{FollowSymlinks, OpenOptionsFollowExt};
