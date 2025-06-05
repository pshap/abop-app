//! Layout container styles for structural UI elements
//!
//! Provides container styling for core layout elements including headers,
//! content areas, and sidebars. These containers form the structural
//! foundation of the application's visual hierarchy.

use crate::styling::material::MaterialTokens;
use crate::theme::ThemeMode;
use iced::border::Radius;
use iced::widget::container;
use iced::{Background, Border, Color, Shadow};

/// Layout-specific container style types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutContainerType {
    /// Header container with primary color background
    /// Used for top-level navigation and title areas
    Header,
    /// Content container with background color matching theme
    /// Used for main content areas and body sections
    Content,
    /// Sidebar container with variant surface color
    /// Used for navigation panels and secondary content
    Sidebar,
}

/// Layout container styling utilities
pub struct LayoutContainerStyles;

impl LayoutContainerStyles {
    /// Get layout container style based on type and theme
    #[must_use]
    pub fn get_style(style_type: LayoutContainerType, theme_mode: ThemeMode) -> container::Style {
        let material_tokens = MaterialTokens::default();
        let shapes = material_tokens.shapes();
        let ui = material_tokens.ui();
        match style_type {
            LayoutContainerType::Header => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(theme_mode.primary_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(0.0),
                },
                shadow: material_tokens.elevation_shadow(3),
            },
            LayoutContainerType::Content => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(theme_mode.background_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(0.0),
                },
                shadow: Shadow::default(),
            },
            LayoutContainerType::Sidebar => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(theme_mode.surface_variant_color())),
                border: Border {
                    color: theme_mode.border_color(),
                    width: ui.border_width_standard,
                    radius: shapes.card().to_radius(),
                },
                shadow: material_tokens.elevation_shadow(2),
            },
        }
    }

    /// Creates a header container style with primary color background
    ///
    /// Header containers use primary colors and strong shadows to
    /// establish visual hierarchy for top-level navigation and titles.
    ///
    /// # Returns
    /// A style function that creates header container appearance
    pub fn header(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(LayoutContainerType::Header, theme_mode);
        move |_| style
    }

    /// Creates a content container style matching the background theme
    ///
    /// Content containers provide neutral backgrounds for main content
    /// areas without competing with the contained information.
    ///
    /// # Returns
    /// A style function that creates content container appearance
    pub fn content(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(LayoutContainerType::Content, theme_mode);
        move |_| style
    }

    /// Creates a sidebar container style with variant surface color
    ///
    /// Sidebar containers use subtly different backgrounds to
    /// distinguish navigation and secondary content areas.
    ///
    /// # Returns
    /// A style function that creates sidebar container appearance
    pub fn sidebar(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(LayoutContainerType::Sidebar, theme_mode);
        move |_| style
    }
}
