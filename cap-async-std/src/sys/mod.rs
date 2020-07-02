use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(unix, target_os = "fuchsia"))] {
        mod yanix;
        pub(crate) use self::yanix::*;
    } else if #[cfg(windows)] {
        mod winx;
        pub(crate) use self::winx::*;
    } else if #[cfg(target_os = "wasi")] {
        mod wasi;
        pub(crate) use self::wasi::*;
    } else {
        compile_error!("cap-async-std doesn't compile for this platform yet");
    }
}
