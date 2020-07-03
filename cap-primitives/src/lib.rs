//! Capability-oriented primitives.

#![deny(missing_docs)]

use cfg_if::cfg_if;

#[cfg(target_os = "linux")]
#[macro_use]
extern crate lazy_static;

mod std;

cfg_if! {
    if #[cfg(any(unix, target_os = "wasi", target_os = "fuchsia"))] {
        mod yanix;
    } else if #[cfg(windows)] {
        mod winx;
    } else {
        compile_error!("cap-std doesn't compile for this platform yet");
    }
}

/// Capability-oriented filesystem manipulation operations.
pub mod fs {
    use cfg_if::cfg_if;

    cfg_if! {
        if #[cfg(any(unix, target_os = "wasi", target_os = "fuchsia"))] {
            #[allow(unused_imports)]
            pub use super::yanix::fs::*;
        } else if #[cfg(windows)] {
            #[allow(unused_imports)]
            pub use super::winx::fs::*;
        } else {
            compile_error!("cap-std doesn't compile for this platform yet");
        }
    }

    pub use super::std::fs::*;
}

/// Capability-oriented networking primitives for TCP/UDP communication.
pub mod net {
    pub use super::std::net::*;
}
