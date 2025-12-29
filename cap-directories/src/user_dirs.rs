use crate::not_found;
use cap_std::fs::Dir;
use cap_std::AmbientAuthority;
use std::io;

/// `UserDirs` provides paths of user-facing standard directories, following
/// the conventions of the operating system the library is running on.
///
/// This corresponds to [`directories::UserDirs`], except that the
/// functions open the directories and returns `Dir`s instead of returning
/// `Path`s.
///
/// Unlike `directories::UserDirs`, the `*_dir` functions return `Dir`s
/// rather than `Path`s, because absolute paths don't interoperate well with
/// the capability model.
#[derive(Clone)]
pub struct UserDirs {
    inner: directories::UserDirs,
}

impl UserDirs {
    /// Creates a `UserDirs` struct which holds the paths to user-facing
    /// directories for audio, font, video, etc. data on the system.
    ///
    /// This corresponds to [`directories::UserDirs::new`].
    pub fn new() -> Option<Self> {
        let inner = directories::UserDirs::new()?;
        Some(Self { inner })
    }

    /// Returns the user's home directory.
    ///
    /// This corresponds to [`directories::UserDirs::home_dir`].
    ///
    /// # Ambient Authority
    ///
    /// This function makes use of ambient authority to access the user
    /// directories.
    pub fn home_dir(&self, ambient_authority: AmbientAuthority) -> io::Result<Dir> {
        Dir::open_ambient_dir(self.inner.home_dir(), ambient_authority)
    }

    /// Returns the user's audio directory.
    ///
    /// This corresponds to [`directories::UserDirs::audio_dir`].
    ///
    /// # Ambient Authority
    ///
    /// This function makes use of ambient authority to access the user
    /// directories.
    pub fn audio_dir(&self, ambient_authority: AmbientAuthority) -> io::Result<Dir> {
        Dir::open_ambient_dir(
            self.inner.audio_dir().ok_or_else(not_found)?,
            ambient_authority,
        )
    }

    /// Returns the user's desktop directory.
    ///
    /// This corresponds to [`directories::UserDirs::desktop_dir`].
    ///
    /// # Ambient Authority
    ///
    /// This function makes use of ambient authority to access the user
    /// directories.
    pub fn desktop_dir(&self, ambient_authority: AmbientAuthority) -> io::Result<Dir> {
        Dir::open_ambient_dir(
            self.inner.desktop_dir().ok_or_else(not_found)?,
            ambient_authority,
        )
    }

    /// Returns the user's document directory.
    ///
    /// This corresponds to [`directories::UserDirs::document_dir`].
    ///
    /// # Ambient Authority
    ///
    /// This function makes use of ambient authority to access the user
    /// directories.
    pub fn document_dir(&self, ambient_authority: AmbientAuthority) -> io::Result<Dir> {
        Dir::open_ambient_dir(
            self.inner.document_dir().ok_or_else(not_found)?,
            ambient_authority,
        )
    }

    /// Returns the user's download directory.
    ///
    /// This corresponds to [`directories::UserDirs::download_dir`].
    ///
    /// # Ambient Authority
    ///
    /// This function makes use of ambient authority to access the user
    /// directories.
    pub fn download_dir(&self, ambient_authority: AmbientAuthority) -> io::Result<Dir> {
        Dir::open_ambient_dir(
            self.inner.download_dir().ok_or_else(not_found)?,
            ambient_authority,
        )
    }

    /// Returns the user's font directory.
    ///
    /// This corresponds to [`directories::UserDirs::font_dir`].
    ///
    /// # Ambient Authority
    ///
    /// This function makes use of ambient authority to access the user
    /// directories.
    pub fn font_dir(&self, ambient_authority: AmbientAuthority) -> io::Result<Dir> {
        Dir::open_ambient_dir(
            self.inner.font_dir().ok_or_else(not_found)?,
            ambient_authority,
        )
    }

    /// Returns the user's picture directory.
    ///
    /// This corresponds to [`directories::UserDirs::picture_dir`].
    ///
    /// # Ambient Authority
    ///
    /// This function makes use of ambient authority to access the user
    /// directories.
    pub fn picture_dir(&self, ambient_authority: AmbientAuthority) -> io::Result<Dir> {
        Dir::open_ambient_dir(
            self.inner.picture_dir().ok_or_else(not_found)?,
            ambient_authority,
        )
    }

    /// Returns the user's public directory.
    ///
    /// This corresponds to [`directories::UserDirs::public_dir`].
    ///
    /// # Ambient Authority
    ///
    /// This function makes use of ambient authority to access the user
    /// directories.
    pub fn public_dir(&self, ambient_authority: AmbientAuthority) -> io::Result<Dir> {
        Dir::open_ambient_dir(
            self.inner.public_dir().ok_or_else(not_found)?,
            ambient_authority,
        )
    }

    /// Returns the user's template directory.
    ///
    /// This corresponds to [`directories::UserDirs::template_dir`].
    ///
    /// # Ambient Authority
    ///
    /// This function makes use of ambient authority to access the user
    /// directories.
    pub fn template_dir(&self, ambient_authority: AmbientAuthority) -> io::Result<Dir> {
        Dir::open_ambient_dir(
            self.inner.template_dir().ok_or_else(not_found)?,
            ambient_authority,
        )
    }

    /// Returns the user's video directory.
    ///
    /// This corresponds to [`directories::UserDirs::video_dir`].
    ///
    /// # Ambient Authority
    ///
    /// This function makes use of ambient authority to access the user
    /// directories.
    pub fn video_dir(&self, ambient_authority: AmbientAuthority) -> io::Result<Dir> {
        Dir::open_ambient_dir(
            self.inner.video_dir().ok_or_else(not_found)?,
            ambient_authority,
        )
    }
}
