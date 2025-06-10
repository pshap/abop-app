//! Feedback container styles for status and messaging
//!
//! Provides container styling for feedback elements including success,
//! warning, error, and info messages. These styles use semantic colors
//! to provide clear visual communication about system state and user actions.

use crate::styling::material::MaterialTokens;
use crate::styling::material::helpers::ElevationHelpers;
use crate::theme::ThemeMode;
use iced::widget::container;
use iced::{Background, Border, Color};

/// Feedback-specific container style types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedbackContainerType {
    /// Success color container for positive feedback
    /// Used for confirmation messages and success states
    Success,
    /// Warning color container for cautionary information
    /// Used for warning messages and attention states
    Warning,
    /// Error color container for negative feedback
    /// Used for error messages and failure states
    Error,
    /// Info color container for informational content
    /// Used for help text and informational messages
    Info,
    /// General status container for state indication
    /// Used for status bars and state displays
    Status,
    /// Playing status container for active audio indication
    /// Used to show currently playing audio
    StatusPlaying,
    /// Info status container for informational status
    /// Used for non-critical status information
    StatusInfo,
}

/// Feedback container styling utilities
pub struct FeedbackContainerStyles;

impl FeedbackContainerStyles {
    /// Get feedback container style based on type and theme
    #[must_use]
    pub fn get_style(style_type: FeedbackContainerType, theme_mode: ThemeMode) -> container::Style {
        let material_tokens = MaterialTokens::default();
        let shapes = material_tokens.shapes();
        match style_type {
            FeedbackContainerType::Success => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(theme_mode.success_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: shapes.card().to_radius(),
                },
                shadow: material_tokens.elevation_shadow(1),
            },
            FeedbackContainerType::Warning => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(theme_mode.warning_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: shapes.card().to_radius(),
                },
                shadow: material_tokens.elevation_shadow(1),
            },
            FeedbackContainerType::Error => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(theme_mode.error_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: shapes.card().to_radius(),
                },
                shadow: material_tokens.elevation_shadow(1),
            },
            FeedbackContainerType::Info => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(theme_mode.info_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: shapes.card().to_radius(),
                },
                shadow: material_tokens.elevation_shadow(1),
            },
            FeedbackContainerType::Status => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(theme_mode.surface_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: shapes.card().to_radius(),
                },
                shadow: material_tokens.elevation_shadow(1),
            },
            FeedbackContainerType::StatusPlaying => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(theme_mode.primary_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: shapes.card().to_radius(),
                },
                shadow: material_tokens.elevation_shadow(1),
            },
            FeedbackContainerType::StatusInfo => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(theme_mode.info_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: shapes.card().to_radius(),
                },
                shadow: material_tokens.elevation_shadow(1),
            },
        }
    }

    /// Creates a success color container style for positive feedback
    ///
    /// Success containers use the theme's success color for confirmations and positive states.
    ///
    /// # Returns
    /// A style function that creates success container appearance
    pub fn success(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(FeedbackContainerType::Success, theme_mode);
        move |_| style
    }

    /// Creates a warning color container style for cautionary information
    ///
    /// Warning containers use the theme's warning color for alerts and caution messages.
    ///
    /// # Returns
    /// A style function that creates warning container appearance
    pub fn warning(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(FeedbackContainerType::Warning, theme_mode);
        move |_| style
    }

    /// Creates an error color container style for negative feedback
    ///
    /// Error containers use the theme's error color for failures and error messages.
    ///
    /// # Returns
    /// A style function that creates error container appearance
    pub fn error(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(FeedbackContainerType::Error, theme_mode);
        move |_| style
    }

    /// Creates an info color container style for informational content
    ///
    /// Info containers use the theme's info color for help text and informational messages.
    ///
    /// # Returns
    /// A style function that creates info container appearance
    pub fn info(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(FeedbackContainerType::Info, theme_mode);
        move |_| style
    }

    /// Creates a status container style for state indication
    ///
    /// Status containers are used for status bars and state displays.
    ///
    /// # Returns
    /// A style function that creates status container appearance
    pub fn status(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(FeedbackContainerType::Status, theme_mode);
        move |_| style
    }

    /// Creates a playing status container style for active audio indication
    ///
    /// Used to show currently playing audio or active states.
    ///
    /// # Returns
    /// A style function that creates playing status container appearance
    pub fn status_playing(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(FeedbackContainerType::StatusPlaying, theme_mode);
        move |_| style
    }

    /// Creates an info status container style for informational status
    ///
    /// Used for non-critical status information.
    ///
    /// # Returns
    /// A style function that creates info status container appearance
    pub fn status_info(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(FeedbackContainerType::StatusInfo, theme_mode);
        move |_| style
    }
}
