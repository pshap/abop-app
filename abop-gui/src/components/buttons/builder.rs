//! A fluent builder for creating Material Design 3 buttons with various styles and configurations.
//!
//! This module provides a type-safe builder API for creating buttons with different variants,
//! sizes, and states. It handles the complexity of Material Design 3 button theming and layout.
//!
//! # Examples
//!
//! ```no_run
//! use abop_gui::components::buttons::{self, ButtonVariant, IconPosition};
//! use abop_gui::styling::material::MaterialTokens;
//! use iced::Length;
//!
//! # fn example(tokens: &MaterialTokens) -> iced::Element<'static, ()> {
//! // Create a primary button with an icon
//! let button = buttons::button(tokens)
//!     .label("Save")
//!     .icon("save", IconPosition::Leading)
//!     .variant(ButtonVariant::Filled)
//!     .width(Length::Fixed(200.0))
//!     .on_press(())
//!     .build()
//!     .expect("Failed to build button");
//! # button
//! # }
//! ```

use crate::styling::material::MaterialButton;
use crate::styling::material::MaterialTokens;
use iced::{
    Alignment, Element, Length, Padding,
    widget::{Row, container, text},
};

use super::{
    error::{ButtonError, ButtonResult},
    icons::IconConfig,
    variants::{ButtonSize, ButtonVariant, IconPosition},
};

/// Default spacing between icon and text elements in pixels
const ICON_TEXT_SPACING: f32 = 8.0;

/// Macro to apply common container properties (width, height, padding)
/// Uses method chaining to avoid intermediate assignments when possible
macro_rules! apply_container_properties {
    ($container:expr, $width:expr, $height:expr, $padding:expr) => {{
        let result = $container;
        let result = if let Some(w) = $width { result.width(w) } else { result };
        let result = if let Some(h) = $height { result.height(h) } else { result };
        if let Some(p) = $padding { result.padding(p) } else { result }
    }};
}

/// A builder for creating Material Design 3 buttons with a fluent API.
///
/// # Examples
///
/// ```no_run
/// use abop_gui::components::buttons::{self, ButtonVariant, IconPosition};
/// use abop_gui::styling::material::MaterialTokens;
///
/// # fn example(tokens: &MaterialTokens) -> iced::Element<'static, ()> {
/// // Create a primary button with an icon
/// let button = buttons::button(tokens)
///     .label("Save")
///     .icon("save", IconPosition::Leading)
///     .variant(ButtonVariant::Filled)
///     .on_press(())
///     .build()
///     .expect("Failed to build button");
/// # button
/// # }
/// ```
pub struct ButtonBuilder<'a, M: Clone + 'a> {
    /// The Material Design tokens for theming
    tokens: &'a MaterialTokens,

    /// The button's label text
    label: Option<&'a str>,

    /// The button's icon configuration
    icon: Option<IconConfig<'a>>,

    /// The visual variant of the button
    variant: ButtonVariant,

    /// The size of the button
    size: ButtonSize,

    /// The message to send when the button is pressed
    on_press: Option<M>,

    /// Whether the button is disabled
    disabled: bool,

    /// The button's width
    width: Option<Length>,

    /// The button's height
    height: Option<Length>,

    /// Custom padding for the button
    padding: Option<Padding>,
}

impl<'a, M: Clone + 'a> ButtonBuilder<'a, M> {
    /// Create a new button builder with default settings
    pub fn new(tokens: &'a MaterialTokens) -> Self {
        Self {
            tokens,
            label: None,
            icon: None,
            variant: ButtonVariant::Filled,
            size: ButtonSize::Medium,
            width: None,
            height: None,
            padding: None,
            disabled: false,
            on_press: None,
        }
    }

    /// Set the button's label text
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the button's icon
    pub fn icon(mut self, icon_name: &'a str, position: IconPosition) -> Self {
        self.icon = Some(IconConfig::new(
            icon_name,
            super::variants::icon_size(self.size),
            position,
        ));
        self
    }

    /// Set the button to be an icon-only button
    pub fn icon_only(mut self, icon_name: &'a str, size: ButtonSize) -> Self {
        self.size = size;
        self.icon = Some(IconConfig::new(
            icon_name,
            super::variants::icon_size(size),
            IconPosition::Only,
        ));
        self
    }

    /// Set the button's visual variant
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the button's size
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        // Update icon size if an icon is already set
        if let Some(icon) = &mut self.icon {
            icon.size = super::variants::icon_size(size);
        }
        self
    }

    /// Set the button's width
    pub fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the button's height
    pub fn height(mut self, height: Length) -> Self {
        self.height = Some(height);
        self
    }

    /// Set the button's padding
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Set the button to be disabled
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }    /// Set the message to send when the button is pressed
    pub fn on_press(mut self, message: M) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Build the button element
    ///
    /// # Errors
    ///
    /// Returns `ButtonError` if:
    /// - The button has neither a label nor an icon
    /// - The button is enabled but no `on_press` handler is provided
    /// - The button has an invalid configuration (e.g., IconPosition::Only with both label and icon)
    pub fn build(self) -> ButtonResult<Element<'a, M>> {
        // Validate button configuration - check for both None and empty string
        let has_label = self.label.is_some_and(|label| !label.is_empty());
        let has_icon = self.icon.is_some();

        if !has_label && !has_icon {
            return Err(ButtonError::MissingLabelAndIcon);
        }

        if !self.disabled && self.on_press.is_none() {
            return Err(ButtonError::MissingOnPress);
        } // Validate icon position early
        if let (Some(label), Some(icon)) = (&self.label, &self.icon)
            && icon.position == IconPosition::Only
            && !label.is_empty()
        {
            return Err(ButtonError::InvalidIconPosition);
        }        // Create the button content based on what's available (icon, label, or both)
        let label = self.label;
        let icon = self.icon;
        let width = self.width;
        let height = self.height;
        let padding = self.padding;
        
        let content: Element<'a, M> = match (label, icon) {
            (Some(label), Some(icon)) => {
                // Button with both icon and label
                let icon_element: Element<'a, M> = container(icon.to_element::<M>())
                    .width(icon.size.value())
                    .height(icon.size.value())
                    .into();

                let text_element: Element<'a, M> = text(label).into();

                // Create a row with icon and text in the correct order based on position
                let row: Element<'a, M> = match icon.position {
                    IconPosition::Leading => Row::new()
                        .spacing(ICON_TEXT_SPACING)
                        .push(icon_element)
                        .push(text_element)
                        .align_y(Alignment::Center)
                        .into(),
                    IconPosition::Trailing => Row::new()
                        .spacing(ICON_TEXT_SPACING)
                        .push(text_element)
                        .push(icon_element)
                        .align_y(Alignment::Center)
                        .into(),
                    IconPosition::Only => {
                        // This case is prevented by the early validation in the build() method
                        return Err(ButtonError::InvalidIconPosition);
                    }
                };

                // Apply sizing and padding using the macro
                apply_container_properties!(container(row), width, height, padding).into()
            }
            (Some(label), None) => {
                // Button with label only
                let text_widget: Element<'a, M> = text(label).into();

                // Create a container for the text and apply properties using the macro
                let container_with_alignment = container(text_widget)
                    .width(Length::Shrink) // Size to content for proper button layout
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center);
                
                apply_container_properties!(container_with_alignment, width, height, padding).into()
            }
            (None, Some(icon)) => {
                // Icon-only button
                let icon_element = icon.to_element::<M>();

                // Create a container for the icon and apply properties using the macro
                let container_with_alignment = container(icon_element)
                    .height(icon.size.value())
                    .width(Length::Shrink) // Size to content for proper button layout
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center);
                
                apply_container_properties!(container_with_alignment, width, height, padding).into()
            }
            (None, None) => unreachable!(), // Already validated above
        };

        // Create the button with the content
        let mut button =
            MaterialButton::new_with_content(content, self.tokens).variant(self.variant.into());

        if self.disabled {
            button = button.disabled();
        }

        if let Some(on_press) = self.on_press {
            button = button.on_press(on_press);
        }

        Ok(button.into())
    }
}

// Conversion from our ButtonVariant to the MaterialButtonVariant
impl From<ButtonVariant> for crate::styling::material::MaterialButtonVariant {
    fn from(variant: ButtonVariant) -> Self {
        use crate::styling::material::MaterialButtonVariant as MBV;

        match variant {
            ButtonVariant::Filled => MBV::Filled,
            ButtonVariant::FilledTonal => MBV::FilledTonal,
            ButtonVariant::Outlined => MBV::Outlined,
            ButtonVariant::Text => MBV::Text,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;

    #[derive(Debug, Clone, PartialEq)]
    enum TestMessage {
        Clicked,
    }

    #[test]
    fn test_button_builder_new() {
        let tokens = MaterialTokens::default();
        let builder = ButtonBuilder::<TestMessage>::new(&tokens);

        assert!(builder.label.is_none());
        assert!(builder.icon.is_none());
        assert_eq!(builder.variant, ButtonVariant::Filled);
        assert_eq!(builder.size, ButtonSize::Medium);
        assert!(builder.on_press.is_none());
        assert!(!builder.disabled);
    }

    #[test]
    fn test_button_builder_fluent() {
        let tokens = MaterialTokens::default();

        let builder = ButtonBuilder::<TestMessage>::new(&tokens)
            .label("Test")
            .icon("test", IconPosition::Leading)
            .variant(ButtonVariant::Outlined)
            .size(ButtonSize::Large)
            .on_press(TestMessage::Clicked)
            .width(Length::Fixed(200.0))
            .height(Length::Fixed(50.0))
            .padding(Padding::from(10.0));

        assert_eq!(builder.label, Some("Test"));
        assert!(builder.icon.is_some());
        assert_eq!(builder.variant, ButtonVariant::Outlined);
        assert_eq!(builder.size, ButtonSize::Large);
        assert!(matches!(builder.on_press, Some(TestMessage::Clicked)));
        assert_eq!(builder.width, Some(Length::Fixed(200.0)));
        assert_eq!(builder.height, Some(Length::Fixed(50.0)));
        assert_eq!(builder.padding, Some(Padding::from(10.0)));
    }

    #[test]
    fn test_build_valid_button() {
        let tokens = MaterialTokens::default();

        // Test with label only
        let result = ButtonBuilder::new(&tokens)
            .label("Test")
            .on_press(TestMessage::Clicked)
            .build();

        assert!(result.is_ok());

        // Test with icon only
        let result = ButtonBuilder::new(&tokens)
            .icon_only("favorite", ButtonSize::Medium)
            .on_press(TestMessage::Clicked)
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_build_missing_label_and_icon() {
        let tokens = MaterialTokens::default();

        let result = ButtonBuilder::<TestMessage>::new(&tokens)
            .on_press(TestMessage::Clicked)
            .build();

        assert!(matches!(result, Err(ButtonError::MissingLabelAndIcon)));

        // Test with empty label
        let result = ButtonBuilder::new(&tokens)
            .label("")
            .on_press(TestMessage::Clicked)
            .build();

        assert!(matches!(result, Err(ButtonError::MissingLabelAndIcon)));
    }

    #[test]
    fn test_build_missing_on_press() {
        let tokens = MaterialTokens::default();

        // Test with label but no on_press
        let result = ButtonBuilder::<TestMessage>::new(&tokens)
            .label("Test")
            .build();

        assert!(matches!(result, Err(ButtonError::MissingOnPress)));

        // Test with icon but no on_press
        let result = ButtonBuilder::<TestMessage>::new(&tokens)
            .icon_only("favorite", ButtonSize::Medium)
            .build();

        assert!(matches!(result, Err(ButtonError::MissingOnPress)));
    }

    #[test]
    fn test_build_disabled_no_on_press() {
        let tokens = MaterialTokens::default();

        // Should not require on_press when disabled (with label)
        let result = ButtonBuilder::<TestMessage>::new(&tokens)
            .label("Disabled")
            .disabled()
            .build();

        assert!(result.is_ok());

        // Should not require on_press when disabled (with icon)
        let result = ButtonBuilder::<TestMessage>::new(&tokens)
            .icon_only("favorite", ButtonSize::Medium)
            .disabled()
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_icon_positioning() {
        let tokens = MaterialTokens::default();

        // Test leading icon position
        let result = ButtonBuilder::new(&tokens)
            .label("Test")
            .icon("favorite", IconPosition::Leading)
            .on_press(TestMessage::Clicked)
            .build();

        assert!(result.is_ok());

        // Test trailing icon position
        let result = ButtonBuilder::new(&tokens)
            .label("Test")
            .icon("favorite", IconPosition::Trailing)
            .on_press(TestMessage::Clicked)
            .build();

        assert!(result.is_ok());

        // Test icon-only (using icon_only method)
        let result = ButtonBuilder::new(&tokens)
            .icon_only("favorite", ButtonSize::Medium)
            .on_press(TestMessage::Clicked)
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_icon_position() {
        let tokens = MaterialTokens::default();

        // Test setting IconPosition::Only when label is already present
        let result = ButtonBuilder::new(&tokens)
            .label("Test")
            .icon("favorite", IconPosition::Only)
            .on_press(TestMessage::Clicked)
            .build();

        assert!(matches!(result, Err(ButtonError::InvalidIconPosition)));
    }
}
