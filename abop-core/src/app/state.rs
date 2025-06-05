//! Application state management

use serde::{Deserialize, Serialize};

/// Main application state
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// Current view/screen
    pub current_view: View,
    // Add other state fields here
}

/// Available views/screens in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum View {
    /// Library view - shows the audiobook collection
    #[default]
    Library,
    /// Player view - shows the currently playing audiobook
    Player,
    /// Settings view - application settings
    Settings,
}
