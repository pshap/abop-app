//! Icon components and utilities for consistent iconography

use iced::Element;
use iced_font_awesome::fa_icon_solid;

// Icon constants for Font Awesome icons (using string names)
/// Icon constants for consistent iconography across the application
pub mod icon_names {
    // Navigation icons
    /// Library/book icon for library view navigation
    pub const LIBRARY: &str = "book-open";
    /// Audio processing icon for audio view navigation
    pub const AUDIO_PROCESSING: &str = "volume-high";
    /// Settings gear icon for settings view navigation
    pub const SETTINGS: &str = "gear";
    /// Information circle icon for help/info sections
    pub const INFO: &str = "circle-info";

    // Library action icons
    /// Folder open icon for directory browsing
    pub const FOLDER_OPEN: &str = "folder-open";
    /// Magnifying glass icon for search functionality
    pub const SEARCH: &str = "magnifying-glass";
    /// Refresh/reload icon for refreshing library content
    pub const REFRESH: &str = "arrow-rotate-right";
    /// Download icon for downloading content
    pub const DOWNLOAD: &str = "download"; // Media control icons
    /// Play button icon for media playback
    pub const PLAY: &str = "play";
    /// Pause button icon for media playback
    pub const PAUSE: &str = "pause";
    /// Stop button icon for media playback
    pub const STOP: &str = "stop";
    /// Skip forward icon for media navigation
    pub const SKIP_FORWARD: &str = "forward-step";
    /// Skip backward icon for media navigation
    pub const SKIP_BACKWARD: &str = "backward-step";
    /// Skip to previous track
    pub const SKIP_PREVIOUS: &str = "backward-step";
    /// Skip to next track
    pub const SKIP_NEXT: &str = "forward-step";

    // General UI icons
    /// Close/X icon for dismissing dialogs and menus
    pub const CLOSE: &str = "xmark";
    /// Check mark icon for confirmations and success states
    pub const CHECK: &str = "check";
    /// Plus icon for adding items
    pub const PLUS: &str = "plus";
    /// Minus icon for removing items
    pub const MINUS: &str = "minus";
    /// Edit/pencil icon for editing functionality
    pub const EDIT: &str = "pen-to-square";
    /// Delete/trash icon for deletion functionality
    pub const DELETE: &str = "trash";
    /// Save/disk icon for save operations
    pub const SAVE: &str = "floppy-disk";
    /// Export icon for exporting data
    pub const EXPORT: &str = "file-export";
    /// Import icon for importing data
    pub const IMPORT: &str = "file-import";
    /// Filter icon for filtering content
    pub const FILTER: &str = "filter";
    /// Sort icon for sorting content
    pub const SORT: &str = "arrow-up-z-a";
}

/// Small icon (12px) - for compact UI elements
#[must_use]
pub fn small_icon<'a>(icon_name: &str) -> Element<'a, crate::messages::Message> {
    fa_icon_solid(icon_name).size(12.0).into()
}

/// Medium icon (16px) - default size for buttons
#[must_use]
pub fn medium_icon<'a>(icon_name: &str) -> Element<'a, crate::messages::Message> {
    fa_icon_solid(icon_name).size(16.0).into()
}

/// Large icon (20px)
#[must_use]
pub fn large_icon<'a>(icon_name: &str) -> Element<'a, crate::messages::Message> {
    fa_icon_solid(icon_name).size(20.0).into()
}

/// Extra large icon (24px) - for prominent actions
#[must_use]
pub fn xl_icon<'a>(icon_name: &str) -> Element<'a, crate::messages::Message> {
    fa_icon_solid(icon_name).size(24.0).into()
}

/// Navigation icon (18px) - specific size for navigation tabs
#[must_use]
pub fn nav_icon<'a>(icon_name: &str) -> Element<'a, crate::messages::Message> {
    fa_icon_solid(icon_name).size(18.0).into()
}

/// Toolbar icon (16px) - specific size for toolbar buttons
#[must_use]
pub fn toolbar_icon<'a>(icon_name: &str) -> Element<'a, crate::messages::Message> {
    fa_icon_solid(icon_name).size(16.0).into()
}

/// Creates an icon with size determined by button context using centralized sizing.
///
/// **RECOMMENDED**: Use this function for button icons instead of the fixed-size functions above.
/// This ensures consistent icon sizing that follows Material Design 3 specifications for
/// different button variants and sizes.
///
/// # Arguments
/// * `icon_name` - Font Awesome icon name (from `icon_names` module)
/// * `button_variant` - The button variant this icon will be used in
/// * `button_size` - The button size this icon will be used in
#[must_use]
pub fn button_icon<'a, Message>(
    icon_name: &str,
    button_variant: crate::styling::material::components::button_style::ButtonStyleVariant,
    button_size: crate::styling::material::components::button_style::ButtonSizeVariant,
) -> Element<'a, Message> {
    crate::styling::material::components::button_style::create_button_icon(
        icon_name,
        button_variant,
        button_size,
    )
}
