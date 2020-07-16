use crate::fs::OpenOptions;

/// Instead of passing bare `bool`s as parameters, pass a distinct
/// enum so that the intent is clear.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum FollowSymlinks {
    /// Yes, do follow symlinks.
    Yes,

    /// No, do not follow symlinks.
    No,
}

impl FollowSymlinks {
    /// Convert a bool where true means "follow" and false means "don't follow"
    /// to a `FollowSymlinks`.
    pub fn follow(follow: bool) -> Self {
        if follow {
            Self::Yes
        } else {
            Self::No
        }
    }

    /// Set the `nofollow` setting in the given `OpenOptions` and return it.
    pub fn options<'options>(
        &self,
        options: &'options mut OpenOptions,
    ) -> &'options mut OpenOptions {
        options.nofollow(match self {
            Self::Yes => false,
            Self::No => true,
        })
    }
}
