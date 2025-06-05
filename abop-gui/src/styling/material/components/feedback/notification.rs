//! Material Design 3 Notification Components
//!
//! This module provides Material Design 3 notification components including
//! toast, banner, inline alert, and snackbar notifications.

use iced::{
    Alignment, Background, Border, Color, Element, Length, Padding,
    widget::{Column, Row, Space, Text, button, container},
};

use crate::styling::container::FeedbackContainerStyles;
use crate::styling::material::{MaterialTokens, elevation::ElevationLevel};
use crate::theme::ThemeMode;

use super::dialog::DialogButton;
use super::style_utils::{StatusType, create_action_button, get_status_colors};

/// Material Design 3 Notification types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    /// Toast notification (temporary, auto-dismissing)
    Toast,
    /// Banner notification (persistent, manual dismiss)
    Banner,
    /// Inline alert (embedded in content)
    InlineAlert,
    /// Snackbar notification (bottom of screen, with actions)
    Snackbar,
}

/// Notification severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationSeverity {
    /// Success notification (green)
    Success,
    /// Warning notification (amber/orange)
    Warning,
    /// Error notification (red)
    Error,
    /// Info notification (blue)
    Info,
    /// Neutral notification (no semantic color)
    Neutral,
}

/// Notification position on screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationPosition {
    /// Top left corner
    TopLeft,
    /// Top center
    TopCenter,
    /// Top right corner
    TopRight,
    /// Bottom left corner
    BottomLeft,
    /// Bottom center
    BottomCenter,
    /// Bottom right corner
    BottomRight,
}

/// Notification action configuration
#[derive(Debug, Clone)]
pub struct NotificationAction {
    /// Action label
    pub label: String,
    /// Action button style
    pub button_type: DialogButton,
    /// Whether action dismisses notification
    pub dismisses: bool,
}

impl NotificationAction {
    /// Create a new notification action
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            button_type: DialogButton::Text,
            dismisses: true,
        }
    }

    /// Set button type
    #[must_use]
    pub const fn button_type(mut self, button_type: DialogButton) -> Self {
        self.button_type = button_type;
        self
    }

    /// Set whether action dismisses notification
    #[must_use]
    pub const fn dismisses(mut self, dismisses: bool) -> Self {
        self.dismisses = dismisses;
        self
    }
}

/// Material Design 3 Notification
///
/// Notifications provide brief, important messages about app processes.
/// They can be temporary (toast) or persistent (banner/alert).
#[derive(Debug, Clone)]
pub struct MaterialNotification {
    /// Notification type
    notification_type: NotificationType,
    /// Severity level
    severity: NotificationSeverity,
    /// Position on screen
    position: NotificationPosition,
    /// Notification title
    title: Option<String>,
    /// Notification message
    message: String,
    /// Notification icon
    icon: Option<String>,
    /// Notification actions
    actions: Vec<NotificationAction>,
    /// Whether notification can be dismissed
    dismissible: bool,
    /// Whether notification has a close button
    show_close_button: bool,
    /// Maximum width
    max_width: Option<f32>,
}

impl MaterialNotification {
    /// Create a new toast notification
    #[must_use]
    pub fn toast(message: impl Into<String>) -> Self {
        Self {
            notification_type: NotificationType::Toast,
            severity: NotificationSeverity::Neutral,
            position: NotificationPosition::BottomCenter,
            title: None,
            message: message.into(),
            icon: None,
            actions: Vec::new(),
            dismissible: true,
            show_close_button: false,
            max_width: Some(400.0),
        }
    }

    /// Create a new banner notification
    #[must_use]
    pub fn banner(message: impl Into<String>) -> Self {
        Self {
            notification_type: NotificationType::Banner,
            severity: NotificationSeverity::Info,
            position: NotificationPosition::TopCenter,
            title: None,
            message: message.into(),
            icon: None,
            actions: Vec::new(),
            dismissible: true,
            show_close_button: true,
            max_width: None,
        }
    }

    /// Create a new inline alert
    #[must_use]
    pub fn inline_alert(message: impl Into<String>) -> Self {
        Self {
            notification_type: NotificationType::InlineAlert,
            severity: NotificationSeverity::Info,
            position: NotificationPosition::TopCenter,
            title: None,
            message: message.into(),
            icon: Some("ℹ".to_string()),
            actions: Vec::new(),
            dismissible: false,
            show_close_button: false,
            max_width: None,
        }
    }

    /// Create a new snackbar notification
    #[must_use]
    pub fn snackbar(message: impl Into<String>) -> Self {
        Self {
            notification_type: NotificationType::Snackbar,
            severity: NotificationSeverity::Neutral,
            position: NotificationPosition::BottomLeft,
            title: None,
            message: message.into(),
            icon: None,
            actions: Vec::new(),
            dismissible: true,
            show_close_button: false,
            max_width: Some(400.0),
        }
    }

    /// Set notification severity
    #[must_use]
    pub const fn severity(mut self, severity: NotificationSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Set notification position
    #[must_use]
    pub const fn position(mut self, position: NotificationPosition) -> Self {
        self.position = position;
        self
    }

    /// Set notification title
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set notification icon
    #[must_use]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Add an action to the notification
    #[must_use]
    pub fn add_action(mut self, action: NotificationAction) -> Self {
        self.actions.push(action);
        self
    }

    /// Set whether notification can be dismissed
    #[must_use]
    pub const fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Set whether notification has a close button
    #[must_use]
    pub const fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }

    /// Set maximum width
    #[must_use]
    pub const fn max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Convert severity to `StatusType`
    const fn severity_to_status_type(&self) -> StatusType {
        match self.severity {
            NotificationSeverity::Success => StatusType::Success,
            NotificationSeverity::Warning => StatusType::Warning,
            NotificationSeverity::Error => StatusType::Error,
            NotificationSeverity::Info => StatusType::Info,
            NotificationSeverity::Neutral => StatusType::Neutral,
        }
    }

    /// Get colors for notification based on severity
    fn get_colors(&self, tokens: &MaterialTokens) -> (Color, Color, Color) {
        let status_type = self.severity_to_status_type();
        let (accent_color, on_accent_color) = get_status_colors(tokens, &status_type);

        let background_color = match self.notification_type {
            NotificationType::InlineAlert => {
                // For inline alerts, use a lighter background
                Color {
                    r: accent_color.r.mul_add(0.15, tokens.colors.surface.r * 0.85),
                    g: accent_color.g.mul_add(0.15, tokens.colors.surface.g * 0.85),
                    b: accent_color.b.mul_add(0.15, tokens.colors.surface.b * 0.85),
                    a: 1.0,
                }
            }
            _ => {
                if self.severity == NotificationSeverity::Neutral {
                    tokens.colors.surface_container
                } else {
                    accent_color
                }
            }
        };

        let text_color = if self.severity == NotificationSeverity::Neutral {
            tokens.colors.on_surface
        } else {
            match self.notification_type {
                NotificationType::InlineAlert => tokens.colors.on_surface,
                _ => on_accent_color,
            }
        };

        (background_color, text_color, accent_color)
    }

    /// Get container styling
    fn get_container_style(&self, tokens: &MaterialTokens) -> container::Style {
        let (background_color, _, accent_color) = self.get_colors(tokens);

        let elevation = match self.notification_type {
            NotificationType::Toast | NotificationType::Snackbar => ElevationLevel::Level2,
            NotificationType::Banner => ElevationLevel::Level1,
            NotificationType::InlineAlert => ElevationLevel::Level0,
        };

        let shape = match self.notification_type {
            NotificationType::Toast | NotificationType::Snackbar => {
                tokens.shapes.corner_small.radius
            }
            NotificationType::Banner => tokens.shapes.corner_none.radius,
            NotificationType::InlineAlert => tokens.shapes.corner_small.radius,
        };

        container::Style {
            background: Some(Background::Color(background_color)),
            border: Border {
                color: if matches!(self.notification_type, NotificationType::InlineAlert) {
                    accent_color
                } else {
                    Color::TRANSPARENT
                },
                width: if matches!(self.notification_type, NotificationType::InlineAlert) {
                    1.0
                } else {
                    0.0
                },
                radius: shape,
            },
            shadow: tokens.elevation.get_level(elevation).shadow,
            ..Default::default()
        }
    }

    /// Create the notification element
    pub fn view<'a, Message>(
        &'a self,
        tokens: &'a MaterialTokens,
        on_dismiss: Option<Message>,
        on_action: impl Fn(usize) -> Message + 'a,
    ) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let (_, text_color, _) = self.get_colors(tokens);

        // Create content
        let mut content = Column::new().spacing(8).width(Length::Fill);

        // Add title if present
        if let Some(title) = &self.title {
            let mut title_row = Row::new().spacing(8).align_y(Alignment::Center);

            // Add icon if present
            if let Some(icon) = &self.icon {
                title_row = title_row.push(
                    Text::new(icon).size(tokens.typography.title_medium.size), // .color(text_color) // Only set if icon contrast is needed
                );
            }

            title_row = title_row.push(
                Text::new(title).size(tokens.typography.title_medium.size), // .color(text_color) // Let container/button style control text color
            );

            // Add close button if needed
            if self.show_close_button {
                title_row = title_row.push(Space::new(Length::Fill, 0));
                if let Some(dismiss_msg) = on_dismiss.clone() {
                    title_row = title_row.push(
                        button(Text::new("×").size(16.0))
                            .padding(4)
                            .style(move |_, _| button::Style {
                                background: None,
                                text_color,
                                ..Default::default()
                            })
                            .on_press(dismiss_msg),
                    );
                }
            }

            content = content.push(title_row);
        }

        // Add message
        content = content.push(
            Text::new(&self.message)
                .size(tokens.typography.body_medium.size)
                // .color(text_color) // Let container/button style control text color
                .width(Length::Fill),
        );

        // Add actions if present
        if !self.actions.is_empty() {
            let mut action_row = Row::new().spacing(8).align_y(Alignment::Center);

            // Add spacer to push actions to the right
            action_row = action_row.push(Space::new(Length::Fill, 0));

            // Add each action button
            for (i, action) in self.actions.iter().enumerate() {
                let action_msg = on_action(i);
                let is_primary = i == self.actions.len() - 1;

                action_row = action_row.push(create_action_button(
                    tokens,
                    &action.label,
                    action_msg,
                    is_primary,
                ));
            }

            content = content.push(action_row);
        }

        // Determine if we should use the theme's dark or light mode
        let theme_mode = if tokens.is_dark_theme() {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };

        // Create the container with appropriate style based on severity
        let mut notification = container(content).padding(Padding::from([12, 16]));

        if self.severity == NotificationSeverity::Success {
            notification = notification.style(FeedbackContainerStyles::success(theme_mode));
        } else if self.severity == NotificationSeverity::Warning {
            notification = notification.style(FeedbackContainerStyles::warning(theme_mode));
        } else if self.severity == NotificationSeverity::Error {
            notification = notification.style(FeedbackContainerStyles::error(theme_mode));
        } else if self.severity == NotificationSeverity::Info {
            notification = notification.style(FeedbackContainerStyles::info(theme_mode));
        } else {
            let style = self.get_container_style(tokens);
            notification = notification.style(move |_: &iced::Theme| style);
        }

        // Apply max width if specified
        if let Some(width) = self.max_width {
            notification = notification.width(Length::Fixed(width));
        } else {
            notification = notification.width(Length::Fill);
        }

        // Make dismissible if needed - wrap in button since Container doesn't have on_press
        if self.dismissible
            && !self.show_close_button
            && let Some(dismiss_msg) = on_dismiss
        {
            return button(notification)
                .style(|_, _| button::Style::default())
                .on_press(dismiss_msg)
                .into();
        }

        notification.into()
    }
}
