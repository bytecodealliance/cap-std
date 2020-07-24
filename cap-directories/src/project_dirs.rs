use crate::not_found;
use cap_std::fs::Dir;
use std::{fs, io};

/// `ProjectDirs` computes the cache, config or data directories for a specific
/// application, which are derived from the standard directories and the name of the
/// project/organization.
///
/// This corresponds to [`directories::ProjectDirs`], except that the functions
/// create the directories if they don't exist, open them, and return `Dir`s
/// instead of returning `Path`s.
///
/// Unlike `directories::ProjectDirs`, this API has no `ProjectDirs::from_path`,
/// `ProjectDirs::path` or `ProjectDirs::project_path`, and the `*_dir` functions return
/// `Dir`s rather than `Path`s, because absolute paths don't interoperate well with the
/// capability model.
///
/// [`directories::ProjectDirs`]: https://docs.rs/directories/latest/directories/struct.ProjectDirs.html
#[derive(Clone)]
pub struct ProjectDirs {
    inner: directories::ProjectDirs,
}

impl ProjectDirs {
    /// Creates a `ProjectDirs` struct from values describing the project.
    ///
    /// This corresponds to [`directories::ProjectDirs::from`].
    ///
    /// [`directories::ProjectDirs::from`]: https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.from
    pub fn from(qualifier: &str, organization: &str, application: &str) -> Option<Self> {
        let inner = directories::ProjectDirs::from(qualifier, organization, application)?;
        Some(Self { inner })
    }

    /// Returns the project's cache directory.
    ///
    /// This corresponds to [`directories::ProjectDirs::cache_dir`].
    ///
    /// [`directories::ProjectDirs::cache_dir`]: https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.cache_dir
    pub fn cache_dir(&self) -> io::Result<Dir> {
        let path = self.inner.cache_dir();
        fs::create_dir_all(path)?;
        fs::File::open(path).map(Dir::from_std_file)
    }

    /// Returns the project's config directory.
    ///
    /// This corresponds to [`directories::ProjectDirs::config_dir`].
    ///
    /// [`directories::ProjectDirs::config_dir`]: https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.config_dir
    pub fn config_dir(&self) -> io::Result<Dir> {
        let path = self.inner.config_dir();
        fs::create_dir_all(path)?;
        fs::File::open(path).map(Dir::from_std_file)
    }

    /// Returns the project's data directory.
    ///
    /// This corresponds to [`directories::ProjectDirs::data_dir`].
    ///
    /// [`directories::ProjectDirs::data_dir`]: https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.data_dir
    pub fn data_dir(&self) -> io::Result<Dir> {
        let path = self.inner.data_dir();
        fs::create_dir_all(path)?;
        fs::File::open(path).map(Dir::from_std_file)
    }

    /// Returns the project's local data directory.
    ///
    /// This corresponds to [`directories::ProjectDirs::data_local_dir`].
    ///
    /// [`directories::ProjectDirs::data_local_dir`]: https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.data_local_dir
    pub fn data_local_dir(&self) -> io::Result<Dir> {
        let path = self.inner.data_local_dir();
        fs::create_dir_all(path)?;
        fs::File::open(path).map(Dir::from_std_file)
    }

    /// Returns the project's preference directory.
    ///
    /// This corresponds to [`directories::ProjectDirs::preference_dir`].
    ///
    /// [`directories::ProjectDirs::preference_dir`]: https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.preference_dir
    pub fn preference_dir(&self) -> io::Result<Dir> {
        let path = self.inner.preference_dir();
        fs::create_dir_all(path)?;
        fs::File::open(path).map(Dir::from_std_file)
    }

    /// Returns the project's runtime directory.
    ///
    /// This corresponds to [`directories::ProjectDirs::runtime_dir`].
    ///
    /// [`directories::ProjectDirs::runtime_dir`]: https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.runtime_dir
    pub fn runtime_dir(&self) -> io::Result<Dir> {
        let path = self.inner.runtime_dir().ok_or_else(not_found)?;
        fs::create_dir_all(path)?;
        fs::File::open(path).map(Dir::from_std_file)
    }
}
