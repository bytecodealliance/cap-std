use super::Dir;
use std::{
    io,
    path::{Component, Path, PathBuf},
};

#[derive(Debug, thiserror::Error)]
pub(crate) enum PathNormalizerError {
    #[error("attempt at escaping beyond the root Dir handle")]
    EscapeAttempt,
    #[error("I/O error: {err}")]
    IoError { err: io::Error },
    #[error("path not found: {path}")]
    NotFound { path: PathBuf },
}

impl From<PathNormalizerError> for io::Error {
    fn from(err: PathNormalizerError) -> Self {
        match err {
            PathNormalizerError::EscapeAttempt => Self::new(io::ErrorKind::PermissionDenied, err),
            PathNormalizerError::IoError { err } => err,
            PathNormalizerError::NotFound { .. } => Self::new(io::ErrorKind::NotFound, err),
        }
    }
}

pub(crate) struct PathNormalizer {
    explored: Vec<Dir>,
    remaining: Vec<PathBuf>,
    normalized_path: PathBuf,
}

impl PathNormalizer {
    pub fn new<P: AsRef<Path>>(root: Dir, path: P) -> Self {
        Self {
            explored: vec![root],
            remaining: vec![path.as_ref().to_owned()],
            normalized_path: PathBuf::new(),
        }
    }

    pub fn advance(&mut self) -> Option<Result<(), PathNormalizerError>> {
        let path = match self.remaining.pop() {
            Some(path) => path,
            None => return None,
        };

        let mut components = path.components();
        let head = match components.next() {
            Some(head) => head,
            None => return None,
        };

        let tail = components.as_path();
        if let Some(_) = tail.components().next() {
            self.remaining.push(tail.to_owned());
        }

        let path = match head {
            Component::Prefix(_) | Component::RootDir => {
                return Some(Err(PathNormalizerError::EscapeAttempt));
            }
            Component::CurDir => {
                // skip
                return self.advance();
            }
            Component::ParentDir => {
                if self.explored.len() == 1 {
                    return Some(Err(PathNormalizerError::EscapeAttempt));
                }
                self.explored.pop().expect("popping root Dir is a bug!");
                self.normalized_path.pop();
                // otherwise, we're all good, continue
                return self.advance();
            }
            Component::Normal(head) => PathBuf::from(head),
        };

        let this_dir = self
            .explored
            .last()
            .expect("empty explored Dir handles is a bug!");
        // try opening as dir
        let sub_dir = match this_dir.open_dir(&path) {
            Ok(dir) => dir,
            Err(err) if this_dir.is_file(&path) && self.remaining.is_empty() => {
                // we've hit the end of path which turns out to be a file,
                // so all good!
                self.normalized_path.push(path);
                return Some(Ok(()));
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                // if the dir doesn't exist, we exit signalling the reason
                let out_dir = match this_dir.try_clone() {
                    Ok(dir) => dir,
                    Err(err) => return Some(Err(PathNormalizerError::IoError { err })),
                };
                return Some(Err(PathNormalizerError::NotFound {
                    path: self.normalized_path.join(path),
                }));
            }
            Err(err) => {
                // otherwise, we try expanding link, if that fails, we throw original
                // error back
                let expanded = match this_dir.read_link(path) {
                    Ok(expanded) => expanded,
                    Err(_) => return Some(Err(PathNormalizerError::IoError { err })),
                };
                self.remaining.push(expanded);
                return self.advance();
            }
        };

        self.explored.push(sub_dir);
        self.normalized_path.push(path);
        Some(Ok(()))
    }

    pub fn leftover_path(&self) -> Option<&Path> {
        self.remaining.last().map(Path::new)
    }

    pub fn last_valid_dir(&self) -> &Dir {
        self.explored
            .last()
            .expect("empty explored Dir handles is a bug!")
    }

    pub fn last_valid_path(&self) -> &Path {
        &self.normalized_path
    }
}
