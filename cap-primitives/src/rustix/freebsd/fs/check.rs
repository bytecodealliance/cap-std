use rustix::fs::{statat, AtFlags};
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

static WORKING: AtomicBool = AtomicBool::new(false);
static CHECKED: AtomicBool = AtomicBool::new(false);

#[inline]
pub(crate) fn beneath_supported(start: &fs::File) -> bool {
    if WORKING.load(Relaxed) {
        return true;
    }
    if CHECKED.load(Relaxed) {
        return false;
    }
    // Unknown O_ flags get ignored but AT_ flags have strict checks, so we use that.
    if let Err(rustix::io::Errno::INVAL) =
        statat(start, "", AtFlags::EMPTY_PATH | AtFlags::RESOLVE_BENEATH)
    {
        CHECKED.store(true, Relaxed);
        false
    } else {
        WORKING.store(true, Relaxed);
        true
    }
}
