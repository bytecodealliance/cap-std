//! Manual path canonicalization, one component at a time, with manual symlink
//! resolution, in order to enforce sandboxing.

use crate::fs::canonicalize_impl;
#[cfg(debug_assertions)]
use crate::fs::{is_same_file, open, OpenOptions};
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

    #[cfg(debug_assertions)]
    if let Ok(canonical_path) = &result {
        let path_result = open(start, path, OpenOptions::new().read(true));
        let canonical_result = open(start, canonical_path, OpenOptions::new().read(true));
        match (path_result, canonical_result) {
            (Ok(path_file), Ok(canonical_file)) => {
                assert!(is_same_file(&path_file, &canonical_file)?)
            }
            (Err(path_err), Err(canonical_err)) => {
                assert_eq!(path_err.to_string(), canonical_err.to_string())
            }
            other => panic!("Inconsistent canonicalize checks: {:?}", other),
        }
    }

    result
}
