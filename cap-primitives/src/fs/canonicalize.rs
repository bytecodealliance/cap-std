//! Manual path canonicalization, one component at a time, with manual symlink
//! resolution, in order to enforce sandboxing.

use crate::fs::canonicalize_impl;
#[cfg(debug_assertions)]
use crate::fs::{get_path, is_same_file, open, OpenOptions};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Canonicalize the given path, ensuring that the resolution of the path never
/// escapes the directory tree rooted at `start`.
#[cfg_attr(not(debug_assertions), allow(clippy::let_and_return))]
#[inline]
pub fn canonicalize(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    // Call the underlying implementation.
    let result = canonicalize_impl(start, path);

    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    check_canonicalize(start, path, &result);

    result
}

#[cfg(debug_assertions)]
fn check_canonicalize(start: &fs::File, path: &Path, result: &io::Result<PathBuf>) {
    if let Ok(canonical_path) = result {
        let path_result = open(start, path, OpenOptions::new().read(true));
        let canonical_result = open(start, canonical_path, OpenOptions::new().read(true));
        match (path_result, canonical_result) {
            (Ok(path_file), Ok(canonical_file)) => {
                assert!(is_same_file(&path_file, &canonical_file).unwrap())
            }
            (Err(path_err), Err(canonical_err)) => {
                assert_eq!(path_err.to_string(), canonical_err.to_string())
            }
            other => panic!("inconsistent canonicalize checks: {:?}", other),
        }

        // On operating systems which can tell us the path of a file descriptor,
        // assert that the path we computed canonicalizes to the same thing as
        // the input canonicalizes too.
        if let Some(start_abspath) = get_path(start) {
            let check_abspath = start_abspath.join(path);
            let result_abspath = start_abspath.join(canonical_path);
            let check_abspath =
                fs::canonicalize(check_abspath).expect("we already canonicalized this");
            let result_abspath =
                fs::canonicalize(result_abspath).expect("we already canonicalized this");
            assert_eq!(
                check_abspath,
                result_abspath,
                "incorrect canonicalization: start='{:?}' path='{}' result='{}'",
                start,
                path.display(),
                canonical_path.display()
            );
            // TODO: When porting to Windows, check whether `start_abspath` not being
            // a canonicalized path leads to `\\?\` extended path prefix differences.
            assert!(
                result_abspath.starts_with(start_abspath),
                "sandbox escape: start='{:?}' path='{}' result='{}'",
                start,
                path.display(),
                canonical_path.display()
            );
        }
    }
}
