//! Constants and magic numbers for ABOP

/// UI-related constants and dimensions
pub mod ui {
    /// Minimum allowed window width (800px)
    pub const WINDOW_MIN_WIDTH: u32 = 800;
    /// Minimum allowed window height (600px)
    pub const WINDOW_MIN_HEIGHT: u32 = 600;
    /// Standard spacing between buttons (10px)
    pub const BUTTON_SPACING: u16 = 10;
    /// Standard padding for panels (20px)
    pub const PANEL_PADDING: u16 = 20;
    /// Default spacing for UI elements (10px)
    pub const DEFAULT_SPACING: u16 = 10;
    /// Large spacing for section separation (20px)
    pub const LARGE_SPACING: u16 = 20;
}
/// Configuration-related constants
pub mod config {
    /// Application display name
    pub const APP_NAME: &str = "ABOP Iced";
    /// Configuration file name
    pub const CONFIG_FILE: &str = "config.toml";
    /// Data storage file name
    pub const DATA_FILE: &str = "data.json";
}
