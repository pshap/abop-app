//! Core button styling functions
//!
//! This module provides the main API functions that other components use
//! to create button styles, handle sizing, and generate icons.

use iced::{Background, Border, Color, Length, Padding, Shadow, Theme, widget::button};
use iced_font_awesome::fa_icon_solid;

use super::strategy::ButtonState;
use super::variants::{ButtonSizeVariant, ButtonStyleVariant};
use crate::styling::material::MaterialTokens;

/// Internal styling structure for button state management
#[derive(Debug, Clone)]
pub struct ButtonStyling {
    /// The background for the button in its default state.
    pub background: Option<Background>,
    /// The text color for the button in its default state.
    pub text_color: Color,
    /// The border for the button in its default state.
    pub border: Border,
    /// The shadow for the button in its default state.
    pub shadow: Shadow,
    /// The background for the button when hovered.
    pub hover_background: Option<Background>,
    /// The text color for the button when hovered.
    pub hover_text_color: Option<Color>,
    /// The background for the button when pressed.
    pub pressed_background: Option<Background>,
    /// The text color for the button when pressed.
    pub pressed_text_color: Option<Color>,
    /// The background for the button when disabled.
    pub disabled_background: Option<Background>,
    /// The text color for the button when disabled.
    pub disabled_text_color: Option<Color>,
}

/// Creates a button style function for the specified variant
///
/// This function replaces the duplicated styling patterns found in
/// the `MaterialButton` implementation's get_*_styling methods.
///
/// # Arguments
/// * `variant` - The button style variant to create
/// * `tokens` - Material Design tokens for consistent styling
///
/// # Returns
/// A style function that can be used with Iced button components
pub fn create_button_style(
    variant: ButtonStyleVariant,
    tokens: &MaterialTokens,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    let styling = get_button_styling(variant, tokens);

    move |_theme: &Theme, status: button::Status| -> button::Style {
        let mut style = button::Style {
            background: styling.background,
            text_color: styling.text_color,
            border: styling.border,
            shadow: styling.shadow,
        };

        // Apply state-specific styling
        match status {
            button::Status::Hovered => {
                if let Some(hover_bg) = styling.hover_background {
                    style.background = Some(hover_bg);
                }
                if let Some(hover_text) = styling.hover_text_color {
                    style.text_color = hover_text;
                }
            }
            button::Status::Pressed => {
                if let Some(pressed_bg) = styling.pressed_background {
                    style.background = Some(pressed_bg);
                }
                if let Some(pressed_text) = styling.pressed_text_color {
                    style.text_color = pressed_text;
                }
            }
            button::Status::Disabled => {
                if let Some(disabled_bg) = styling.disabled_background {
                    style.background = Some(disabled_bg);
                }
                if let Some(disabled_text) = styling.disabled_text_color {
                    style.text_color = disabled_text;
                }
            }
            button::Status::Active => {
                // Use default styling for active state
            }
        }

        style
    }
}

/// Gets Material Design styling for the specified button variant
///
/// Centralizes the styling logic that was previously duplicated across
/// multiple get_*_styling methods in `MaterialButton`.
///
/// # Arguments
/// * `variant` - The button variant to style
/// * `tokens` - Material Design tokens for consistent styling
///
/// # Returns
/// `ButtonStyling` struct with all state variations configured
#[must_use]
pub fn get_button_styling(variant: ButtonStyleVariant, tokens: &MaterialTokens) -> ButtonStyling {
    let strategy = variant.get_strategy();

    // Get styling for default state and convert to the expected format
    let default_styling = strategy.get_styling(
        ButtonState::Default,
        tokens,
        &tokens.colors,
        &tokens.elevation,
        &tokens.shapes,
    );

    let hover_styling = strategy.get_styling(
        ButtonState::Hovered,
        tokens,
        &tokens.colors,
        &tokens.elevation,
        &tokens.shapes,
    );

    let pressed_styling = strategy.get_styling(
        ButtonState::Pressed,
        tokens,
        &tokens.colors,
        &tokens.elevation,
        &tokens.shapes,
    );

    let disabled_styling = strategy.get_styling(
        ButtonState::Disabled,
        tokens,
        &tokens.colors,
        &tokens.elevation,
        &tokens.shapes,
    );

    // Convert to the expected ButtonStyling format
    ButtonStyling {
        background: Some(default_styling.background),
        text_color: default_styling.text_color,
        border: default_styling.border,
        shadow: default_styling.shadow.unwrap_or_default(),
        hover_background: Some(hover_styling.background),
        hover_text_color: Some(hover_styling.text_color),
        pressed_background: Some(pressed_styling.background),
        pressed_text_color: Some(pressed_styling.text_color),
        disabled_background: Some(disabled_styling.background),
        disabled_text_color: Some(disabled_styling.text_color),
    }
}

/// Gets size-specific properties for button variants
///
/// Centralizes the size logic that was previously in `MaterialButton`'s
/// `get_size_properties` method.
///
/// # Arguments
/// * `variant` - The button variant
/// * `size` - The button size
/// * `tokens` - Material Design tokens for sizing
///
/// # Returns
/// Tuple of (width, height, padding) for the button
#[must_use]
pub fn get_button_size_properties(
    variant: ButtonStyleVariant,
    size: ButtonSizeVariant,
    tokens: &MaterialTokens,
) -> (Length, Length, Padding) {
    use crate::design_tokens::constants::sizing;
    use crate::design_tokens::constants::spacing;

    match (variant, size) {
        // Standard buttons with minimum width to avoid circular appearance
        (
            ButtonStyleVariant::Filled
            | ButtonStyleVariant::FilledTonal
            | ButtonStyleVariant::Outlined
            | ButtonStyleVariant::Text
            | ButtonStyleVariant::Elevated,
            ButtonSizeVariant::Small,
        ) => (
            Length::Fixed(88.0), // Minimum width for proper button appearance
            Length::Fixed(sizing::BUTTON_HEIGHT_SM),
            Padding::from([tokens.spacing().md, tokens.spacing().xs]),
        ),
        (
            ButtonStyleVariant::Filled
            | ButtonStyleVariant::FilledTonal
            | ButtonStyleVariant::Outlined
            | ButtonStyleVariant::Text
            | ButtonStyleVariant::Elevated,
            ButtonSizeVariant::Medium,
        ) => (
            Length::Fixed(100.0), // Slightly larger for medium size
            Length::Fixed(sizing::BUTTON_HEIGHT),
            Padding::from([tokens.spacing().lg, tokens.spacing().md]),
        ),
        (
            ButtonStyleVariant::Filled
            | ButtonStyleVariant::FilledTonal
            | ButtonStyleVariant::Outlined
            | ButtonStyleVariant::Text
            | ButtonStyleVariant::Elevated,
            ButtonSizeVariant::Large,
        ) => (
            Length::Fixed(120.0), // Larger minimum width for large buttons
            Length::Fixed(sizing::BUTTON_HEIGHT_LG),
            Padding::from([tokens.spacing().xl, tokens.spacing().md]),
        ),
        // Icon buttons and FAB buttons
        (ButtonStyleVariant::Icon | ButtonStyleVariant::Fab, ButtonSizeVariant::Small) => (
            Length::Fixed(sizing::ICON_BUTTON_SM),
            Length::Fixed(sizing::ICON_BUTTON_SM),
            Padding::new(spacing::SM),
        ),
        (ButtonStyleVariant::Icon, ButtonSizeVariant::Medium) => (
            Length::Fixed(sizing::ICON_BUTTON_MD),
            Length::Fixed(sizing::ICON_BUTTON_MD),
            Padding::new(spacing::SM + 4.0),
        ),
        (ButtonStyleVariant::Icon, ButtonSizeVariant::Large)
        | (ButtonStyleVariant::Fab, ButtonSizeVariant::Medium) => (
            Length::Fixed(sizing::ICON_BUTTON_LG),
            Length::Fixed(sizing::ICON_BUTTON_LG),
            Padding::new(spacing::MD),
        ),
        // FAB Large size (unique)
        (ButtonStyleVariant::Fab, ButtonSizeVariant::Large) => (
            Length::Fixed(96.0),
            Length::Fixed(96.0),
            Padding::new(spacing::XL - 2.0),
        ),
    }
}

/// Gets the appropriate icon size for a button based on its variant and size
///
/// This function centralizes icon sizing decisions to ensure consistency across
/// all button types and compliance with Material Design specifications.
///
/// # Arguments
/// * `variant` - The button style variant
/// * `size` - The button size variant
///
/// # Returns
/// The appropriate icon size in pixels as f32
#[must_use]
pub const fn get_icon_size_for_button(variant: ButtonStyleVariant, size: ButtonSizeVariant) -> f32 {
    use crate::design_tokens::constants::sizing;

    match variant {
        // Icon buttons and FABs use larger icons as they are icon-only
        ButtonStyleVariant::Icon | ButtonStyleVariant::Fab => match size {
            ButtonSizeVariant::Small => sizing::ICON_MD, // 20px for 40px container
            ButtonSizeVariant::Medium => sizing::ICON_LG, // 24px for 48px/56px container
            ButtonSizeVariant::Large => 28.0,            // 28px for 96px FAB large
        },
        // Regular buttons with icons use smaller icons to balance with text
        ButtonStyleVariant::Filled
        | ButtonStyleVariant::FilledTonal
        | ButtonStyleVariant::Outlined
        | ButtonStyleVariant::Text
        | ButtonStyleVariant::Elevated => match size {
            ButtonSizeVariant::Small => sizing::ICON_SM, // 16px for 32px height
            ButtonSizeVariant::Medium => 18.0,           // 18px for 40px height (MD3 standard)
            ButtonSizeVariant::Large => sizing::ICON_MD, // 20px for 48px height
        },
    }
}

/// Creates an icon element with appropriate sizing for the given button context
///
/// This function replaces manual icon sizing by automatically determining the
/// correct icon size based on button variant and size context.
///
/// # Arguments
/// * `icon_name` - The Font Awesome icon name
/// * `variant` - The button style variant
/// * `size` - The button size variant
///
/// # Returns
/// An icon Element with the correct size for the button context
#[must_use]
pub fn create_button_icon<'a, Message>(
    icon_name: &str,
    variant: ButtonStyleVariant,
    size: ButtonSizeVariant,
) -> iced::Element<'a, Message> {
    let icon_size = get_icon_size_for_button(variant, size);
    fa_icon_solid(icon_name).size(icon_size).into()
}
