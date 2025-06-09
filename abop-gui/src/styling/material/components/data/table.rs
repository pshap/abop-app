//! Material Design 3 Data Table component implementation

use crate::styling::material::MaterialTokens;
use iced::Element;
use iced::widget::container;

use super::types::TextAlignment;

// Standard border for table cells and headers
fn table_border(color: iced::Color) -> iced::Border {
    iced::Border {
        color,
        width: 0.5,
        radius: 0.0.into(),
    }
}

// Transparent border for containers
fn transparent_border() -> iced::Border {
    iced::Border {
        color: iced::Color::TRANSPARENT,
        width: 0.0,
        radius: 0.0.into(),
    }
}

/// Color helper functions for table styling
mod color_helpers {
    use crate::styling::material::MaterialTokens;
    use iced::Color;

    /// Get background color for table rows
    pub fn get_row_background_color(
        tokens: &MaterialTokens,
        index: usize,
        is_selected: bool,
        is_striped: bool,
    ) -> Color {
        if is_selected {
            tokens.colors.secondary_container
        } else if is_striped && index % 2 == 1 {
            tokens.colors.surface_container
        } else {
            tokens.colors.surface_container_lowest
        }
    }

    /// Get background color for header cells
    pub fn get_header_background_color(tokens: &MaterialTokens, is_sorted: bool) -> Color {
        if is_sorted {
            tokens.colors.primary_container
        } else {
            tokens.colors.surface_variant
        }
    }
}

/// Typography helper functions for table text
mod typography_helpers {
    use crate::styling::material::typography::TypeStyle;
    use iced::Element;
    use iced::widget::text;
    use iced::widget::text::LineHeight;
    use iced::{Color, Pixels};

    /// Create text element with Material Design typography style
    pub fn create_text_element<'a>(
        content: &'a str,
        type_style: &TypeStyle,
        color: Color,
    ) -> Element<'a, crate::messages::Message> {
        text(content)
            .size(type_style.size())
            .line_height(LineHeight::Absolute(Pixels(type_style.line_height.0)))
            .color(color)
            .into()
    }
}

/// Material Design 3 Data Table implementation
pub struct MaterialDataTable;

impl MaterialDataTable {
    /// Create a Material Design 3 compliant data table container
    pub fn table_container(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens = tokens.clone();
        move |_theme| container::Style {
            background: Some(iced::Background::Color(tokens.colors.surface)),
            border: table_border(tokens.colors.outline_variant),
            ..Default::default()
        }
    }

    /// Style for table header
    pub fn header_container(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens = tokens.clone();
        move |_theme| container::Style {
            background: Some(iced::Background::Color(tokens.colors.surface_variant)),
            border: transparent_border(),
            ..Default::default()
        }
    }

    /// Style for table row
    pub fn row_container(
        tokens: &MaterialTokens,
        is_selected: bool,
        is_striped: bool,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens = tokens.clone();
        move |_theme| {
            let background = if is_selected {
                tokens.colors.surface_container_high
            } else if is_striped {
                tokens.colors.surface_container_low
            } else {
                tokens.colors.surface
            };

            container::Style {
                background: Some(iced::Background::Color(background)),
                border: transparent_border(),
                ..Default::default()
            }
        }
    }

    /// Style for table cell
    pub fn table_cell(
        tokens: &MaterialTokens,
        _align: TextAlignment,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let _tokens = tokens.clone();
        move |_theme| container::Style {
            background: None,
            border: transparent_border(),
            ..Default::default()
        }
    }

    /// Create header text
    pub fn header_text<'a>(
        content: &'a str,
        tokens: &MaterialTokens,
    ) -> Element<'a, crate::messages::Message> {
        typography_helpers::create_text_element(
            content,
            &tokens.typography.body_medium,
            tokens.colors.on_surface,
        )
    }

    /// Create body text
    pub fn body_text<'a>(
        content: &'a str,
        tokens: &MaterialTokens,
    ) -> Element<'a, crate::messages::Message> {
        typography_helpers::create_text_element(
            content,
            &tokens.typography.body_small,
            tokens.colors.on_surface,
        )
    }

    /// Style for header cell (backward compatibility)
    pub fn header_cell(
        tokens: &MaterialTokens,
        _sortable: bool,
        is_sorted: bool,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens = tokens.clone();
        move |_theme| {
            let background = color_helpers::get_header_background_color(&tokens, is_sorted);
            container::Style {
                background: Some(iced::Background::Color(background)),
                border: transparent_border(),
                ..Default::default()
            }
        }
    }

    /// Style for table row (backward compatibility)
    pub fn table_row(
        tokens: &MaterialTokens,
        index: usize,
        is_selected: bool,
        is_striped: bool,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens = tokens.clone();
        move |_theme| {
            let background =
                color_helpers::get_row_background_color(&tokens, index, is_selected, is_striped);
            container::Style {
                background: Some(iced::Background::Color(background)),
                border: transparent_border(),
                ..Default::default()
            }
        }
    }
}

// Backward compatibility re-exports
pub use MaterialDataTable as DataTable;
