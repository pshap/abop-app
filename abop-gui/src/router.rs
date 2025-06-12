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
    /// 
    /// # Returns
    /// A no-op Task since navigation only updates internal state synchronously
    pub fn navigate_to(&mut self, route: Route) -> Task<Message> {
        self.history.push(route);
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
}
