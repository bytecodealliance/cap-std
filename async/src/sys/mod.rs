use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(unix, target_os = "wasi", target_os = "fuchsia"))] {
        mod yanix;
        pub(crate) use self::yanix::*;
    } else if #[cfg(windows)] {
        mod winx;
        pub(crate) use self::winx::*;
    } else {
        compile_error!("cap-async-std doesn't compile for this platform yet");
    }
}

// For now, assume all platforms use the std implementation.
mod std;
pub(crate) use self::std::*;
