//! Color tokens for Material Design 3
//!
//! This module contains the color token definitions for the Material Design 3
//! color system. Tokens are organized by their semantic meaning and usage context.

mod core;
mod surface;
mod container;
mod fixed;
mod state;

pub use self::core::CoreTokens;
pub use self::surface::SurfaceTokens;
pub use self::container::ContainerTokens;
pub use self::fixed::FixedTokens;
pub use self::state::StateLayer;

/// Collection of all color tokens for Material Design 3
///
/// The `ColorTokens` struct organizes all color tokens into semantic categories
/// based on their usage and purpose. This organization helps maintain consistency
/// and makes it easier to manage theme changes.
///
/// # Token Categories
///
/// - `core`: Primary semantic colors (primary, secondary, tertiary, error)
/// - `surface`: Colors for surfaces and backgrounds
/// - `container`: Colors for container elements
/// - `fixed`: Colors that don't change with theme (e.g., black, white)
///
/// # Examples
///
/// ```rust
/// use abop_iced::material::color::token::ColorTokens;
///
/// // Create default tokens
/// let tokens = ColorTokens::default();
///
/// // Access core tokens
/// let primary = tokens.core.primary;
///
/// // Access surface tokens
/// let background = tokens.surface.background;
///
/// // Access container tokens
/// let container = tokens.container.primary;
///
/// // Access fixed tokens
/// let black = tokens.fixed.black;
/// ```
#[derive(Debug, Clone)]
pub struct ColorTokens {
    /// Core semantic color tokens
    ///
    /// Contains the primary semantic colors used throughout the application,
    /// including primary, secondary, tertiary, and error colors.
    pub core: CoreTokens,
    /// Surface color tokens
    ///
    /// Contains colors used for surfaces and backgrounds, including
    /// background, surface, and their variants.
    pub surface: SurfaceTokens,
    /// Container color tokens
    ///
    /// Contains colors used for container elements, including
    /// primary, secondary, and tertiary containers.
    pub container: ContainerTokens,
    /// Fixed color tokens
    ///
    /// Contains colors that don't change with theme, such as
    /// black, white, and other fixed colors.
    pub fixed: FixedTokens,
}

impl Default for ColorTokens {
    fn default() -> Self {
        Self {
            core: CoreTokens::default(),
            surface: SurfaceTokens::default(),
            container: ContainerTokens::default(),
            fixed: FixedTokens::default(),
        }
    }
}
