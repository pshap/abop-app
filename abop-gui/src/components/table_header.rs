// Table header component with Material Design 3 styling and selection support

use iced::widget::{button, checkbox, container, row, text};
use iced::{Background, Color, Element, Length, Padding};
use std::collections::HashSet;

use crate::messages::Message;
use crate::state::TableState;
use crate::styling::material::MaterialTokens;
use crate::styling::material::components::data;

/// Component for creating table headers with Material Design styling
pub struct TableHeader;

impl TableHeader {    /// Create a header element from columns and state with selection support
    #[must_use]
    pub fn create<'a>(
        columns: &[data::TableColumn],
        state: &TableState,
        tokens: &'a MaterialTokens,
        selected_items: &HashSet<String>,
        total_items: usize,
    ) -> Element<'a, Message> {
        let mut header_cells = vec![];        // Add select-all checkbox with enhanced indeterminate state visualization
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
                        a: 0.5, // Semi-transparent to indicate partial selection
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
                .height(Length::Fixed(48.0)) // Standard header height
                .width(Length::Fill)
                .align_y(iced::Alignment::Center),
        )
        .style(data::MaterialDataTable::header_container(tokens))
        .width(Length::Fill)
        .height(Length::Fixed(48.0)) // Ensure container matches content height
        .padding(Padding::ZERO) // Remove padding to prevent extra space
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

        let sort_icon = if is_sorted {
            if state.sort_ascending { " ↑" } else { " ↓" }
        } else {
            ""
        };

        let header_text = format!("{}{}", column.title, sort_icon);

        let width = match column.width {
            data::ColumnWidth::Fixed(w) => Length::Fixed(w),
            data::ColumnWidth::FillPortion(factor) => Length::FillPortion(factor),
            data::ColumnWidth::Auto => Length::Shrink,
            data::ColumnWidth::Ratio(num, _den) => Length::FillPortion(num as u16),
            data::ColumnWidth::Shrink => Length::Shrink,
        };

        // Create the entire header cell as a clickable button for better UX
        if column.sortable {
            button(
                text(header_text)
                    .size(tokens.typography().title_medium.size)
                    .color(tokens.colors.on_surface_variant)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(iced::Alignment::Start)
                    .align_y(iced::Alignment::Center),
            )
            .width(width)
            .height(Length::Fill)
            .padding(Padding::from([8.0, 8.0]))
            .style(move |theme, status| {
                Self::header_cell_button_style(theme, status, tokens, is_sorted)
            })
            .on_press(Message::SortBy(column.id.clone()))
            .into()
        } else {
            container(
                text(header_text)
                    .size(tokens.typography().title_medium.size)
                    .color(tokens.colors.on_surface_variant),
            )
            .width(width)
            .height(Length::Fill)
            .padding(Padding::from([8.0, 8.0]))
            .style(data::MaterialDataTable::header_cell(
                tokens,
                column.sortable,
                is_sorted,
            ))
            .align_y(iced::Alignment::Center)
            .into()
        }
    }

    /// Style for header cell as a clickable button (entire cell area)
    fn header_cell_button_style(
        _theme: &iced::Theme,
        status: button::Status,
        tokens: &MaterialTokens,
        is_sorted: bool,
    ) -> button::Style {
        let colors = &tokens.colors;

        // Base colors for normal state
        let base_background = if is_sorted {
            colors.primary.container
        } else {
            colors.surface_variant
        };

        let background_color = match status {
            button::Status::Hovered => {
                // Create a more visible hover effect
                let hover_overlay = Color {
                    r: colors.on_surface_variant.r,
                    g: colors.on_surface_variant.g,
                    b: colors.on_surface_variant.b,
                    a: 0.08,
                };

                // Blend the hover overlay with the base background
                Some(Background::Color(Color {
                    r: base_background
                        .r
                        .mul_add(1.0 - hover_overlay.a, hover_overlay.r * hover_overlay.a),
                    g: base_background
                        .g
                        .mul_add(1.0 - hover_overlay.a, hover_overlay.g * hover_overlay.a),
                    b: base_background
                        .b
                        .mul_add(1.0 - hover_overlay.a, hover_overlay.b * hover_overlay.a),
                    a: base_background.a.max(hover_overlay.a),
                }))
            }
            button::Status::Pressed => {
                // Create a more visible pressed effect
                let pressed_overlay = Color {
                    r: colors.on_surface_variant.r,
                    g: colors.on_surface_variant.g,
                    b: colors.on_surface_variant.b,
                    a: 0.12,
                };

                // Blend the pressed overlay with the base background
                Some(Background::Color(Color {
                    r: base_background.r.mul_add(
                        1.0 - pressed_overlay.a,
                        pressed_overlay.r * pressed_overlay.a,
                    ),
                    g: base_background.g.mul_add(
                        1.0 - pressed_overlay.a,
                        pressed_overlay.g * pressed_overlay.a,
                    ),
                    b: base_background.b.mul_add(
                        1.0 - pressed_overlay.a,
                        pressed_overlay.b * pressed_overlay.a,
                    ),
                    a: base_background.a.max(pressed_overlay.a),
                }))
            }
            _ => Some(Background::Color(base_background)),
        };

        button::Style {
            background: background_color,
            text_color: colors.on_surface_variant,
            border: iced::Border {
                color: colors.outline_variant,
                width: 0.5,
                radius: iced::border::Radius::new(0.0),
            },
            shadow: iced::Shadow::default(),
        }
    }
}
