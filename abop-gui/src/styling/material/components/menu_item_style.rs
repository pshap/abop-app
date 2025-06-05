//! Material Design 3 Menu Item Styling System
//!
//! This module provides a centralized styling system for all menu item buttons,
//! eliminating code duplication across MaterialMenu, MaterialSelectMenu, and MaterialAutocomplete.

use iced::{
    Element, Length, Padding, Theme,
    widget::{Row, Space, Text, text},
};

use crate::styling::color_utils::ColorUtils;
use crate::styling::material::MaterialTokens;
use crate::styling::material::components::widgets::{MaterialButton, MaterialButtonVariant};

/// Menu item variants for different styling states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItemVariant {
    /// Standard menu item with normal styling
    Normal,
    /// Selected/active menu item with highlighted background
    Selected,
    /// Header item with subdued styling (non-interactive)
    Header,
    /// Disabled menu item with reduced opacity
    Disabled,
    /// Destructive action item (delete, remove, etc.)
    Destructive,
    /// Divider item for visual separation
    Divider,
}

/// Helper function to create styled menu item button
/// Replaces duplicated button creation code across all menu components
pub fn create_menu_item_button<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    variant: MenuItemVariant,
    on_press: Option<Message>,
    tokens: &'a MaterialTokens,
) -> MaterialButton<'a, Message> {
    // Convert the menu item variant to an appropriate material button variant
    let (button_variant, is_disabled) = match variant {
        // Selected items should use the secondary container color scheme
        MenuItemVariant::Selected => (MaterialButtonVariant::FilledTonal, false),

        // Normal items should be subtle (text variant)
        MenuItemVariant::Normal => (MaterialButtonVariant::Text, false),

        // Header items are non-interactive text
        MenuItemVariant::Header => (MaterialButtonVariant::Text, true),

        // Disabled items use the standard disabled state
        MenuItemVariant::Disabled => (MaterialButtonVariant::Text, true),

        // Destructive items use text button
        MenuItemVariant::Destructive => (MaterialButtonVariant::Text, false),

        // Dividers use text button
        MenuItemVariant::Divider => (MaterialButtonVariant::Text, true),
    };

    // Create the Material button with proper content
    let mut button = MaterialButton::new_with_content(content, tokens)
        .variant(button_variant)
        .width(Length::Fill)
        .padding(Padding::from([12, 16])); // Standard menu item padding

    // Apply any optional message/on_press callback
    if let Some(message) = on_press
        && !is_disabled
    {
        button = button.on_press(message);
    }

    // Disable the button if needed
    if is_disabled {
        button = button.disabled();
    }

    button
}

/// Helper function to create menu item with icon and text
/// Provides consistent layout for icon + text menu items
#[must_use]
pub fn create_icon_text_menu_item<'a, Message: Clone + 'a>(
    icon: Option<&str>,
    text: &str,
    trailing_icon: Option<&str>,
    variant: MenuItemVariant,
    on_press: Option<Message>,
    tokens: &'a MaterialTokens,
) -> Element<'a, Message> {
    let colors = &tokens.colors;

    let text_color = match variant {
        MenuItemVariant::Selected => colors.on_secondary_container,
        MenuItemVariant::Header => colors.on_surface_variant,
        MenuItemVariant::Disabled => ColorUtils::with_alpha(colors.on_surface, 0.38),
        MenuItemVariant::Destructive => colors.error.base,
        _ => colors.on_surface,
    };

    let mut row = Row::new().spacing(12).align_y(iced::Alignment::Center);

    // Add leading icon if present
    if let Some(icon) = icon {
        let icon_text = icon.to_string();
        row = row.push(
            Text::new(icon_text)
                .size(18)
                .style(move |_theme: &Theme| text::Style {
                    color: Some(text_color),
                }),
        );
    }

    // Add main text
    let main_text = text.to_string();
    row = row.push(
        Text::new(main_text)
            .size(14)
            .style(move |_theme: &Theme| text::Style {
                color: Some(text_color),
            }),
    );

    // Add spacer and trailing icon if present
    if let Some(trailing_icon) = trailing_icon {
        row = row.push(Space::with_width(Length::Fill));
        let trailing_text = trailing_icon.to_string();
        row = row.push(
            Text::new(trailing_text)
                .size(18)
                .style(move |_theme: &Theme| text::Style {
                    color: Some(colors.on_surface_variant),
                }),
        );
    }

    create_menu_item_button(row, variant, on_press, tokens).into()
}

/// Helper function to create simple text menu item
/// Provides consistent styling for text-only menu items
pub fn create_text_menu_item<'a, Message: Clone + 'a>(
    text: &str,
    variant: MenuItemVariant,
    on_press: Option<Message>,
    tokens: &'a MaterialTokens,
) -> Element<'a, Message> {
    let colors = &tokens.colors;

    let text_color = match variant {
        MenuItemVariant::Selected => colors.on_secondary_container,
        MenuItemVariant::Header => colors.on_surface_variant,
        MenuItemVariant::Disabled => ColorUtils::with_alpha(colors.on_surface, 0.38),
        MenuItemVariant::Destructive => colors.error.base,
        _ => colors.on_surface,
    };

    let text_string = text.to_string();
    let content = Text::new(text_string)
        .size(14)
        .style(move |_theme: &Theme| text::Style {
            color: Some(text_color),
        });

    create_menu_item_button(content, variant, on_press, tokens).into()
}
