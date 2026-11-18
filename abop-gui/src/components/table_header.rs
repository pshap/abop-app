// Table header component with Material Design 3 styling and selection support

use iced::widget::{button, checkbox, container, row, text};
use iced::{Background, Color, Element, Length, Padding};
use std::collections::HashSet;

use crate::messages::Message;
use crate::state::TableState;
use crate::styling::material::MaterialTokens;
use crate::styling::material::components::data;
use crate::styling::material::components::selection_style::constants;

/// Height of the table header row
pub const HEADER_HEIGHT: f32 = 44.0;

/// Component for creating table headers with Material Design styling
pub struct TableHeader;

impl TableHeader {
    /// Create a header element from columns and state with selection support
    #[must_use]
    pub fn create<'a>(
        columns: &[data::TableColumn],
        state: &TableState,
        tokens: &'a MaterialTokens,
        selected_items: &HashSet<String>,
        total_items: usize,
    ) -> Element<'a, Message> {
        let mut header_cells = vec![]; // Add select-all checkbox with enhanced indeterminate state visualization
        let all_selected = !selected_items.is_empty() && selected_items.len() == total_items;
        let some_selected = !selected_items.is_empty() && selected_items.len() < total_items;

        // Enhanced checkbox with proper Material Design 3 styling and indeterminate state support
        let select_all_checkbox = checkbox("", all_selected)
            .on_toggle(|_| Message::ToggleSelectAll)
            .size(20)
            .style(move |_theme, _status| {
                // Enhanced Material Design styling with indeterminate state
                let background_color = if some_selected {
                    // Show a distinct background for indeterminate state
                    Color {
                        r: tokens.colors.primary.base.r,
                        g: tokens.colors.primary.base.g,
                        b: tokens.colors.primary.base.b,
                        a: constants::opacity::INDETERMINATE, // Material Design token for partial selection
                    }
                } else if all_selected {
                    tokens.colors.primary.base
                } else {
                    Color::TRANSPARENT
                };

                let border_color = if some_selected || all_selected {
                    tokens.colors.primary.base
                } else {
                    tokens.colors.outline
                };

                iced::widget::checkbox::Style {
                    background: Background::Color(background_color),
                    icon_color: if some_selected {
                        // Use a different icon color for indeterminate state
                        tokens.colors.primary.on_container
                    } else {
                        tokens.colors.primary.base
                    },
                    border: iced::Border {
                        color: border_color,
                        width: 2.0,
                        radius: 4.0.into(),
                    },
                    text_color: Some(tokens.colors.on_surface),
                }
            });

        let select_cell = container(select_all_checkbox)
            .width(Length::Fixed(48.0))
            .height(Length::Fill)
            .padding(Padding::from([8.0, 8.0]))
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center);

        header_cells.push(select_cell.into());

        // Add the regular column headers
        let column_headers = columns
            .iter()
            .enumerate()
            .map(|(index, column)| Self::create_header_cell(column, index, state, tokens));

        header_cells.extend(column_headers);

        container(
            row(header_cells)
                .height(Length::Fixed(HEADER_HEIGHT))
                .width(Length::Fill)
                .align_y(iced::Alignment::Center),
        )
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(tokens.colors.surface_container)),
            border: iced::Border {
                radius: iced::border::Radius::default()
                    .top(crate::components::ui::CARD_RADIUS - 1.0),
                ..iced::Border::default()
            },
            ..iced::widget::container::Style::default()
        })
        .width(Length::Fill)
        .height(Length::Fixed(HEADER_HEIGHT))
        .padding(Padding::ZERO)
        .into()
    }

    /// Create individual header cell with sorting functionality
    fn create_header_cell<'a>(
        column: &data::TableColumn,
        _index: usize,
        state: &TableState,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        let is_sorted = state.sort_column == column.id;
        let color = if is_sorted {
            tokens.colors.primary.base
        } else {
            tokens.colors.on_surface_variant
        };
        let align = match column.alignment {
            data::TextAlignment::End => iced::Alignment::End,
            data::TextAlignment::Center => iced::Alignment::Center,
            _ => iced::Alignment::Start,
        };

        let width = match column.width {
            data::ColumnWidth::Fixed(w) => Length::Fixed(w),
            data::ColumnWidth::FillPortion(factor) => Length::FillPortion(factor),
            data::ColumnWidth::Auto => Length::Shrink,
            data::ColumnWidth::Ratio(num, _den) => {
                Length::FillPortion(u16::try_from(num).unwrap_or(u16::MAX))
            }
            data::ColumnWidth::Shrink => Length::Shrink,
        };

        let mut label = row![
            text(column.title.clone())
                .size(tokens.typography().label_large.size)
                .color(color)
                .wrapping(iced::widget::text::Wrapping::None)
        ]
        .spacing(6)
        .align_y(iced::Alignment::Center);
        if is_sorted {
            let chevron = if state.sort_ascending {
                "chevron-up"
            } else {
                "chevron-down"
            };
            label = label.push(crate::components::ui::icon(chevron, 10.0, color));
        }
        let content = container(label)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(align)
            .align_y(iced::Alignment::Center);

        if column.sortable {
            button(content)
                .width(width)
                .height(Length::Fill)
                .padding(Padding::from([0.0, 12.0]))
                .style(move |theme, status| {
                    Self::header_cell_button_style(theme, status, tokens, is_sorted)
                })
                .on_press(Message::SortBy(column.id.clone()))
                .into()
        } else {
            container(content)
                .width(width)
                .height(Length::Fill)
                .padding(Padding::from([0.0, 12.0]))
                .into()
        }
    }

    /// Style for header cells: transparent, with a subtle hover tint
    fn header_cell_button_style(
        _theme: &iced::Theme,
        status: button::Status,
        tokens: &MaterialTokens,
        _is_sorted: bool,
    ) -> button::Style {
        let colors = &tokens.colors;
        let background = match status {
            button::Status::Hovered => Some(Background::Color(Color {
                a: 0.06,
                ..colors.on_surface
            })),
            button::Status::Pressed => Some(Background::Color(Color {
                a: 0.10,
                ..colors.on_surface
            })),
            _ => None,
        };

        button::Style {
            background,
            text_color: colors.on_surface_variant,
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        }
    }
}
