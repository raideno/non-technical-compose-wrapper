use std::path::PathBuf;

/// The screen the navigation is originating from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Entry,
    Services,
}

/// Carries everything the router needs to transition between screens.
pub struct NavigationEvent {
    pub from: Route,
    pub payload: NavigationPayload,
}

/// Screen-specific data that travels with a navigation event.
#[derive(Debug, Clone)]
pub enum NavigationPayload {
    /// Emitted by the Entry screen – contains the path to the chosen
    /// docker-compose file so the router can parse it before building the
    /// Services screen.
    OpenFile(PathBuf),

    /// No payload required (e.g. navigating back to Entry).
    None,
}
