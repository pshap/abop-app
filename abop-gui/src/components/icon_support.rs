//! Icon support for Material Design 3 button widgets
//!
//! This module provides functionality to properly integrate Font Awesome icons
//! with the Material Design 3 button widget system.

use iced::widget::{Space, container, row, text};
use iced::{Alignment, Element, Length};
use iced_font_awesome::fa_icon_solid;

use crate::styling::material::MaterialTokens;
use crate::styling::material::components::button_style::{ButtonSizeVariant, ButtonStyleVariant};
use crate::styling::material::components::widgets::{IconPosition, MaterialButtonVariant};

/// Creates button content with an icon positioned relative to text
///
/// # Arguments
/// * `label` - The button text
/// * `icon_name` - The icon name to use from Font Awesome
/// * `icon_position` - Position of the icon relative to the text
/// * `variant` - The button variant (determines the icon size)
/// * `tokens` - Material design tokens for styling
#[must_use]
pub fn create_button_with_icon_content<'a, Message: 'a>(
    label: &'a str,
    icon_name: &'a str,
    icon_position: IconPosition,
    variant: MaterialButtonVariant,
    tokens: &'a MaterialTokens,
) -> Element<'a, Message> {
    let button_style_variant = map_button_variant(variant);

    // Default to medium size
    let size_variant = ButtonSizeVariant::Medium;

    // Get appropriate icon size based on button variant
    let icon_size =
        crate::styling::material::components::button_style::functions::get_icon_size_for_button(
            button_style_variant,
            size_variant,
        );
    // Create the icon element
    let icon = fa_icon_solid(icon_name).size(icon_size);

    let label_style = &tokens.typography().label_large;
    let text_element = text(label).size(label_style.size);

    // Create content based on icon position
    match icon_position {
        IconPosition::Leading => row![
            icon,
            Space::new(Length::Fixed(8.0), Length::Shrink),
            text_element
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .into(),
        IconPosition::Trailing => row![
            text_element,
            Space::new(Length::Fixed(8.0), Length::Shrink),
            icon
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .into(),
        IconPosition::Only => container(icon)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .width(Length::Shrink)
            .height(Length::Shrink)
            .into(),
    }
}

/// Creates an icon-only button content
///
/// # Arguments
/// * `icon_name` - The icon name to use from Font Awesome
/// * `variant` - The button variant (determines the icon size)
/// * `size` - The button size variant
/// * `tokens` - Material design tokens for styling
#[must_use]
pub fn create_icon_button_content<'a, Message: 'a>(
    icon_name: &'a str,
    variant: MaterialButtonVariant,
    size_variant: ButtonSizeVariant,
    _tokens: &'a MaterialTokens,
) -> Element<'a, Message> {
    let button_style_variant = map_button_variant(variant);

    // Get appropriate icon size based on button variant and size
    let icon_size =
        crate::styling::material::components::button_style::functions::get_icon_size_for_button(
            button_style_variant,
            size_variant,
        );

    // Create the icon element with proper size
    let icon_element = fa_icon_solid(icon_name).size(icon_size);
    // Create centered icon content
    container(icon_element)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .into()
}

/// Map from widget button variant to style button variant
const fn map_button_variant(variant: MaterialButtonVariant) -> ButtonStyleVariant {
    match variant {
        MaterialButtonVariant::Filled => ButtonStyleVariant::Filled,
        MaterialButtonVariant::FilledTonal => ButtonStyleVariant::FilledTonal,
        MaterialButtonVariant::Outlined => ButtonStyleVariant::Outlined,
        MaterialButtonVariant::Text => ButtonStyleVariant::Text,
        MaterialButtonVariant::Elevated => ButtonStyleVariant::Elevated,
    }
}

/// Map from old button size to style button size variant
#[must_use]
pub const fn map_button_size(
    size: crate::styling::material::components::widgets::ButtonSize,
) -> ButtonSizeVariant {
    match size {
        crate::styling::material::components::widgets::ButtonSize::Small => {
            ButtonSizeVariant::Small
        }
        crate::styling::material::components::widgets::ButtonSize::Medium => {
            ButtonSizeVariant::Medium
        }
        crate::styling::material::components::widgets::ButtonSize::Large => {
            ButtonSizeVariant::Large
        }
    }
}
