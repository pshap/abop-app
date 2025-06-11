//! Unified view methods for chip rendering
//!
//! This module provides view methods for rendering chips as Iced widgets with
//! Material Design 3 styling. It includes different view methods for various
//! use cases such as basic view, toggle view, and filter chip view.

use super::super::builder::Chip;
use super::super::common::{ChipState, ComponentSize, SelectionComponent};
use crate::styling::material::colors::MaterialColors;
use crate::styling::material::components::selection_style::{
    SelectionSize as LegacySelectionSize, SelectionStyleBuilder, SelectionVariant,
};
use crate::styling::material::tokens::MaterialTokens;

use iced::{
    Color, Element, Padding, Renderer,
    theme::Theme,
    widget::{Container, Row, Text, button},
};
use iced_font_awesome::fa_icon_solid;

// ============================================================================
// View Configuration Structures
// ============================================================================

/// Configuration for enhanced chip view with icons and badges
#[derive(Debug, Clone)]
pub struct ChipViewConfig<'a, Message> {
    /// Leading icon (shown before the label)
    pub leading_icon: Option<&'a str>,
    /// Trailing icon (shown after the label, often for deletion)
    pub trailing_icon: Option<&'a str>,
    /// Badge count to display
    pub badge_count: Option<u32>,
    /// Custom badge color (uses theme default if None)
    pub badge_color: Option<Color>,
    /// Main press action
    pub on_press: Option<Message>,
    /// Trailing icon press action (e.g., for delete functionality)
    pub on_trailing_press: Option<Message>,
}

impl<'a, Message> Default for ChipViewConfig<'a, Message> {
    fn default() -> Self {
        Self {
            leading_icon: None,
            trailing_icon: None,
            badge_count: None,
            badge_color: None,
            on_press: None,
            on_trailing_press: None,
        }
    }
}

// ============================================================================
// Chip View Methods Implementation
// ============================================================================

impl Chip {
    /// Create the Iced widget element for this chip
    ///
    /// # Arguments
    /// * `on_press` - Optional callback when the chip is pressed
    /// * `color_scheme` - Material Design color scheme to use for styling
    ///
    /// # Returns
    /// An Iced Element that can be added to the UI
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        on_press: Option<Message>,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        // Convert modern size to legacy size
        let legacy_size = match self.props().size {
            ComponentSize::Small => LegacySelectionSize::Small,
            ComponentSize::Medium => LegacySelectionSize::Medium,
            ComponentSize::Large => LegacySelectionSize::Large,
        };

        // Create styling function (avoid cloning color_scheme)
        let style_fn = SelectionStyleBuilder::new(
            MaterialTokens::default().with_colors(color_scheme.clone()),
            SelectionVariant::Chip,
        )
        .size(legacy_size)
        .chip_style(self.is_selected());

        // Create chip content
        let content = Text::new(self.label()).size(self.props().size.text_size());

        // Create chip button
        let mut chip_button = button(content).style(style_fn);

        // Only add on_press handler if the chip is not disabled and callback is provided
        if !self.props().disabled
            && let Some(message) = on_press
        {
            chip_button = chip_button.on_press(message);
        }

        chip_button.into()
    }

    /// Create a view that handles selection state changes automatically
    ///
    /// This is a convenience method for chips that should toggle their
    /// selection state when pressed.
    pub fn view_with_toggle<'a, Message: Clone + 'a>(
        &'a self,
        on_toggle: impl Fn(ChipState) -> Message + 'a,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        let next_state = self.state().toggle();
        // Use a single message creation and mapping for efficiency
        self.view(None, color_scheme)
            .map(move |_: Message| on_toggle(next_state))
    }

    /// Create a view for filter chips with selection state management
    ///
    /// This is specifically designed for filter chips that need to
    /// maintain selected/unselected state.
    pub fn view_as_filter<'a, Message: Clone + 'a>(
        &'a self,
        on_selection_change: impl Fn(bool) -> Message + 'a,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        let is_selected = self.is_selected();
        let new_selection = !is_selected;
        // Use a single message creation and mapping for efficiency
        self.view(None, color_scheme)
            .map(move |_: Message| on_selection_change(new_selection))
    }

    /// View with leading and/or trailing icons
    ///
    /// This method allows for adding icons to enhance the chip's visual communication.
    /// Common use cases include:
    /// - Leading icons: category indicators, status icons
    /// - Trailing icons: delete buttons, dropdown indicators
    ///
    /// # Arguments
    /// * `leading_icon` - Font Awesome icon name for leading position
    /// * `trailing_icon` - Font Awesome icon name for trailing position  
    /// * `on_press` - Main chip press action
    /// * `on_trailing_press` - Separate action for trailing icon (e.g., delete)
    /// * `color_scheme` - Material Design color scheme for styling
    pub fn view_with_icons<'a, Message: Clone + 'a>(
        &'a self,
        leading_icon: Option<&'a str>,
        trailing_icon: Option<&'a str>,
        on_press: Option<Message>,
        on_trailing_press: Option<Message>,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        self.view_enhanced(
            ChipViewConfig {
                leading_icon,
                trailing_icon,
                badge_count: None,
                badge_color: None,
                on_press,
                on_trailing_press,
            },
            color_scheme,
        )
    }

    /// View with badge/count indicator
    ///
    /// Displays a badge with a numeric count, useful for showing quantities
    /// or notification counts.
    ///
    /// # Arguments
    /// * `badge_count` - Number to display in the badge (None for no badge)
    /// * `badge_color` - Optional custom badge color
    /// * `on_press` - Chip press action
    /// * `color_scheme` - Material Design color scheme for styling
    pub fn view_with_badge<'a, Message: Clone + 'a>(
        &'a self,
        badge_count: Option<u32>,
        badge_color: Option<Color>,
        on_press: Option<Message>,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        self.view_enhanced(
            ChipViewConfig {
                leading_icon: None,
                trailing_icon: None,
                badge_count,
                badge_color,
                on_press,
                on_trailing_press: None,
            },
            color_scheme,
        )
    }

    /// Combined view with icons AND badge
    ///
    /// This is the most comprehensive chip view method, supporting all
    /// visual enhancements: leading icons, trailing icons, and badges.
    ///
    /// # Arguments
    /// * `config` - Complete configuration for the enhanced chip view
    /// * `color_scheme` - Material Design color scheme for styling
    pub fn view_enhanced<'a, Message: Clone + 'a>(
        &'a self,
        config: ChipViewConfig<'a, Message>,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        // Convert modern size to legacy size for consistent styling
        let legacy_size = match self.props().size {
            ComponentSize::Small => LegacySelectionSize::Small,
            ComponentSize::Medium => LegacySelectionSize::Medium,
            ComponentSize::Large => LegacySelectionSize::Large,
        };

        // Create styling function
        let style_fn = SelectionStyleBuilder::new(
            MaterialTokens::default().with_colors(color_scheme.clone()),
            SelectionVariant::Chip,
        )
        .size(legacy_size)
        .chip_style(self.is_selected());

        // Build content with icons and text
        let mut content_row = Row::new().spacing(4.0);

        // Add leading icon if specified
        if let Some(icon_name) = config.leading_icon {
            let icon_size = match self.props().size {
                ComponentSize::Small => 12.0,
                ComponentSize::Medium => 14.0,
                ComponentSize::Large => 16.0,
            };

            let icon = fa_icon_solid(icon_name)
                .size(icon_size)
                .style(move |_theme: &Theme| iced::widget::text::Style {
                    color: Some(color_scheme.on_surface),
                });

            content_row = content_row.push(icon);
        }

        // Add main text
        let text_element = Text::new(self.label())
            .size(self.props().size.text_size())
            .style(move |_theme: &Theme| iced::widget::text::Style {
                color: Some(color_scheme.on_surface),
            });

        content_row = content_row.push(text_element);

        // Add badge if specified
        if let Some(count) = config.badge_count {
            let badge_color = config.badge_color.unwrap_or(color_scheme.error.base);
            let badge_text_color = color_scheme.on_error;

            let badge = Container::new(Text::new(count.to_string()).size(10.0).style(
                move |_theme: &Theme| iced::widget::text::Style {
                    color: Some(badge_text_color),
                },
            ))
            .padding(Padding::from([2, 6]))
            .style(move |_theme: &Theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(badge_color)),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            });

            content_row = content_row.push(badge);
        }

        // Add trailing icon if specified (typically for delete functionality)
        if let Some(icon_name) = config.trailing_icon {
            let icon_size = match self.props().size {
                ComponentSize::Small => 12.0,
                ComponentSize::Medium => 14.0,
                ComponentSize::Large => 16.0,
            };

            // If there's a trailing press action, make it a separate button
            if let Some(trailing_message) = config.on_trailing_press {
                let icon_button = button(fa_icon_solid(icon_name).size(icon_size).style(
                    move |_theme: &Theme| iced::widget::text::Style {
                        color: Some(color_scheme.on_surface_variant),
                    },
                ))
                .padding(Padding::from([2, 2]))
                .style(move |_theme: &Theme, _status| iced::widget::button::Style {
                    background: Some(iced::Background::Color(Color::TRANSPARENT)),
                    text_color: color_scheme.on_surface_variant,
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 4.0.into(),
                    },
                    shadow: iced::Shadow::default(),
                })
                .on_press(trailing_message);

                content_row = content_row.push(icon_button);
            } else {
                // Just a static icon
                let icon = fa_icon_solid(icon_name)
                    .size(icon_size)
                    .style(move |_theme: &Theme| iced::widget::text::Style {
                        color: Some(color_scheme.on_surface_variant),
                    });

                content_row = content_row.push(icon);
            }
        }

        // Create the main chip button
        let mut chip_button = button(content_row).style(style_fn);

        // Add main press handler if specified and chip is not disabled
        if !self.props().disabled
            && let Some(message) = config.on_press
        {
            chip_button = chip_button.on_press(message);
        }

        chip_button.into()
    }
}
