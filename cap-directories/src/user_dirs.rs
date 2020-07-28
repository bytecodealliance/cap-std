use crate::not_found;
use cap_std::fs::Dir;
use std::io;

/// `UserDirs` provides paths of user-facing standard directories, following the
/// conventions of the operating system the library is running on.
///
/// This corresponds to [`directories::UserDirs`], except that the functions open the
/// directories and returns `Dir`s instead of returning `Path`s.
///
/// Unlike `directories::UserDirs`, the `*_dir` functions return `Dir`s rather than
/// `Path`s, because absolute paths don't interoperate well with the capability model.
///
/// [`directories::UserDirs`]: https://docs.rs/directories/latest/directories/struct.UserDirs.html
#[derive(Clone)]
pub struct UserDirs {
    inner: directories::UserDirs,
}

impl UserDirs {
    /// Creates a UserDirs struct which holds the paths to user-facing directories for audio, font,
    /// video, etc. data on the system.
    ///
    /// This corresponds to [`directories::UserDirs::new`].
    ///
    /// [`directories::UserDirs::new`]: https://docs.rs/directories/latest/directories/struct.UserDirs.html#method.new
    pub fn new() -> Option<Self> {
        let inner = directories::UserDirs::new()?;
        Some(Self { inner })
    }

    /// Returns the user's home directory.
    ///
    /// This corresponds to [`directories::UserDirs::home_dir`].
    ///
    /// [`directories::UserDirs::home_dir`]: https://docs.rs/directories/latest/directories/struct.UserDirs.html#method.home_dir
    pub fn home_dir(&self) -> io::Result<Dir> {
        unsafe { Dir::open_ambient_dir(self.inner.home_dir()) }
    }

    /// Returns the user's audio directory.
    ///
    /// This corresponds to [`directories::UserDirs::audio_dir`].
    ///
    /// [`directories::UserDirs::audio_dir`]: https://docs.rs/directories/latest/directories/struct.UserDirs.html#method.audio_dir
    pub fn audio_dir(&self) -> io::Result<Dir> {
        unsafe { Dir::open_ambient_dir(self.inner.audio_dir().ok_or_else(not_found)?) }
    }

    /// Returns the user's desktop directory.
    ///
    /// This corresponds to [`directories::UserDirs::desktop_dir`].
    ///
    /// [`directories::UserDirs::desktop_dir`]: https://docs.rs/directories/latest/directories/struct.UserDirs.html#method.desktop_dir
    pub fn desktop_dir(&self) -> io::Result<Dir> {
        unsafe { Dir::open_ambient_dir(self.inner.desktop_dir().ok_or_else(not_found)?) }
    }

    /// Returns the user's document directory.
    ///
    /// This corresponds to [`directories::UserDirs::document_dir`].
    ///
    /// [`directories::UserDirs::document_dir`]: https://docs.rs/directories/latest/directories/struct.UserDirs.html#method.document_dir
    pub fn document_dir(&self) -> io::Result<Dir> {
        unsafe { Dir::open_ambient_dir(self.inner.document_dir().ok_or_else(not_found)?) }
    }

    /// Returns the user's download directory.
    ///
    /// This corresponds to [`directories::UserDirs::download_dir`].
    ///
    /// [`directories::UserDirs::download_dir`]: https://docs.rs/directories/latest/directories/struct.UserDirs.html#method.download_dir
    pub fn download_dir(&self) -> io::Result<Dir> {
        unsafe { Dir::open_ambient_dir(self.inner.download_dir().ok_or_else(not_found)?) }
    }

    /// Returns the user's font directory.
    ///
    /// This corresponds to [`directories::UserDirs::font_dir`].
    ///
    /// [`directories::UserDirs::font_dir`]: https://docs.rs/directories/latest/directories/struct.UserDirs.html#method.font_dir
    pub fn font_dir(&self) -> io::Result<Dir> {
        unsafe { Dir::open_ambient_dir(self.inner.font_dir().ok_or_else(not_found)?) }
    }

    /// Returns the user's picture directory.
    ///
    /// This corresponds to [`directories::UserDirs::picture_dir`].
    ///
    /// [`directories::UserDirs::picture_dir`]: https://docs.rs/directories/latest/directories/struct.UserDirs.html#method.picture_dir
    pub fn picture_dir(&self) -> io::Result<Dir> {
        unsafe { Dir::open_ambient_dir(self.inner.picture_dir().ok_or_else(not_found)?) }
    }

    /// Returns the user's public directory.
    ///
    /// This corresponds to [`directories::UserDirs::public_dir`].
    ///
    /// [`directories::UserDirs::public_dir`]: https://docs.rs/directories/latest/directories/struct.UserDirs.html#method.public_dir
    pub fn public_dir(&self) -> io::Result<Dir> {
        unsafe { Dir::open_ambient_dir(self.inner.public_dir().ok_or_else(not_found)?) }
    }

    /// Returns the user's template directory.
    ///
    /// This corresponds to [`directories::UserDirs::template_dir`].
    ///
    /// [`directories::UserDirs::template_dir`]: https://docs.rs/directories/latest/directories/struct.UserDirs.html#method.template_dir
    pub fn template_dir(&self) -> io::Result<Dir> {
        unsafe { Dir::open_ambient_dir(self.inner.template_dir().ok_or_else(not_found)?) }
    }

    /// Returns the user's video directory.
    ///
    /// This corresponds to [`directories::UserDirs::video_dir`].
    ///
    /// [`directories::UserDirs::video_dir`]: https://docs.rs/directories/latest/directories/struct.UserDirs.html#method.video_dir
    pub fn video_dir(&self) -> io::Result<Dir> {
        unsafe { Dir::open_ambient_dir(self.inner.video_dir().ok_or_else(not_found)?) }
    }
}
