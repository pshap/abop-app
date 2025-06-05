//! Style trait system for consistent component styling
//!
//! This module provides trait-based abstractions for component styling
//! to enable flexible and reusable styling patterns.

use crate::theme::ThemeMode;
use iced::Color;

/// Style variant enumeration for different component states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleVariant {
    /// Default/primary variant
    Primary,
    /// Secondary variant with less prominence
    Secondary,
    /// Success variant for positive actions
    Success,
    /// Warning variant for cautionary actions
    Warning,
    /// Error variant for destructive actions
    Error,
    /// Info variant for informational actions
    Info,
    /// Outline variant with transparent background
    Outline,
    /// Text variant with minimal styling
    Text,
    /// Active/selected variant
    Active,
    /// Disabled variant
    Disabled,
}

/// Component size enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentSize {
    /// Small size for compact layouts
    Small,
    /// Medium size (default)
    Medium,
    /// Large size for emphasis
    Large,
}

/// Component state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentState {
    /// Normal state
    Normal,
    /// Hovered state
    Hovered,
    /// Pressed/active state
    Pressed,
    /// Focused state
    Focused,
    /// Disabled state
    Disabled,
}

/// Trait for components that can be styled based on theme
pub trait ComponentStyle<T> {
    /// Generate style based on current theme
    fn style(&self, theme: &ThemeMode) -> T;

    /// Get the current style variant
    fn variant(&self) -> StyleVariant;

    /// Get the current component size
    fn size(&self) -> ComponentSize;

    /// Get the current component state
    fn state(&self) -> ComponentState;
}

/// Trait for components that can respond to theme changes
pub trait ThemeAware {
    /// Apply theme changes to the component
    fn apply_theme(&mut self, theme: &ThemeMode);

    /// Check if component supports dark mode
    fn supports_dark_mode(&self) -> bool {
        true
    }

    /// Check if component supports light mode
    fn supports_light_mode(&self) -> bool {
        true
    }
}

/// Trait for style builders that can be configured fluently
pub trait StyleBuilder<T> {
    /// Build the final style
    fn build(self, theme: &ThemeMode) -> T;
}

/// Trait for components that can be configured with semantic colors
pub trait SemanticColorAware {
    /// Set the primary color
    fn with_primary_color(self, color: Color) -> Self;

    /// Set the secondary color
    fn with_secondary_color(self, color: Color) -> Self;

    /// Set the success color
    fn with_success_color(self, color: Color) -> Self;

    /// Set the warning color
    fn with_warning_color(self, color: Color) -> Self;

    /// Set the error color
    fn with_error_color(self, color: Color) -> Self;

    /// Set the info color
    fn with_info_color(self, color: Color) -> Self;
}

/// Trait for components that support interactive states
pub trait InteractiveStyle<T> {
    /// Get normal state style
    fn normal(&self, theme: &ThemeMode) -> T;

    /// Get hover state style
    fn hover(&self, theme: &ThemeMode) -> T;

    /// Get active/pressed state style
    fn active(&self, theme: &ThemeMode) -> T;

    /// Get disabled state style
    fn disabled(&self, theme: &ThemeMode) -> T;
}

/// Trait for components that support validation states
pub trait ValidationStyle<T> {
    /// Get default/valid state style
    fn valid(&self, theme: &ThemeMode) -> T;

    /// Get error state style
    fn error(&self, theme: &ThemeMode) -> T;

    /// Get warning state style
    fn warning(&self, theme: &ThemeMode) -> T;

    /// Get success state style
    fn success(&self, theme: &ThemeMode) -> T;
}

/// Helper macro for creating style builders
#[macro_export]
macro_rules! create_style_builder {
    (
        $builder_name:ident for $style_type:ty {
            $(
                $field:ident: $field_type:ty = $default:expr
            ),* $(,)?
        }
    ) => {
        /// Style builder for creating configured styles
        #[derive(Debug, Clone)]
        pub struct $builder_name {
            $(
                $field: $field_type,
            )*
        }

        impl $builder_name {
            /// Create a new style builder with default values
            #[must_use]
            pub fn new() -> Self {
                Self {
                    $(
                        $field: $default,
                    )*
                }
            }

            $(
                /// Set the field value
                pub fn $field(mut self, value: $field_type) -> Self {
                    self.$field = value;
                    self
                }
            )*
        }

        impl Default for $builder_name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl StyleBuilder<$style_type> for $builder_name {
            fn build(self, theme: &ThemeMode) -> $style_type {
                self.style(theme)
            }
        }
    };
}

/// Common style utilities that work with the trait system
pub struct StyleUtils;

impl StyleUtils {
    /// Get color for a semantic variant
    #[must_use]
    pub fn color_for_variant(variant: StyleVariant, theme: &ThemeMode) -> Color {
        match variant {
            StyleVariant::Primary => theme.primary_color(),
            StyleVariant::Secondary => theme.secondary_color(),
            StyleVariant::Success => theme.success_color(),
            StyleVariant::Warning => theme.warning_color(),
            StyleVariant::Error => theme.error_color(),
            StyleVariant::Info => theme.info_color(),
            StyleVariant::Outline => theme.border_color(),
            StyleVariant::Text => theme.text_primary_color(),
            StyleVariant::Active => theme.primary_light_color(),
            StyleVariant::Disabled => Color {
                a: 0.4,
                ..theme.text_secondary_color()
            },
        }
    }

    /// Get appropriate text color for a background color variant
    #[must_use]
    pub fn text_color_for_variant(variant: StyleVariant, theme: &ThemeMode) -> Color {
        match variant {
            StyleVariant::Primary
            | StyleVariant::Success
            | StyleVariant::Warning
            | StyleVariant::Error
            | StyleVariant::Info => {
                // Use white text for colored backgrounds for better contrast
                Color::WHITE
            }
            StyleVariant::Secondary | StyleVariant::Outline | StyleVariant::Text => {
                theme.text_primary_color()
            }
            StyleVariant::Active => theme.text_primary_color(),
            StyleVariant::Disabled => Color {
                a: 0.4,
                ..theme.text_secondary_color()
            },
        }
    }

    /// Get padding values for component size
    #[must_use]
    pub const fn padding_for_size(size: ComponentSize) -> (f32, f32) {
        match size {
            ComponentSize::Small => (8.0, 4.0), // horizontal, vertical
            ComponentSize::Medium => (16.0, 8.0),
            ComponentSize::Large => (24.0, 12.0),
        }
    }

    /// Get font size for component size
    #[must_use]
    pub const fn font_size_for_size(size: ComponentSize) -> u16 {
        match size {
            ComponentSize::Small => 12,
            ComponentSize::Medium => 14,
            ComponentSize::Large => 16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_variant_enum() {
        let variant = StyleVariant::Primary;
        assert_eq!(variant, StyleVariant::Primary);
    }

    #[test]
    fn test_component_size_enum() {
        let size = ComponentSize::Medium;
        assert_eq!(size, ComponentSize::Medium);
    }

    #[test]
    fn test_padding_for_size() {
        let (h, v) = StyleUtils::padding_for_size(ComponentSize::Medium);
        assert_eq!(h, 16.0);
        assert_eq!(v, 8.0);
    }
}
