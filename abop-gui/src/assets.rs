//! Assets module for the application

use iced::Font;

/// Font assets including embedded fonts
pub mod fonts {
    use super::Font;

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
/// This function configures the application to use system fonts
/// to ensure proper text rendering and avoid wingdings issues
pub fn register_fonts() {
    // Use system font fallback to avoid font rendering issues
    // All custom font references have been replaced with Font::DEFAULT

    log::info!("Font registration: Using system font fallback");
    log::info!("All text elements configured to use system default fonts");
}
