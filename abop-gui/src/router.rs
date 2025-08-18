//! Application router for view navigation

use iced::Task;
use serde::{Deserialize, Serialize};

use crate::messages::Message;

/// Represents the different views in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Route {
    /// Library view - shows the audiobook library
    Library,
    /// Player view - shows the current playback
    Player,
    /// Settings view - application settings
    Settings,
    /// About view - application information
    About,
}

impl Default for Route {
    fn default() -> Self {
        Self::Library
    }
}

/// Manages application navigation state
#[derive(Debug, Default)]
pub struct Router {
    current_route: Route,
    history: Vec<Route>,
}

impl Router {
    /// Maximum number of routes to keep in history
    const MAX_HISTORY: usize = 64;
    /// Creates a new router with the default route
    pub fn new() -> Self {
        Self {
            current_route: Route::default(),
            history: vec![Route::default()],
        }
    }

    /// Returns the current route
    pub fn current_route(&self) -> Route {
        self.current_route
    }

    /// Navigates to the specified route
    ///
    /// # Behavior
    /// - Adds the new route to navigation history
    /// - Updates the current route
    /// - Returns `Task::none()` as navigation is a synchronous state change
    ///
    /// # Arguments
    /// * `route` - The target route to navigate to
    pub fn navigate_to(&mut self, route: Route) -> Task<Message> {
        // De-duplicate consecutive routes
        if self.history.last().copied() == Some(route) {
            self.current_route = route;
            return Task::none();
        }

        self.history.push(route);
        // Cap history length by trimming the oldest entry
        if self.history.len() > Self::MAX_HISTORY {
            let overflow = self.history.len() - Self::MAX_HISTORY;
            self.history.drain(0..overflow);
        }
        self.current_route = route;
        Task::none()
    }

    /// Navigates back to the previous route if available
    ///
    /// # Behavior
    /// - Pops the current route from history if history has more than one entry
    /// - Sets current route to the last item in history
    /// - Does nothing if already at the first route in history
    ///
    /// # Returns
    /// A no-op Task since navigation only updates internal state synchronously
    pub fn navigate_back(&mut self) -> Task<Message> {
        if self.history.len() > 1 {
            self.history.pop();
            // Safe: we verified history.len() > 1, so after pop() it's still not empty
            self.current_route = *self.history.last().unwrap();
        }
        Task::none()
    }

    /// Replace the current route without pushing a new history entry
    ///
    /// Useful when you want to redirect or correct navigation without growing history.
    pub fn replace(&mut self, route: Route) -> Task<Message> {
        if let Some(last) = self.history.last_mut() {
            *last = route;
        } else {
            self.history.push(route);
        }
        self.current_route = route;
        Task::none()
    }

    /// Returns the current history length (for diagnostics and tests)
    #[cfg(test)]
    pub(crate) fn history_len(&self) -> usize {
        self.history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router_initializes_with_default_route() {
        let r = Router::new();
        assert_eq!(r.current_route(), Route::Library);
        assert_eq!(r.history_len(), 1);
    }

    #[test]
    fn navigate_to_pushes_and_updates_current() {
        let mut r = Router::new();
    let _ = r.navigate_to(Route::Settings);
        assert_eq!(r.current_route(), Route::Settings);
        assert_eq!(r.history_len(), 2);
    }

    #[test]
    fn navigate_to_dedupes_consecutive_routes() {
        let mut r = Router::new();
    let _ = r.navigate_to(Route::Settings);
    let _ = r.navigate_to(Route::Settings);
        assert_eq!(r.current_route(), Route::Settings);
        assert_eq!(r.history_len(), 2, "should not push duplicate route");
    }

    #[test]
    fn navigate_back_moves_to_previous_when_available() {
        let mut r = Router::new();
    let _ = r.navigate_to(Route::Settings);
    let _ = r.navigate_to(Route::About);
    let _ = r.navigate_back();
        assert_eq!(r.current_route(), Route::Settings);
        assert_eq!(r.history_len(), 2);
    }

    #[test]
    fn navigate_back_noop_on_root() {
        let mut r = Router::new();
    let _ = r.navigate_back();
        assert_eq!(r.current_route(), Route::Library);
        assert_eq!(r.history_len(), 1);
    }

    #[test]
    fn replace_updates_current_without_growing_history() {
        let mut r = Router::new();
    let _ = r.navigate_to(Route::Settings);
        let before = r.history_len();
    let _ = r.replace(Route::About);
        assert_eq!(r.current_route(), Route::About);
        assert_eq!(r.history_len(), before);
    }

    #[test]
    fn history_is_capped() {
        let mut r = Router::new();
        for _ in 0..(Router::MAX_HISTORY + 10) {
            let _ = r.navigate_to(Route::Player);
            let _ = r.navigate_to(Route::Settings);
            let _ = r.navigate_to(Route::About);
            let _ = r.navigate_to(Route::Library);
        }
        assert!(r.history_len() <= Router::MAX_HISTORY);
        // current route remains valid
        assert!(matches!(r.current_route(), Route::Library));
    }
}
