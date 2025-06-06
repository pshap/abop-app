//! # Material Design 3 Color System
//!
//! This module provides a comprehensive implementation of the Material Design 3 (MD3) color system,
//! including color roles, theming, and accessibility features.
//!
//! ## Features
//!
//! - **Color Tokens**: Semantic color tokens for consistent theming
//! - **Dynamic Theming**: Support for light/dark themes and custom color schemes
//! - **Accessibility**: Built-in contrast checking and color adjustment
//! - **HCT Color Space**: Perceptually accurate color manipulation
//!
//! ## Basic Usage
//!
//! ```rust
//! use abop_iced::material::color::{Theme, ThemeVariant, Srgb};
//!
//! // Create a light theme
//! let mut theme = Theme::light();
//!
//! // Access color tokens
//! let primary = theme.colors.primary;
//! let on_primary = theme.colors.on_primary;
//!
//! // Toggle between light/dark themes
//! theme.toggle();
//! ```
//!
//! ## Creating a Theme from a Seed Color
//!
//! You can create a theme with a custom seed color that will generate a complete,
//! harmonious color scheme:
//!
//! ```rust
//! use abop_iced::material::color::{Theme, ThemeVariant, Srgb};
//!
//! // Create a theme with a custom seed color
//! let theme = Theme::from_seed(Srgb::new(0.5, 0.2, 0.8), ThemeVariant::Light);
//! ```
//!
//! ## Advanced Theming with DynamicTheme
//!
//! For more control over the theming process, use the `DynamicTheme` builder:
//!
//! ```rust
//! use abop_iced::material::color::{DynamicTheme, ThemeVariant, Srgb};
//!
//! let theme = DynamicTheme::new()
//!     .with_seed_color(Srgb::new(0.3, 0.5, 0.8))
//!     .with_variant(ThemeVariant::Dark) // Optional: default is Light
//!     .with_custom_color("primary", Srgb::new(0.2, 0.4, 0.8)) // Override specific colors
//!     .generate_theme();
//! ```
//!
//! ## Color Roles
//!
//! The color system is based on semantic color roles that define the purpose of a color:
//!
//! - **Primary**: The primary brand color
//! - **Secondary**: Accent colors that complement the primary color
//! - **Tertiary**: Additional accent colors
//! - **Error**: Colors for error states
//! - **Background/Surface**: Colors for surfaces and backgrounds
//! - **On-* colors**: Colors for content that appears on top of a color
//!
//! Each role has a main color and container/on-* variants for different UI states.
//!
//! ## Light and Dark Themes
//!
//! The system supports both light and dark themes, with appropriate contrast ratios
//! and color adjustments for each theme variant.
//!
//! ```rust
//! use abop_iced::material::color::{Theme, ThemeVariant};
//!
//! // Create a dark theme
//! let dark_theme = Theme::dark();
//!
//! // Or create a light theme
//! let light_theme = Theme::light();
//! ```
//!
//! ## Customization
//!
//! You can customize individual colors after theme creation:
//!
//! ```rust
//! use abop_iced::material::color::{Theme, Srgb};
//!
//! let mut theme = Theme::light();
//! theme.colors.primary = Srgb::new(0.1, 0.4, 0.8);
//! theme.colors.on_primary = Srgb::new(1.0, 1.0, 1.0);
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

mod palette;
pub use palette::{TonalPalette, MaterialPalette};

mod token;
pub use token::{
    core::CoreTokens,
    surface::SurfaceTokens,
    container::ContainerTokens,
    fixed::FixedTokens,
    state::StateLayer,
};

mod scheme;
pub use scheme::{Theme, ThemeVariant, DynamicTheme};

mod hct;
mod contrast;

/// Color in sRGB color space
///
/// The `Srgb` struct represents a color in the sRGB color space, which is the
/// standard color space used for digital displays. Each component (red, green, blue)
/// is represented as a floating-point value between 0.0 and 1.0.
///
/// # Examples
///
/// ```rust
/// use abop_iced::material::color::Srgb;
///
/// // Create a red color
/// let red = Srgb::new(1.0, 0.0, 0.0);
///
/// // Create a green color
/// let green = Srgb::new(0.0, 1.0, 0.0);
///
/// // Create a blue color
/// let blue = Srgb::new(0.0, 0.0, 1.0);
///
/// // Convert to RGBA
/// let rgba = red.to_rgba(0.5); // Semi-transparent red
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Srgb {
    /// Red component (0.0 - 1.0)
    ///
    /// The red component of the color, where 0.0 is no red and 1.0 is full red.
    pub r: f32,
    /// Green component (0.0 - 1.0)
    ///
    /// The green component of the color, where 0.0 is no green and 1.0 is full green.
    pub g: f32,
    /// Blue component (0.0 - 1.0)
    ///
    /// The blue component of the color, where 0.0 is no blue and 1.0 is full blue.
    pub b: f32,
}

impl Srgb {
    /// Create a new sRGB color
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (0.0 - 1.0)
    /// * `g` - Green component (0.0 - 1.0)
    /// * `b` - Blue component (0.0 - 1.0)
    ///
    /// # Returns
    ///
    /// A new `Srgb` color with the specified components.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use abop_iced::material::color::Srgb;
    ///
    /// let color = Srgb::new(0.5, 0.2, 0.8);
    /// assert_eq!(color.r, 0.5);
    /// assert_eq!(color.g, 0.2);
    /// assert_eq!(color.b, 0.8);
    /// ```
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Convert to 32-bit RGBA color (0xAARRGGBB)
    ///
    /// # Arguments
    ///
    /// * `alpha` - Alpha component (0.0 - 1.0)
    ///
    /// # Returns
    ///
    /// A 32-bit unsigned integer representing the color in RGBA format,
    /// where the bits are arranged as 0xAARRGGBB.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use abop_iced::material::color::Srgb;
    ///
    /// let color = Srgb::new(1.0, 0.0, 0.0);
    /// let rgba = color.to_rgba(0.5);
    /// assert_eq!(rgba, 0x80FF0000); // Semi-transparent red
    /// ```
    pub fn to_rgba(&self, alpha: f32) -> u32 {
        let r = (self.r.clamp(0.0, 1.0) * 255.0) as u32;
        let g = (self.g.clamp(0.0, 1.0) * 255.0) as u32;
        let b = (self.b.clamp(0.0, 1.0) * 255.0) as u32;
        let a = (alpha.clamp(0.0, 1.0) * 255.0) as u32;
        
        (a << 24) | (r << 16) | (g << 8) | b
    }
}

/// Common color roles used throughout the application
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorRoles {
    // Primary colors
    /// Primary color role
    pub primary: Srgb,
    /// On-primary color (text/iconography on primary)
    pub on_primary: Srgb,
    /// Primary container color
    pub primary_container: Srgb,
    /// On-primary container color
    pub on_primary_container: Srgb,
    
    // Secondary colors
    /// Secondary color role
    pub secondary: Srgb,
    /// On-secondary color (text/iconography on secondary)
    pub on_secondary: Srgb,
    /// Secondary container color
    pub secondary_container: Srgb,
    /// On-secondary container color
    pub on_secondary_container: Srgb,
    
    // Tertiary colors
    /// Tertiary color role
    pub tertiary: Srgb,
    /// On-tertiary color (text/iconography on tertiary)
    pub on_tertiary: Srgb,
    /// Tertiary container color
    pub tertiary_container: Srgb,
    /// On-tertiary container color
    pub on_tertiary_container: Srgb,
    
    // Error colors
    /// Error color role
    pub error: Srgb,
    /// On-error color (text/iconography on error)
    pub on_error: Srgb,
    /// Error container color
    pub error_container: Srgb,
    /// On-error container color
    pub on_error_container: Srgb,
    
    // Background colors
    /// Background color
    pub background: Srgb,
    /// On-background color (text/iconography on background)
    pub on_background: Srgb,
    
    // Surface colors
    /// Surface color
    pub surface: Srgb,
    /// On-surface color (text/iconography on surface)
    pub on_surface: Srgb,
    /// Surface variant color
    pub surface_variant: Srgb,
    /// On-surface variant color
    pub on_surface_variant: Srgb,
    
    // Outline colors
    /// Outline color
    pub outline: Srgb,
    /// Outline variant color
    pub outline_variant: Srgb,
    
    // Shadow and scrim
    /// Shadow color
    pub shadow: Srgb,
    /// Scrim color
    pub scrim: Srgb,
    
    // Inverse colors
    /// Inverse surface color
    pub inverse_surface: Srgb,
    /// On-inverse surface color
    pub inverse_on_surface: Srgb,
    /// Inverse primary color
    pub inverse_primary: Srgb,
}

/// Theme data structure
#[derive(Debug, Clone)]
pub struct Theme {
    /// Color scheme variant (light/dark)
    pub variant: ThemeVariant,
    /// Core color roles
    pub colors: ColorRoles,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            variant: ThemeVariant::Light,
            colors: ColorRoles {
                // Default light theme colors
                primary: Srgb::new(0.0, 0.0, 0.0),  // Black as default
                on_primary: Srgb::new(1.0, 1.0, 1.0),  // White as default
                primary_container: Srgb::new(0.9, 0.9, 0.9),  // Light gray
                on_primary_container: Srgb::new(0.1, 0.1, 0.1),  // Dark gray
            },
        }
    }
}
