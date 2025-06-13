//! Builder patterns for creating complex UI components

use iced::{Element, Length, Padding};
use thiserror::Error;

use crate::components::buttons::variants::{IconPosition as CustomIconPosition, ButtonSize as CustomButtonSize};
use crate::styling::material::MaterialTokens;
use crate::styling::material::components::widgets::material_button::MaterialButtonVariant;
use crate::styling::material::components::widgets::MaterialButton;
use crate::styling::material::components::widgets::{
    ButtonSize as MaterialButtonSize, 
    IconPosition as MaterialIconPosition
};


/// Errors that can occur during button building
#[derive(Debug, Error, Clone)]
pub enum ButtonBuildError {
    /// Occurs when a required field is missing
    #[error("Missing required field: {0}")]
    MissingField(&'static str),

    /// Occurs when there's a conflict in button configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(&'static str),
}

use crate::components::icon_support;

/// Advanced button builder for complex configurations
///
/// This builder provides a fluent interface for creating Material Design 3 buttons
/// with various options while maintaining type safety and reducing code duplication.
pub struct ButtonBuilder<'a, M: Clone + 'a> {
    label: Option<&'a str>,
    icon_name: Option<&'a str>,
    icon_position: CustomIconPosition,
    variant: MaterialButtonVariant,
    size: Option<MaterialButtonSize>,
    width: Option<Length>,
    height: Option<Length>,
    padding: Option<Padding>,
    disabled: bool,
    tokens: &'a MaterialTokens,
    on_press: Option<M>,
}

impl<'a, M: Clone + 'a> ButtonBuilder<'a, M> {
    /// Create a new button builder with the given tokens
    pub const fn new(tokens: &'a MaterialTokens) -> Self {
        Self {
            label: None,
            icon_name: None,
            icon_position: CustomIconPosition::Leading,
            variant: MaterialButtonVariant::Filled,
            size: None,
            width: None,
            height: None,
            padding: None,
            disabled: false,
            tokens,
            on_press: None,
        }
    }

    /// Set the button label text
    pub const fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the icon name and position
    pub fn icon(mut self, icon_name: &'a str, position: impl Into<CustomIconPosition>) -> Self {
        self.icon_name = Some(icon_name);
        self.icon_position = position.into();
        self
    }

    /// Set the Material Design variant
    pub const fn variant(mut self, variant: MaterialButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the button size
    pub fn size(mut self, size: impl Into<MaterialButtonSize>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Set custom width
    pub const fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }

    /// Set custom height
    pub const fn height(mut self, height: Length) -> Self {
        self.height = Some(height);
        self
    }

    /// Set custom padding
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Make the button disabled
    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Set the on_press handler
    pub fn on_press(mut self, message: M) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Build the button element
    ///
    /// # Errors
    ///
    /// Returns `ButtonBuildError` if:
    /// - Button has neither label nor icon
    /// - Button is enabled but no `on_press` handler is provided
    /// - Both `on_press` and `disabled` are set
    pub fn build(self) -> Result<Element<'a, M>, ButtonBuildError> {
        // Validate button configuration
        if self.label.is_none() && self.icon_name.is_none() {
            return Err(ButtonBuildError::MissingField("label or icon"));
        }

        if !self.disabled && self.on_press.is_none() {
            return Err(ButtonBuildError::MissingField("on_press"));
        }

        if self.disabled && self.on_press.is_some() {
            return Err(ButtonBuildError::InvalidConfiguration(
                "Cannot have both `on_press` and `disabled` set",
            ));
        }

        // Create the button based on configuration
        if let Some(icon_name) = self.icon_name {
            if let Some(label) = self.label {
                // Button with both icon and label
                let button = material_button_with_icon_widget(
                    label,
                    icon_name,
                    self.icon_position.into(),
                    self.variant,
                    self.on_press.unwrap_or_else(|| {
                        panic!("Button with label '{}' and icon '{}' is enabled but has no on_press handler", label, icon_name)
                    }),
                    self.tokens,
                );

                Ok(button)
            } else {
                // Icon-only button
                let button = material_icon_button_widget(
                    icon_name,
                    self.variant,
                    self.size.unwrap_or(MaterialButtonSize::Medium),
                    self.on_press.unwrap_or_else(|| {
                        panic!("Icon button with icon '{}' is enabled but has no on_press handler", icon_name)
                    }),
                    self.tokens,
                );

                Ok(button)
            }
        } else if let Some(label) = self.label {
            // Text-only button
            let mut btn = MaterialButton::new(label, self.tokens).variant(self.variant);

            if let Some(msg) = self.on_press {
                btn = btn.on_press(msg);
            }

            if self.disabled {
                btn = btn.disabled();
            }

            // Apply custom dimensions if specified
            if let Some(width) = self.width {
                btn = btn.width(width);
            }

            if let Some(height) = self.height {
                btn = btn.height(height);
            }

            if let Some(padding) = self.padding {
                btn = btn.padding(padding);
            }

            Ok(btn.into())
        } else if let Some(icon_name) = self.icon_name {
            // Icon-only button
            if self.disabled {
                // Get the size or default to Medium
                let button_size = self.size.unwrap_or(MaterialButtonSize::Medium);

                // Create icon content using the material button size directly
                let content = icon_support::create_icon_button_content::<M>(
                    icon_name,
                    self.variant,
                    map_material_size_to_style_size(button_size),
                    self.tokens,
                );

                // Use centralized size calculation
                let button_size_px = super::sizing::button_size_to_pixels(button_size);

                // Create disabled button
                Ok(MaterialButton::new_with_content(content, self.tokens)
                    .variant(self.variant)
                    .width(Length::Fixed(button_size_px))
                    .height(Length::Fixed(button_size_px))
                    .padding(8.0)
                    .disabled()
                    .into())
            } else {
                // Create an enabled icon button
                let on_press = self.on_press.ok_or({
                    ButtonBuildError::MissingField("on_press for enabled icon button")
                })?;

                Ok(material_icon_button_widget(
                    icon_name,
                    self.variant,
                    self.size.unwrap_or(MaterialButtonSize::Medium),
                    on_press,
                    self.tokens,
                ))
            }
        } else {
            // This should be unreachable due to validation at start of method
            unreachable!("Button must have either label or icon");
        }
    }
}

/// Create a button using the advanced builder pattern
///
/// This provides a more flexible interface for complex button configurations
/// while maintaining the simplicity of the existing helper functions.
///
/// # Examples
/// ```no_run
/// use abop_gui::components::common::button_builder;
/// use abop_gui::styling::material::{MaterialTokens, MaterialButtonVariant, ButtonSize};
/// use abop_gui::styling::material::components::widgets::IconPosition;
/// use iced::Length;
///
/// #[derive(Debug, Clone)]
/// enum AppMessage {
///     Save,
///     Export,
/// }
///
/// let tokens = MaterialTokens::default();
///
/// // Simple primary button
/// let btn = button_builder(&tokens)
///     .label("Save")
///     .variant(MaterialButtonVariant::Filled)
///     .on_press(AppMessage::Save)
///     .build();
///
/// // Complex button with icon and custom sizing
/// let btn = button_builder(&tokens)
///     .label("Export")
///     .icon("download", IconPosition::Leading)
///     .variant(MaterialButtonVariant::Outlined)
///     .size(ButtonSize::Large)
///     .width(Length::Fixed(200.0))
///     .on_press(AppMessage::Export)
///     .build();
/// ```
pub const fn button_builder<'a, M: Clone + 'a>(tokens: &'a MaterialTokens) -> ButtonBuilder<'a, M> {
    ButtonBuilder::new(tokens)
}

/// Creates a semantic primary button using the builder pattern
///
/// # Deprecated
/// Use `buttons::button(tokens).label("text").variant(ButtonVariant::Filled).on_press(message).build()?` instead.
#[deprecated(
    since = "0.1.0",
    note = "Use `buttons::button(tokens).label(\"text\").variant(ButtonVariant::Filled).on_press(message).build()?` instead"
)]
pub fn primary_button_semantic<'a, M: Clone + 'a>(
    label: &'a str,
    on_press: M,
    tokens: &'a MaterialTokens,
) -> Element<'a, M> {
    crate::components::buttons::button(tokens)
        .label(label)
        .variant(crate::components::buttons::ButtonVariant::Filled)
        .on_press(on_press)
        .build()
        .unwrap_or_else(|e| {
            log::warn!("Failed to build primary button: {}", e);
            iced::widget::Text::new(label).into()
        })
}

/// Creates a semantic secondary button using the builder pattern
///
/// # Deprecated
/// Use `buttons::button(tokens).label("text").variant(ButtonVariant::Outlined).on_press(message).build()?` instead.
#[deprecated(
    since = "0.1.0",
    note = "Use `buttons::button(tokens).label(\"text\").variant(ButtonVariant::Outlined).on_press(message).build()?` instead"
)]
pub fn secondary_button_semantic<'a, M: Clone + 'a>(
    label: &'a str,
    on_press: M,
    tokens: &'a MaterialTokens,
) -> Element<'a, M> {
    crate::components::buttons::button(tokens)
        .label(label)
        .variant(crate::components::buttons::ButtonVariant::Outlined)
        .on_press(on_press)
        .build()
        .unwrap_or_else(|e| {
            log::warn!("Failed to build secondary button: {}", e);
            iced::widget::Text::new(label).into()
        })
}

/// Creates a semantic tertiary button using the builder pattern
///
/// # Deprecated
/// Use `buttons::button(tokens).label("text").variant(ButtonVariant::Text).on_press(message).build()?` instead.
#[deprecated(
    since = "0.1.0",
    note = "Use `buttons::button(tokens).label(\"text\").variant(ButtonVariant::Text).on_press(message).build()?` instead"
)]
pub fn tertiary_button<'a, M: Clone + 'a>(
    label: &'a str,
    on_press: M,
    tokens: &'a MaterialTokens,
) -> Element<'a, M> {
    crate::components::buttons::button(tokens)
        .label(label)
        .variant(crate::components::buttons::ButtonVariant::Text)
        .on_press(on_press)
        .build()
        .unwrap_or_else(|e| {
            log::warn!("Failed to build tertiary button: {}", e);
            iced::widget::Text::new(label).into()
        })
}

/// Creates a primary button with icon using the builder pattern
///
/// # Deprecated
/// Use `buttons::button(tokens).label("text").icon("icon", position).variant(ButtonVariant::Filled).on_press(message).build()?` instead.
#[deprecated(
    since = "0.1.0",
    note = "Use `buttons::button(tokens).label(\"text\").icon(\"icon\", position).variant(ButtonVariant::Filled).on_press(message).build()?` instead"
)]
pub fn primary_button_with_icon_semantic<'a, M: Clone + 'a>(
    label: &'a str,
    icon_name: &'a str,
    icon_position: CustomIconPosition,
    on_press: M,
    tokens: &'a MaterialTokens,
) -> Element<'a, M> {
    crate::components::buttons::button(tokens)
        .label(label)
        .icon(icon_name, icon_position.into())
        .variant(crate::components::buttons::ButtonVariant::Filled)
        .on_press(on_press)
        .build()
        .unwrap_or_else(|e| {
            log::warn!("Failed to build primary icon button: {}", e);
            iced::widget::Text::new(label).into()
        })
}

/// Creates a secondary button with icon using the builder pattern
///
/// # Deprecated
/// Use `buttons::button(tokens).label("text").icon("icon", position).variant(ButtonVariant::Outlined).on_press(message).build()?` instead.
#[deprecated(
    since = "0.1.0",
    note = "Use `buttons::button(tokens).label(\"text\").icon(\"icon\", position).variant(ButtonVariant::Outlined).on_press(message).build()?` instead"
)]
pub fn secondary_button_with_icon_semantic<'a, M: Clone + 'a>(
    label: &'a str,
    icon_name: &'a str,
    icon_position: CustomIconPosition,
    on_press: M,
    tokens: &'a MaterialTokens,
) -> Element<'a, M> {
    crate::components::buttons::button(tokens)
        .label(label)
        .icon(icon_name, icon_position.into())
        .variant(crate::components::buttons::ButtonVariant::Outlined)
        .on_press(on_press)
        .build()
        .unwrap_or_else(|e| {
            log::warn!("Failed to build secondary icon button: {}", e);
            iced::widget::Text::new(label).into()
        })
}

/// Creates a tertiary button with icon using the builder pattern
///
/// # Deprecated
/// Use `buttons::button(tokens).label("text").icon("icon", position).variant(ButtonVariant::Text).on_press(message).build()?` instead.
#[deprecated(
    since = "0.1.0",
    note = "Use `buttons::button(tokens).label(\"text\").icon(\"icon\", position).variant(ButtonVariant::Text).on_press(message).build()?` instead"
)]
pub fn tertiary_button_with_icon<'a, M: Clone + 'a>(
    label: &'a str,
    icon_name: &'a str,
    icon_position: CustomIconPosition,
    on_press: M,
    tokens: &'a MaterialTokens,
) -> Element<'a, M> {
    crate::components::buttons::button(tokens)
        .label(label)
        .icon(icon_name, icon_position.into())
        .variant(crate::components::buttons::ButtonVariant::Text)
        .on_press(on_press)
        .build()
        .unwrap_or_else(|e| {
            log::warn!("Failed to build tertiary icon button: {}", e);
            iced::widget::Text::new(label).into()
        })
}

/// Creates an icon-only button using the builder pattern
///
/// # Deprecated
/// Use `buttons::button(tokens).icon_only("icon", size).variant(variant).on_press(message).build()?` instead.
#[deprecated(
    since = "0.1.0",
    note = "Use `buttons::button(tokens).icon_only(\"icon\", size).variant(variant).on_press(message).build()?` instead"
)]
pub fn icon_button_semantic<'a, M: Clone + 'a>(
    icon_name: &'a str,
    variant: MaterialButtonVariant,
    size: CustomButtonSize,
    on_press: M,
    tokens: &'a MaterialTokens,
) -> Element<'a, M> {
    use crate::components::buttons::ButtonVariant as NewVariant;
    
    let variant = match variant {
        MaterialButtonVariant::Filled => NewVariant::Filled,
        MaterialButtonVariant::FilledTonal => NewVariant::FilledTonal,
        MaterialButtonVariant::Outlined => NewVariant::Outlined,
        MaterialButtonVariant::Text => NewVariant::Text,
        _ => {
            log::warn!("Unsupported button variant for icon button");
            NewVariant::Filled
        }
    };
    
    crate::components::buttons::button(tokens)
        .icon_only(icon_name, size.into())
        .variant(variant)
        .on_press(on_press)
        .build()
        .unwrap_or_else(|e| {
            log::warn!("Failed to build icon button: {}", e);
            iced::widget::Text::new("").into()
        })
}

/// Creates a filled icon button using the builder pattern
///
/// # Deprecated
/// Use `buttons::button(tokens).icon_only("icon", size).variant(ButtonVariant::Filled).on_press(message).build()?` instead.
#[deprecated(
    since = "0.1.0",
    note = "Use `buttons::button(tokens).icon_only(\"icon\", size).variant(ButtonVariant::Filled).on_press(message).build()?` instead"
)]
pub fn filled_icon_button_semantic<'a, M: Clone + 'a>(
    icon_name: &'a str,
    size: CustomButtonSize,
    on_press: M,
    tokens: &'a MaterialTokens,
) -> Element<'a, M> {
    crate::components::buttons::button(tokens)
        .icon_only(icon_name, size.into())
        .variant(crate::components::buttons::ButtonVariant::Filled)
        .on_press(on_press)
        .build()
        .unwrap_or_else(|e| {
            log::warn!("Failed to build filled icon button: {}", e);
            iced::widget::Text::new("").into()
        })
}

/// Creates a filled tonal icon button using the builder pattern
///
/// # Deprecated
/// Use `buttons::button(tokens).icon_only("icon", size).variant(ButtonVariant::FilledTonal).on_press(message).build()?` instead.
#[deprecated(
    since = "0.1.0",
    note = "Use `buttons::button(tokens).icon_only(\"icon\", size).variant(ButtonVariant::FilledTonal).on_press(message).build()?` instead"
)]
pub fn filled_tonal_icon_button_semantic<'a, M: Clone + 'a>(
    icon_name: &'a str,
    size: CustomButtonSize,
    on_press: M,
    tokens: &'a MaterialTokens,
) -> Element<'a, M> {
    crate::components::buttons::button(tokens)
        .icon_only(icon_name, size.into())
        .variant(crate::components::buttons::ButtonVariant::FilledTonal)
        .on_press(on_press)
        .build()
        .unwrap_or_else(|e| {
            log::warn!("Failed to build filled tonal icon button: {}", e);
            iced::widget::Text::new("").into()
        })
}

/// Creates an outlined icon button using the builder pattern
///
/// # Deprecated
/// Use `buttons::button(tokens).icon_only("icon", size).variant(ButtonVariant::Outlined).on_press(message).build()?` instead.
#[deprecated(
    since = "0.1.0",
    note = "Use `buttons::button(tokens).icon_only(\"icon\", size).variant(ButtonVariant::Outlined).on_press(message).build()?` instead"
)]
pub fn outlined_icon_button_semantic<'a, M: Clone + 'a>(
    icon_name: &'a str,
    size: CustomButtonSize,
    on_press: M,
    tokens: &'a MaterialTokens,
) -> Element<'a, M> {
    crate::components::buttons::button(tokens)
        .icon_only(icon_name, size.into())
        .variant(crate::components::buttons::ButtonVariant::Outlined)
        .on_press(on_press)
        .build()
        .unwrap_or_else(|e| {
            log::warn!("Failed to build outlined icon button: {}", e);
            iced::widget::Text::new("").into()
        })
}

/// Creates a Material Design 3 button with an icon using the proper Widget implementation.
///
/// This uses the MaterialButton widget with properly integrated icon content.
///
/// # Arguments
/// * `label` - The button text
/// * `icon_name` - The icon name
/// * `icon_position` - The position of the icon relative to the text
/// * `variant` - The Material Design 3 button variant
/// * `message` - The message to send when pressed
/// * `tokens` - Material Design tokens for styling
pub fn material_button_with_icon_widget<'a, Message: Clone + 'a>(
    label: &'a str,
    icon_name: &'a str,
    icon_position: MaterialIconPosition,
    variant: MaterialButtonVariant,
    message: Message,
    tokens: &'a MaterialTokens,
) -> Element<'a, Message> {
    // Create the button content with proper icon integration
    let content = icon_support::create_button_with_icon_content::<Message>(
        label,
        icon_name,
        icon_position,
        variant,
        tokens,
    );

    // Use the button widget with the content we created
    let mut button = MaterialButton::new_with_content(content, tokens)
        .variant(variant)
        .on_press(message);

    // Adjust button sizing based on icon position
    if icon_position == MaterialIconPosition::Only {
        // Icon-only buttons should be square
        let size = 40.0; // Default medium size
        button = button
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .padding(Padding::new(8.0)); // Less padding for icon-only
    } else {
        // Text with icon buttons need horizontal padding
        button = button.padding(Padding::new(16.0).top(10.0).bottom(10.0));
    }

    button.into()
}

/// Creates a Material Design 3 icon button using the proper Widget implementation.
///
/// This uses the MaterialButton widget with properly sized icon content.
///
/// # Arguments
/// * `icon_name` - The icon name
/// * `variant` - The Material Design 3 button variant
/// * `size` - The button size
/// * `message` - The message to send when pressed
/// * `tokens` - Material Design tokens for styling
pub fn material_icon_button_widget<'a, Message: Clone + 'a>(
    icon_name: &'a str,
    variant: MaterialButtonVariant,
    size: MaterialButtonSize,
    message: Message,
    tokens: &'a MaterialTokens,
) -> Element<'a, Message> {
    // Create icon content using our helper
    let content = icon_support::create_icon_button_content::<Message>(
        icon_name,
        variant,
        map_material_size_to_style_size(size),
        tokens,
    );

    // Use centralized size calculation
    let button_size = super::sizing::button_size_to_pixels(size);

    // Use the button widget with the icon content - icon buttons should be square
    MaterialButton::new_with_content(content, tokens)
        .variant(variant)
        .width(Length::Fixed(button_size))
        .height(Length::Fixed(button_size))
        .padding(8.0) // Less padding for icon-only buttons
        .on_press(message)
        .into()
}

/// Creates a Material Design 3 action button.
///
/// This is the main function for creating Material Design 3 buttons using the proper Widget implementation.
///
/// # Arguments
/// * `label` - The button text
/// * `variant` - The Material Design 3 button variant
/// * `message` - The message to send when pressed
/// * `tokens` - Material Design tokens for styling
///
/// # Examples
/// ```no_run
/// use abop_gui::components::common::create_button;
/// use abop_gui::styling::material::components::widgets::MaterialButtonVariant;
/// use abop_gui::styling::material::MaterialTokens;
///
/// #[derive(Clone)]
/// enum Message { Save, Cancel }
///
/// let tokens = MaterialTokens::default();
/// // Primary action button
/// let save_btn = create_button("Save", MaterialButtonVariant::Filled, Message::Save, &tokens);
///
/// // Secondary action button  
/// let cancel_btn = create_button("Cancel", MaterialButtonVariant::Outlined, Message::Cancel, &tokens);
/// ```
pub fn create_button<'a, Message: Clone + 'a>(
    label: &'a str,
    variant: MaterialButtonVariant,
    message: Message,
    tokens: &'a MaterialTokens,
) -> Element<'a, Message> {
    button_builder(tokens)
        .label(label)
        .variant(variant)
        .on_press(message)
        .build()
        .unwrap_or_else(|e| iced::widget::Text::new(format!("Button build error: {e}")).into())
}

// Helper conversion functions to bridge between different type systems
fn map_material_size_to_style_size(size: MaterialButtonSize) -> crate::styling::material::components::button_style::ButtonSizeVariant {
    use crate::styling::material::components::button_style::ButtonSizeVariant;
    match size {
        MaterialButtonSize::Small => ButtonSizeVariant::Small,
        MaterialButtonSize::Medium => ButtonSizeVariant::Medium,
        MaterialButtonSize::Large => ButtonSizeVariant::Large,
    }
}
