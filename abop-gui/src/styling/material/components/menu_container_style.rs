//! Material Design 3 Menu Container Styling System
//!
//! This module provides a centralized styling system for all menu container styles,
//! eliminating code duplication across MaterialMenu, MaterialSelectMenu, and MaterialAutocomplete.
//!
//! ## Design Goals
//! - Centralized container styling for all menu components
//! - Consistent Material Design 3 elevation and shadow patterns
//! - Reduced code duplication from menus.rs file
//! - Type-safe styling with clear variants for different menu types

use iced::{Background, Border, Shadow, Theme, Vector, widget::container};

use crate::styling::material::{MaterialTokens, elevation::ElevationLevel};

/// Menu container variants for different menu component types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuContainerVariant {
    /// Standard dropdown menu container with medium elevation
    /// Used for `MaterialMenu` component
    DropdownMenu,
    /// Select menu container with medium elevation
    /// Used for `MaterialSelectMenu` component
    SelectMenu,
    /// Autocomplete suggestions container with border and medium elevation
    /// Used for `MaterialAutocomplete` component
    AutocompleteSuggestions,
    /// Divider container for visual separation within menus
    /// Used for divider elements in menu items
    Divider,
}

/// Creates a menu container style function for the specified variant
///
/// This function replaces the duplicated container styling patterns found in
/// `MaterialMenu`, `MaterialSelectMenu`, and `MaterialAutocomplete` components.
///
/// # Arguments
/// * `variant` - The menu container variant to style
/// * `tokens` - Material Design tokens for consistent styling
///
/// # Returns
/// A style function that can be used with Iced container components
pub fn create_menu_container_style(
    variant: MenuContainerVariant,
    tokens: &MaterialTokens,
) -> impl Fn(&Theme) -> container::Style {
    let colors = tokens.colors.clone();
    let elevation = tokens.elevation.clone();
    let corner_radius = tokens.shapes.corner_extra_small.to_radius();
    let surface_container = colors.surface_container;
    let outline_variant = colors.outline_variant;

    move |_theme: &Theme| match variant {
        MenuContainerVariant::DropdownMenu | MenuContainerVariant::SelectMenu => container::Style {
            background: Some(Background::Color(surface_container)),
            border: Border {
                radius: corner_radius,
                ..Default::default()
            },
            shadow: elevation.get_level(ElevationLevel::Level2).shadow,
            ..Default::default()
        },

        MenuContainerVariant::AutocompleteSuggestions => container::Style {
            background: Some(Background::Color(surface_container)),
            border: Border {
                radius: corner_radius,
                width: 1.0,
                color: outline_variant,
            },
            shadow: elevation.get_level(ElevationLevel::Level2).shadow,
            ..Default::default()
        },

        MenuContainerVariant::Divider => container::Style {
            background: Some(Background::Color(outline_variant)),
            ..Default::default()
        },
    }
}

/// Creates a menu container style with custom shadow offset
///
/// Allows customization of shadow offset while maintaining consistent other properties.
/// Used when `MaterialMenu` needs different shadow offset values.
///
/// # Arguments
/// * `variant` - The menu container variant to style
/// * `shadow_offset_y` - Custom Y offset for the shadow
/// * `tokens` - Material Design tokens for consistent styling
///
/// # Returns
/// A style function with custom shadow offset
pub fn create_menu_container_style_with_shadow(
    variant: MenuContainerVariant,
    shadow_offset_y: f32,
    tokens: &MaterialTokens,
) -> impl Fn(&Theme) -> container::Style {
    let colors = tokens.colors.clone();
    let corner_radius = tokens.shapes.corner_extra_small.to_radius();
    let shadow_color = colors.shadow;
    let surface_container = colors.surface_container;
    let outline_variant = colors.outline_variant;

    move |_theme: &Theme| match variant {
        MenuContainerVariant::DropdownMenu | MenuContainerVariant::SelectMenu => container::Style {
            background: Some(Background::Color(surface_container)),
            border: Border {
                radius: corner_radius,
                ..Default::default()
            },
            shadow: Shadow {
                color: shadow_color,
                offset: Vector::new(0.0, shadow_offset_y),
                blur_radius: 8.0,
            },
            ..Default::default()
        },

        MenuContainerVariant::AutocompleteSuggestions => container::Style {
            background: Some(Background::Color(surface_container)),
            border: Border {
                radius: corner_radius,
                width: 1.0,
                color: outline_variant,
            },
            shadow: Shadow {
                color: shadow_color,
                offset: Vector::new(0.0, shadow_offset_y),
                blur_radius: 8.0,
            },
            ..Default::default()
        },

        MenuContainerVariant::Divider => container::Style {
            background: Some(Background::Color(outline_variant)),
            ..Default::default()
        },
    }
}

/// Creates a menu container style with custom blur radius
///
/// Allows customization of shadow blur radius while maintaining consistent other properties.
///
/// # Arguments
/// * `variant` - The menu container variant to style
/// * `blur_radius` - Custom blur radius for the shadow
/// * `tokens` - Material Design tokens for consistent styling
///
/// # Returns
/// A style function with custom blur radius
pub fn create_menu_container_style_with_blur(
    variant: MenuContainerVariant,
    blur_radius: f32,
    tokens: &MaterialTokens,
) -> impl Fn(&Theme) -> container::Style {
    let colors = tokens.colors.clone();
    let corner_radius = tokens.shapes.corner_extra_small.to_radius();
    let shadow_color = colors.shadow;
    let surface_container = colors.surface_container;
    let outline_variant = colors.outline_variant;

    move |_theme: &Theme| match variant {
        MenuContainerVariant::DropdownMenu | MenuContainerVariant::SelectMenu => container::Style {
            background: Some(Background::Color(surface_container)),
            border: Border {
                radius: corner_radius,
                ..Default::default()
            },
            shadow: Shadow {
                color: shadow_color,
                offset: Vector::new(0.0, 2.0),
                blur_radius,
            },
            ..Default::default()
        },

        MenuContainerVariant::AutocompleteSuggestions => container::Style {
            background: Some(Background::Color(surface_container)),
            border: Border {
                radius: corner_radius,
                width: 1.0,
                color: outline_variant,
            },
            shadow: Shadow {
                color: shadow_color,
                offset: Vector::new(0.0, 2.0),
                blur_radius,
            },
            ..Default::default()
        },

        MenuContainerVariant::Divider => container::Style {
            background: Some(Background::Color(outline_variant)),
            ..Default::default()
        },
    }
}

/// Creates a fully customizable menu container style
///
/// Provides complete control over shadow properties while maintaining Material Design patterns.
///
/// # Arguments
/// * `variant` - The menu container variant to style
/// * `shadow_offset_y` - Custom Y offset for the shadow
/// * `blur_radius` - Custom blur radius for the shadow
/// * `tokens` - Material Design tokens for consistent styling
///
/// # Returns
/// A style function with fully customizable shadow
pub fn create_menu_container_style_custom(
    variant: MenuContainerVariant,
    shadow_offset_y: f32,
    blur_radius: f32,
    tokens: &MaterialTokens,
) -> impl Fn(&Theme) -> container::Style {
    let colors = tokens.colors.clone();
    let corner_radius = tokens.shapes.corner_extra_small.to_radius();
    let shadow_color = colors.shadow;
    let surface_container = colors.surface_container;
    let outline_variant = colors.outline_variant;

    move |_theme: &Theme| match variant {
        MenuContainerVariant::DropdownMenu | MenuContainerVariant::SelectMenu => container::Style {
            background: Some(Background::Color(surface_container)),
            border: Border {
                radius: corner_radius,
                ..Default::default()
            },
            shadow: Shadow {
                color: shadow_color,
                offset: Vector::new(0.0, shadow_offset_y),
                blur_radius,
            },
            ..Default::default()
        },

        MenuContainerVariant::AutocompleteSuggestions => container::Style {
            background: Some(Background::Color(surface_container)),
            border: Border {
                radius: corner_radius,
                width: 1.0,
                color: outline_variant,
            },
            shadow: Shadow {
                color: shadow_color,
                offset: Vector::new(0.0, shadow_offset_y),
                blur_radius,
            },
            ..Default::default()
        },

        MenuContainerVariant::Divider => container::Style {
            background: Some(Background::Color(outline_variant)),
            ..Default::default()
        },
    }
}

/// Creates a menu container style function with custom elevation level
///
/// This function allows components to specify their own elevation level
/// while still using the centralized styling patterns.
///
/// # Arguments
/// * `variant` - The menu container variant to style
/// * `elevation_level` - The elevation level to apply
/// * `tokens` - Material Design tokens for consistent styling
///
/// # Returns
/// A style function with proper Material Design elevation
pub fn create_menu_container_style_with_elevation(
    variant: MenuContainerVariant,
    elevation_level: ElevationLevel,
    tokens: &MaterialTokens,
) -> impl Fn(&Theme) -> container::Style {
    let colors = tokens.colors.clone();
    let elevation = tokens.elevation.clone();
    let corner_radius = tokens.shapes.corner_extra_small.to_radius();
    let surface_container = colors.surface_container;
    let outline_variant = colors.outline_variant;

    move |_theme: &Theme| match variant {
        MenuContainerVariant::DropdownMenu | MenuContainerVariant::SelectMenu => container::Style {
            background: Some(Background::Color(surface_container)),
            border: Border {
                radius: corner_radius,
                ..Default::default()
            },
            shadow: elevation.get_level(elevation_level).shadow,
            ..Default::default()
        },

        MenuContainerVariant::AutocompleteSuggestions => container::Style {
            background: Some(Background::Color(surface_container)),
            border: Border {
                radius: corner_radius,
                width: 1.0,
                color: outline_variant,
            },
            shadow: elevation.get_level(elevation_level).shadow,
            ..Default::default()
        },

        MenuContainerVariant::Divider => container::Style {
            background: Some(Background::Color(outline_variant)),
            ..Default::default()
        },
    }
}
