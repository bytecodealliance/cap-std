use std::{borrow::Cow, ffi::OsStr, path::Component};

/// Like `std::path::Component` except we combine `Prefix` and `RootDir` since
/// we don't support absolute paths, and `Normal` has a `Cow` instead of a plain
/// `OsStr` reference, so it can optionally own its own string.
#[derive(Debug)]
pub(super) enum CowComponent<'borrow> {
    PrefixOrRootDir,
    CurDir,
    ParentDir,
    Normal(Cow<'borrow, OsStr>),
}

/// Convert a `Component` into a `CowComponent` which borrows strings.
pub(super) fn to_borrowed_component(component: Component) -> CowComponent {
    match component {
        Component::Prefix(_) | Component::RootDir => CowComponent::PrefixOrRootDir,
        Component::CurDir => CowComponent::CurDir,
        Component::ParentDir => CowComponent::ParentDir,
        Component::Normal(os_str) => CowComponent::Normal(os_str.into()),
    }
}

/// Convert a `Component` into a `CowComponent` which owns strings.
pub(super) fn to_owned_component<'borrow>(component: Component) -> CowComponent<'borrow> {
    match component {
        Component::Prefix(_) | Component::RootDir => CowComponent::PrefixOrRootDir,
        Component::CurDir => CowComponent::CurDir,
        Component::ParentDir => CowComponent::ParentDir,
        Component::Normal(os_str) => CowComponent::Normal(os_str.to_os_string().into()),
    }
}
