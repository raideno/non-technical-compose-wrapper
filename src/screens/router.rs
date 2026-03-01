use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Entry,
    Services,
}

pub struct NavigationEvent {
    pub from: Route,
    pub payload: NavigationPayload,
}

#[derive(Debug, Clone)]
pub enum NavigationPayload {
    /// Emitted by the Entry screen – contains the path to the chosen
    /// docker-compose file so the router can parse it before building the
    /// Services screen.
    OpenFile(PathBuf),

    /// No payload required (e.g. navigating back to Entry).
    None,
}
