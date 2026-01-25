//! Assets module for the application

use iced::{Font, Task};

/// Font assets including embedded fonts
pub mod fonts {
    use super::Font;

    /// Font Awesome icon font bytes - using a simple fallback
    pub const ICON_FONT_BYTES: &[u8] = b"";

    /// Material Design Roboto font family
    pub mod roboto {
        use super::Font;

        /// Roboto Regular font (using system default)
        pub const REGULAR: Font = Font::DEFAULT;

        /// Roboto Medium font (using system default)
        pub const MEDIUM: Font = Font::DEFAULT;

        /// Roboto Bold font (using system default)
        pub const BOLD: Font = Font::DEFAULT;

        /// Get font by `iced::font::Weight`
        #[must_use]
        pub const fn by_weight(_weight: iced::font::Weight) -> Font {
            // Always return system default to avoid font rendering issues
            Font::DEFAULT
        }
    }

    /// Default system font
    pub const DEFAULT: Font = Font::DEFAULT;
}

/// Register fonts with Iced
/// This function returns a Task that loads the required fonts
pub fn register_fonts() -> Task<()> {
    log::info!("Registering application fonts...");

    // Load FontAwesome Solid icons
    Task::batch(vec![iced::font::load(fonts::ICON_FONT_BYTES)]).map(|_| ())
}
