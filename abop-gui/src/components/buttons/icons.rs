//! Icon-related functionality for buttons

use super::error::{ButtonError, ButtonResult};
use super::variants::IconPosition;
use iced::Element;

/// A validated icon size that ensures icons stay within reasonable bounds
///
/// This newtype wrapper provides type safety and validation for icon sizes,
/// preventing extremely small or large icons that could break the UI.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IconSize(f32);

impl IconSize {
    /// Minimum allowed icon size (8px)
    pub const MIN: f32 = 8.0;

    /// Maximum allowed icon size (48px)
    pub const MAX: f32 = 48.0;

    /// Small icon size (16px)
    pub const SMALL: IconSize = IconSize(16.0);

    /// Medium icon size (20px) - default
    pub const MEDIUM: IconSize = IconSize(20.0);

    /// Large icon size (24px)
    pub const LARGE: IconSize = IconSize(24.0);

    /// Create a new validated icon size
    ///
    /// # Arguments
    /// * `size` - The desired icon size in pixels
    ///
    /// # Returns
    /// * `Ok(IconSize)` if the size is within valid bounds
    /// * `Err(ButtonError::InvalidConfiguration)` if the size is too small or too large
    ///
    /// # Examples
    /// ```
    /// use abop_gui::components::buttons::icons::IconSize;
    ///
    /// let size = IconSize::new(20.0)?; // Valid size
    /// let invalid = IconSize::new(100.0); // Returns error
    /// ```
    pub fn new(size: f32) -> ButtonResult<Self> {
        if size < Self::MIN {
            Err(ButtonError::InvalidConfiguration("Icon size too small"))
        } else if size > Self::MAX {
            Err(ButtonError::InvalidConfiguration("Icon size too large"))
        } else {
            Ok(IconSize(size))
        }
    }

    /// Create a new icon size without validation (for constants)
    ///
    /// # Safety
    /// This should only be used for compile-time constants where the size is known to be valid.
    /// For runtime values, use `new()` instead.
    pub const fn new_unchecked(size: f32) -> Self {
        IconSize(size)
    }

    /// Get the size value as f32
    pub fn value(self) -> f32 {
        self.0
    }
}

impl Default for IconSize {
    fn default() -> Self {
        Self::MEDIUM
    }
}

impl From<IconSize> for f32 {
    fn from(size: IconSize) -> f32 {
        size.0
    }
}

/// Configuration for an icon in a button
#[derive(Debug, Clone)]
pub(crate) struct IconConfig<'a> {
    /// The name of the icon
    pub name: &'a str,

    /// The size of the icon
    pub size: IconSize,

    /// The position of the icon relative to the text
    pub position: IconPosition,
}

impl<'a> IconConfig<'a> {
    /// Create a new icon configuration
    pub fn new(name: &'a str, size: IconSize, position: IconPosition) -> Self {
        Self {
            name,
            size,
            position,
        }
    }

    /// Convert the icon configuration to an element with the given message type
    pub fn to_element<M: 'a>(&self) -> Element<'a, M> {
        iced_font_awesome::fa_icon_solid(self.name)
            .size(self.size.value())
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_size_validation() {
        // Valid sizes
        assert!(IconSize::new(16.0).is_ok());
        assert!(IconSize::new(20.0).is_ok());
        assert!(IconSize::new(24.0).is_ok());

        // Invalid sizes
        assert!(IconSize::new(7.0).is_err()); // Too small
        assert!(IconSize::new(50.0).is_err()); // Too large
    }

    #[test]
    fn test_icon_size_constants() {
        assert_eq!(IconSize::SMALL.value(), 16.0);
        assert_eq!(IconSize::MEDIUM.value(), 20.0);
        assert_eq!(IconSize::LARGE.value(), 24.0);
    }

    #[test]
    fn test_icon_size_default() {
        assert_eq!(IconSize::default(), IconSize::MEDIUM);
    }

    #[test]
    fn test_icon_config_creation() {
        let config = IconConfig::new("test-icon", IconSize::MEDIUM, IconPosition::Leading);
        assert_eq!(config.name, "test-icon");
        assert_eq!(config.size, IconSize::MEDIUM);
    }
}
