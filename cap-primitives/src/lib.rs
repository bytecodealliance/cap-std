//! Capability-oriented primitives.
//!
//! TODO: Is it worth defining our own `Path` and `PathBuf` types? Rust's `Path`
//! has several ambient-authority methods: `metadata`, `read_link`, `read_dir`,
//! `symlink_metadata`, `canonicalize`. If we had our own `Path`, we could also
//! handle absolute paths specially too.

#![deny(missing_docs)]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(target_os = "windows", feature(windows_by_handle))]
#![cfg_attr(target_os = "windows", feature(windows_file_type_ext))]

use cfg_if::cfg_if;

#[cfg(not(target_os = "wasi"))]
mod std;

cfg_if! {
    if #[cfg(any(unix, target_os = "fuchsia"))] {
        mod yanix;
    } else if #[cfg(windows)] {
        mod winx;
    } else if #[cfg(not(target_os = "wasi"))] {
        compile_error!("cap-std doesn't compile for this platform yet");
    }
}

/// Capability-oriented filesystem manipulation operations.
pub mod fs {
    use cfg_if::cfg_if;

    cfg_if! {
        if #[cfg(any(unix, target_os = "fuchsia"))] {
            #[allow(unused_imports)]
            pub use super::yanix::fs::*;
        } else if #[cfg(windows)] {
            #[allow(unused_imports)]
            pub use super::winx::fs::*;
        } else if #[cfg(not(target_os = "wasi"))] {
            compile_error!("cap-std doesn't compile for this platform yet");
        }
    }

    #[cfg(not(target_os = "wasi"))]
    pub use super::std::fs::*;
}

/// Capability-oriented networking primitives for TCP/UDP communication.
pub mod net {
    #[cfg(not(target_os = "wasi"))]
    pub use super::std::net::*;
}
