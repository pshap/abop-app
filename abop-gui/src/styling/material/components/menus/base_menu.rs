//! Base menu traits and shared functionality
//!
//! This module provides common traits and shared functionality for all menu components.

use iced::{
    Element, Length, Padding, Theme,
    widget::{Column, Space, button, container, scrollable},
};

use crate::styling::material::components::menu_container_style::{
    MenuContainerVariant, create_menu_container_style_custom,
};
use crate::styling::material::{MaterialTokens, elevation::ElevationLevel};

/// Type alias for complex button style function
pub type ButtonStyleFn<'a> = Box<dyn Fn(&Theme, button::Status) -> button::Style + 'a>;

/// Trait for common builder pattern methods shared across menu components
pub trait MenuComponent {
    /// Set whether this component is enabled
    fn enabled(self, enabled: bool) -> Self;
}

/// Trait for components that can be opened and closed
pub trait Openable {
    /// Set whether the component is open
    fn open(self, open: bool) -> Self;

    /// Check if the component is currently open
    fn is_open(&self) -> bool;
}

/// Trait for components that can have a minimum width
pub trait HasMinWidth {
    /// Set the minimum width
    fn min_width(self, width: f32) -> Self;

    /// Get the current minimum width
    fn get_min_width(&self) -> Option<f32>;
}

/// Trait for components that can have a maximum height
pub trait HasMaxHeight {
    /// Set the maximum height
    fn max_height(self, height: f32) -> Self;

    /// Get the current maximum height
    fn get_max_height(&self) -> Option<f32>;
}

/// Trait for components that can have an elevation level
pub trait HasElevation {
    /// Set the elevation level (0-5)
    fn elevation(self, level: u8) -> Self;

    /// Get the current elevation level
    fn get_elevation(&self) -> u8;

    /// Get the elevation level as an `ElevationLevel` enum
    fn get_elevation_level(&self) -> ElevationLevel {
        ElevationLevel::from_u8(self.get_elevation()).unwrap_or(ElevationLevel::Level1)
    }
}

/// Helper function to create a scrollable menu container with proper styling
#[must_use]
pub fn create_menu_container<'a, Message: 'a + Clone>(
    content: Column<'a, Message>,
    variant: MenuContainerVariant,
    min_width: Option<f32>,
    max_height: Option<f32>,
    shadow_offset_y: f32,
    blur_radius: f32,
    tokens: &'a MaterialTokens,
) -> Element<'a, Message> {
    let mut menu_content = scrollable(content)
        .width(Length::Fill)
        .height(Length::Shrink);

    if let Some(max_height) = max_height {
        menu_content = menu_content.height(Length::Fixed(max_height));
    }

    let mut menu_container = container(menu_content)
        .style(create_menu_container_style_custom(
            variant,
            shadow_offset_y,
            blur_radius,
            tokens,
        ))
        .padding(Padding::from([8, 0]));

    if let Some(min_width) = min_width {
        menu_container = menu_container.width(Length::Fixed(min_width));
    }

    menu_container.into()
}

/// Helper function to extract shadow properties from elevation level
#[must_use]
pub const fn get_shadow_properties(
    elevation_level: ElevationLevel,
    tokens: &MaterialTokens,
) -> (f32, f32) {
    let shadow = tokens.elevation.get_level(elevation_level).shadow;
    (shadow.offset.y, shadow.blur_radius)
}

/// Helper function to create a placeholder element when menu is closed
#[must_use]
pub fn create_empty_element<'a, Message: 'a>() -> Element<'a, Message> {
    Space::with_width(Length::Shrink).into()
}
