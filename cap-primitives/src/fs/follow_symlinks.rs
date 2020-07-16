/// Instead of passing bare `bool`s as parameters, pass a distinct
/// enum so that the intent is clear.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FollowSymlinks {
    /// Yes, do follow symlinks.
    Yes,

    /// No, do not follow symlinks.
    No,
}
