//! Material Design 3 Dialog Components
//!
//! This module provides Material Design 3 dialog components including alert,
//! confirmation, form, bottom sheet, and full screen dialogs. These dialogs are
//! designed to be safe, ergonomic, and easy to use.
//!
//! ## Overview
//!
//! The `MaterialDialog` system provides a builder-style API for creating various types
//! of dialogs with customizable properties. Each dialog is created using a constructor
//! function that sets appropriate defaults for that dialog type, and then customized
//! using chainable methods.

use iced::{
    Alignment, Background, Border, Color, Element, Length, Padding, Shadow, Theme,
    widget::{Column, Row, Space, Text, button, container, text},
};

use crate::styling::material::{MaterialTokens, elevation::ElevationLevel};

/// Material Design 3 Dialog types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogType {
    /// Basic alert dialog with acknowledgment
    Alert,
    /// Confirmation dialog with accept/cancel actions
    Confirmation,
    /// Form dialog for data input
    Form,
    /// Bottom sheet dialog (slides up from bottom)
    BottomSheet,
    /// Full screen dialog (covers entire screen)
    FullScreen,
}

/// Dialog button configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogButton {
    /// Text-only button (low emphasis)
    Text,
    /// Outlined button (medium emphasis)
    Outlined,
    /// Filled button (high emphasis)
    Filled,
    /// Tonal button (medium-high emphasis)
    Tonal,
}

/// Dialog size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogSize {
    /// Small dialog (280px max width)
    Small,
    /// Medium dialog (400px max width)
    Medium,
    /// Large dialog (560px max width)
    Large,
    /// Extra large dialog (720px max width)
    ExtraLarge,
}

/// Material Design 3 Dialog
///
/// Dialogs inform users about a task and can contain critical information,
/// require decisions, or involve multiple tasks. This implementation provides
/// a type-safe builder pattern for creating various dialog types.
///
/// Each dialog can be customized with titles, content, buttons, and behavior options
/// using the chainable methods provided by this struct. The final dialog is rendered
/// using the `view` method which takes callback messages for dialog interactions.
///
/// See the module-level documentation for usage examples.
#[derive(Debug, Clone)]
pub struct MaterialDialog {
    /// Dialog type
    #[allow(dead_code)]
    dialog_type: DialogType,
    /// Dialog size
    size: DialogSize,
    /// Dialog title
    title: Option<String>,
    /// Dialog content/body text
    content: Option<String>,
    /// Primary action button configuration
    primary_button: Option<(DialogButton, String)>,
    /// Secondary action button configuration
    secondary_button: Option<(DialogButton, String)>,
    /// Whether dialog can be dismissed by clicking outside
    dismissible: bool,
    /// Whether dialog has a close button
    show_close_button: bool,
    /// Custom elevation level
    elevation: Option<ElevationLevel>,
}

impl Default for MaterialDialog {
    fn default() -> Self {
        Self {
            dialog_type: DialogType::Alert,
            size: DialogSize::Medium,
            title: None,
            content: None,
            primary_button: None,
            secondary_button: None,
            dismissible: true,
            show_close_button: false,
            elevation: None,
        }
    }
}

impl MaterialDialog {
    /// Create a new alert dialog
    #[must_use]
    pub fn alert() -> Self {
        Self {
            dialog_type: DialogType::Alert,
            primary_button: Some((DialogButton::Filled, "OK".to_string())),
            ..Default::default()
        }
    }

    /// Create a new confirmation dialog
    #[must_use]
    pub fn confirmation() -> Self {
        Self {
            dialog_type: DialogType::Confirmation,
            primary_button: Some((DialogButton::Filled, "Confirm".to_string())),
            secondary_button: Some((DialogButton::Text, "Cancel".to_string())),
            ..Default::default()
        }
    }

    /// Create a new form dialog
    #[must_use]
    pub fn form() -> Self {
        Self {
            dialog_type: DialogType::Form,
            primary_button: Some((DialogButton::Filled, "Submit".to_string())),
            secondary_button: Some((DialogButton::Text, "Cancel".to_string())),
            show_close_button: true,
            ..Default::default()
        }
    }

    /// Create a new bottom sheet dialog
    #[must_use]
    pub fn bottom_sheet() -> Self {
        Self {
            dialog_type: DialogType::BottomSheet,
            size: DialogSize::Large,
            dismissible: true,
            ..Default::default()
        }
    }

    /// Create a new full screen dialog
    #[must_use]
    pub fn full_screen() -> Self {
        Self {
            dialog_type: DialogType::FullScreen,
            size: DialogSize::ExtraLarge,
            show_close_button: true,
            dismissible: false,
            ..Default::default()
        }
    }

    /// Set the dialog size
    #[must_use]
    pub const fn size(mut self, size: DialogSize) -> Self {
        self.size = size;
        self
    }

    /// Set the dialog title
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the dialog content
    #[must_use]
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Set the primary action button
    #[must_use]
    pub fn primary_button(mut self, button_type: DialogButton, label: impl Into<String>) -> Self {
        self.primary_button = Some((button_type, label.into()));
        self
    }

    /// Set the secondary action button
    #[must_use]
    pub fn secondary_button(mut self, button_type: DialogButton, label: impl Into<String>) -> Self {
        self.secondary_button = Some((button_type, label.into()));
        self
    }

    /// Make dialog dismissible by clicking outside
    #[must_use]
    pub const fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Show close button in dialog header
    #[must_use]
    pub const fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }

    /// Set custom elevation level
    #[must_use]
    pub const fn elevation(mut self, level: ElevationLevel) -> Self {
        self.elevation = Some(level);
        self
    }

    /// Create button element based on type
    fn create_button<Message: Clone + 'static>(
        button_type: DialogButton,
        label: String,
        tokens: &MaterialTokens,
        on_press: Option<Message>,
    ) -> Element<'static, Message> {
        let button_style = match button_type {
            DialogButton::Text => button::Style {
                background: None,
                text_color: tokens.colors.primary.base,
                border: Border::default(),
                shadow: Shadow::default(),
            },
            DialogButton::Outlined => button::Style {
                background: None,
                text_color: tokens.colors.primary.base,
                border: Border {
                    color: tokens.colors.outline,
                    width: 1.0,
                    radius: tokens.shapes.corner_full.radius,
                },
                shadow: Shadow::default(),
            },
            DialogButton::Filled => button::Style {
                background: Some(Background::Color(tokens.colors.primary.base)),
                text_color: tokens.colors.primary.on_base,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: tokens.shapes.corner_full.radius,
                },
                shadow: Shadow::default(),
            },
            DialogButton::Tonal => button::Style {
                background: Some(Background::Color(tokens.colors.secondary.container)),
                text_color: tokens.colors.secondary.on_container,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: tokens.shapes.corner_full.radius,
                },
                shadow: tokens.elevation.level1.shadow,
            },
        };

        // Create text without explicit color to allow button styling to control text color
        let label_text = Text::new(label)
            .size(tokens.typography.label_large.size)
            .font(iced::Font::DEFAULT); // Use system default font for button text

        let btn = button(label_text)
            .padding([8, 16])
            .style(move |_, _| button_style);

        if let Some(msg) = on_press {
            btn.on_press(msg).into()
        } else {
            btn.into()
        }
    }

    /// Create the dialog element
    pub fn view<Message: Clone + 'static>(
        self,
        tokens: &MaterialTokens,
        on_primary: Option<Message>,
        on_secondary: Option<Message>,
        on_close: Option<Message>,
    ) -> Element<'static, Message> {
        // Move all owned data out of self so nothing is borrowed
        let title = self.title;
        let content_text = self.content;
        let primary_button = self.primary_button;
        let secondary_button = self.secondary_button;
        let show_close_button = self.show_close_button;
        let size = self.size;
        let dialog_type = self.dialog_type;
        let elevation = self.elevation;
        // Note: dismissible behavior would typically be handled by a modal overlay container
        let _ = self.dismissible;

        let mut content = Column::new().spacing(16).width(Length::Fill);

        if title.is_some() || show_close_button {
            let mut header_row = Row::new().align_y(Alignment::Center);

            if let Some(title) = title {
                header_row = header_row.push(text(title).size(tokens.typography.title_large.size));
            }

            if show_close_button {
                header_row = header_row.push(Space::new(Length::Fill, 0));
                if let Some(close_msg) = on_close {
                    header_row = header_row.push(Self::create_button(
                        DialogButton::Text,
                        "Close".to_string(),
                        tokens,
                        Some(close_msg),
                    ));
                }
            }

            content = content.push(header_row);
        }

        if let Some(content_text) = content_text {
            content = content.push(
                text(content_text)
                    .size(tokens.typography.body_medium.size)
                    .width(Length::Fill),
            );
        }

        if primary_button.is_some() || secondary_button.is_some() {
            let mut button_row = Row::new()
                .spacing(8)
                .align_y(Alignment::Center)
                .width(Length::Fill);

            button_row = button_row.push(Space::new(Length::Fill, 0));

            if let Some((button_type, label)) = secondary_button {
                button_row = button_row.push(Self::create_button(
                    button_type,
                    label,
                    tokens,
                    on_secondary,
                ));
            }
            if let Some((button_type, label)) = primary_button {
                button_row =
                    button_row.push(Self::create_button(button_type, label, tokens, on_primary));
            }

            content = content.push(button_row);
        }

        let container_style = {
            let elevation = elevation.unwrap_or(match dialog_type {
                DialogType::Alert | DialogType::Confirmation | DialogType::Form => {
                    ElevationLevel::Level3
                }
                DialogType::BottomSheet => ElevationLevel::Level2,
                DialogType::FullScreen => ElevationLevel::Level1,
            });

            let shape = match dialog_type {
                DialogType::BottomSheet => tokens.shapes.corner_extra_large.radius,
                DialogType::FullScreen => tokens.shapes.corner_none.radius,
                _ => tokens.shapes.corner_extra_large.radius,
            };

            container::Style {
                background: Some(Background::Color(tokens.colors.surface_container_high)),
                border: Border {
                    color: tokens.colors.outline_variant,
                    width: if matches!(dialog_type, DialogType::FullScreen) {
                        0.0
                    } else {
                        1.0
                    },
                    radius: shape,
                },
                shadow: tokens.elevation.get_level(elevation).shadow,
                ..Default::default()
            }
        };

        container(content)
            .width(match size {
                DialogSize::Small => Length::Fixed(280.0),
                DialogSize::Medium => Length::Fixed(400.0),
                DialogSize::Large => Length::Fixed(560.0),
                DialogSize::ExtraLarge => Length::Fixed(720.0),
            })
            .padding(Padding::from([24, 24]))
            .style(move |_theme: &Theme| container_style)
            .into()
    }
}
