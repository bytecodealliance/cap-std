//! This defines `mkdir`, the primary entrypoint to sandboxed directory creation.

#[cfg(not(feature = "no_racy_asserts"))]
use crate::fs::{canonicalize, mkdir_unchecked, stat_unchecked, FollowSymlinks, Metadata};
use crate::fs::{mkdir_impl, DirOptions};
use std::{fs, io, path::Path};

/// Perform a `mkdirat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(feature = "no_racy_asserts", allow(clippy::let_and_return))]
#[inline]
pub fn mkdir(start: &fs::File, path: &Path, options: &DirOptions) -> io::Result<()> {
    #[cfg(not(feature = "no_racy_asserts"))]
    let stat_before = stat_unchecked(start, path, FollowSymlinks::No);

    // Call the underlying implementation.
    let result = mkdir_impl(start, path, options);

    #[cfg(not(feature = "no_racy_asserts"))]
    let stat_after = stat_unchecked(start, path, FollowSymlinks::No);

    #[cfg(not(feature = "no_racy_asserts"))]
    check_mkdir(start, path, options, &stat_before, &result, &stat_after);

    result
}

#[cfg(not(feature = "no_racy_asserts"))]
#[allow(clippy::enum_glob_use)]
fn check_mkdir(
    start: &fs::File,
    path: &Path,
    options: &DirOptions,
    stat_before: &io::Result<Metadata>,
    result: &io::Result<()>,
    stat_after: &io::Result<Metadata>,
) {
    use super::map_result;
    use io::ErrorKind::*;

    match (
        map_result(stat_before),
        map_result(result),
        map_result(stat_after),
    ) {
        (Err((NotFound, _)), Ok(()), Ok(metadata)) => {
            assert!(metadata.is_dir());
            assert!(stat_unchecked(
                start,
                &canonicalize(start, path).unwrap(),
                FollowSymlinks::No
            )
            .unwrap()
            .is_same_file(&metadata));
        }

        (Ok(metadata_before), Err((AlreadyExists, _)), Ok(metadata_after)) => {
            assert!(metadata_before.is_same_file(&metadata_after));
        }

        (_, Err((kind, message)), _) => match map_result(&canonicalize(start, path)) {
            Ok(canon) => match map_result(&mkdir_unchecked(start, &canon, options)) {
                Err((unchecked_kind, unchecked_message)) => {
                    assert_eq!(
                        kind,
                        unchecked_kind,
                        "unexpected error kind from mkdir start='{:?}', \
                         path='{}':\nstat_before={:#?}\nresult={:#?}\nstat_after={:#?}",
                        start,
                        path.display(),
                        stat_before,
                        result,
                        stat_after
                    );
                    assert_eq!(message, unchecked_message);
                }
                _ => panic!("unsandboxed mkdir success"),
            },
            Err((_canon_kind, _canon_message)) => {
                /* TODO: Check error messages
                assert_eq!(kind, canon_kind);
                assert_eq!(message, canon_message);
                */
            }
        },

        other => panic!(
            "inconsistent mkdir checks: start='{:?}' path='{}':\n{:#?}",
            start,
            path.display(),
            other,
        ),
    }
}
