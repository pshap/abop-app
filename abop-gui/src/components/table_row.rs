// Table row component with Material Design 3 styling and selection support

use iced::widget::{button, checkbox, container, row, text};
use iced::{Background, Color, Element, Length, Padding};
use std::collections::HashSet;

use abop_core::models::Audiobook;

use crate::messages::Message;
use crate::styling::material::MaterialTokens;
use crate::styling::material::components::data;

/// Component for creating table rows with Material Design styling
pub struct TableRow;

impl TableRow {
    /// Create a collection of row elements from audiobooks
    #[must_use]
    pub fn create_rows<'a>(
        audiobooks: &'a [Audiobook],
        columns: &[data::TableColumn],
        selected_items: &HashSet<String>,
        tokens: &'a MaterialTokens,
        config: &data::DataTableConfig,
    ) -> Vec<Element<'a, Message>> {
        log::debug!("Creating {} rows for audiobooks", audiobooks.len());
        let result: Vec<Element<'a, Message>> = audiobooks
            .iter()
            .enumerate()
            .map(|(index, audiobook)| {
                Self::create_single_row(audiobook, columns, selected_items, tokens, config, index)
            })
            .collect();
        log::debug!("Finished creating rows, total: {}", result.len());
        result
    }
    /// Create a single row element with selection checkbox
    fn create_single_row<'a>(
        audiobook: &'a Audiobook,
        columns: &[data::TableColumn],
        selected_items: &HashSet<String>,
        tokens: &'a MaterialTokens,
        config: &data::DataTableConfig,
        row_index: usize,
    ) -> Element<'a, Message> {
        let is_selected = selected_items.contains(&audiobook.id);
        let is_striped = config.striped && row_index % 2 == 1;

        // Create enhanced selection checkbox with improved Material Design 3 styling
        let mut all_cells = vec![];

        let selection_checkbox = checkbox("", is_selected)
            .on_toggle(|_| Message::ToggleAudiobookSelection(audiobook.id.clone()))
            .size(20)
            .style(move |_theme, _status| {
                // Enhanced Material Design styling with better visual feedback
                iced::widget::checkbox::Style {
                    background: Background::Color(if is_selected {
                        tokens.colors.primary.base
                    } else {
                        Color::TRANSPARENT
                    }),
                    icon_color: if is_selected {
                        tokens.colors.primary.on_base
                    } else {
                        tokens.colors.primary.base
                    },
                    border: iced::Border {
                        color: if is_selected {
                            tokens.colors.primary.base
                        } else {
                            tokens.colors.outline
                        },
                        width: 2.0,
                        radius: 4.0.into(),
                    },
                    text_color: Some(tokens.colors.on_surface),
                }
            });

        let checkbox_cell = container(selection_checkbox)
            .width(Length::Fixed(48.0))
            .height(Length::Fill)
            .padding(Padding::from([8.0, 8.0]))
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center);

        all_cells.push(checkbox_cell.into());

        // Create data cells
        let data_cells: Vec<Element<'a, Message>> = columns
            .iter()
            .map(|column| {
                log::debug!("Creating cell for column: {}", column.id);
                Self::create_cell(audiobook, column, tokens)
            })
            .collect();
        log::debug!("Created {} cells for row {}", data_cells.len(), row_index);

        all_cells.extend(data_cells);

        let row_content = row(all_cells);

        // Make the entire row clickable for selection (excluding checkbox area)
        let styled_row: Element<'a, Message> = if config.selectable {
            button(row_content)
                .style(move |theme, status| {
                    Self::row_button_style(
                        theme,
                        status,
                        tokens,
                        is_selected,
                        is_striped,
                        row_index,
                    )
                })
                .on_press(Message::ToggleAudiobookSelection(audiobook.id.clone()))
                .width(Length::Fill)
                .into()
        } else {
            container(row_content)
                .style(data::MaterialDataTable::table_row(
                    tokens,
                    row_index,
                    is_selected,
                    is_striped,
                ))
                .width(Length::Fill)
                .into()
        };
        container(styled_row)
            .width(Length::Fill)
            .height(Length::Fixed(config.density.row_height())) // Ensure each row has a proper height
            .padding(Padding::from([2, 0])) // Minimal vertical padding, no horizontal padding
            .into()
    }

    /// Create individual cell content
    fn create_cell<'a>(
        audiobook: &'a Audiobook,
        column: &data::TableColumn,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        let cell_text = match column.id.as_str() {
            "title" => audiobook
                .title
                .as_deref()
                .unwrap_or(&audiobook.id)
                .to_string(),
            "author" => audiobook.author.as_deref().unwrap_or("Unknown").to_string(),
            "duration" => Self::format_duration(audiobook.duration_seconds.unwrap_or(0)),
            "size" => Self::format_file_size(audiobook.size_bytes.unwrap_or(0)),
            _ => String::new(),
        };
        let width = match column.width {
            data::ColumnWidth::Fixed(w) => Length::Fixed(w),
            data::ColumnWidth::FillPortion(factor) => Length::FillPortion(factor),
            data::ColumnWidth::Auto => Length::Shrink,
            data::ColumnWidth::Ratio(num, _den) => Length::FillPortion(num as u16),
            data::ColumnWidth::Shrink => Length::Shrink,
        };

        // Use proper Material Design text color
        let text_color = tokens.colors.on_surface;
        log::debug!("Table cell text: {} (column: {})", cell_text, column.id);

        container(
            text(cell_text)
                .size(tokens.typography().body_medium.size)
                .line_height(iced::widget::text::LineHeight::Absolute(
                    tokens.typography().body_medium.line_height,
                ))
                .color(text_color),
        )
        .width(width)
        .padding(Padding::from([8, 8])) // Increased padding for Standard density
        .align_y(iced::alignment::Vertical::Center) // Center text vertically
        .into()
    }

    /// Style for selectable row buttons with Material Design 3 hover effects
    fn row_button_style(
        _theme: &iced::Theme,
        status: button::Status,
        tokens: &MaterialTokens,
        is_selected: bool,
        is_striped: bool,
        row_index: usize,
    ) -> button::Style {
        let colors = &tokens.colors;

        // Get the base background color using Material Design patterns
        let base_background = if is_selected {
            colors.secondary.container
        } else if is_striped && row_index % 2 == 1 {
            colors.surface_container
        } else {
            colors.surface_container_lowest
        };

        let background_color = match status {
            button::Status::Hovered => {
                // Create a visible hover effect using Material Design overlay technique
                // This creates a subtle but noticeable visual feedback on hover
                let hover_overlay = Color {
                    r: colors.on_surface_variant.r,
                    g: colors.on_surface_variant.g,
                    b: colors.on_surface_variant.b,
                    a: 0.08, // Material Design 3 hover overlay opacity
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
                    a: 0.12, // Material Design 3 pressed overlay opacity
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
            text_color: colors.on_surface,
            border: iced::Border {
                color: colors.outline_variant,
                width: 0.5,
                radius: iced::border::Radius::new(0.0),
            },
            shadow: iced::Shadow::default(),
        }
    }

    /// Format duration in seconds to human-readable string
    fn format_duration(seconds: u64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;

        if hours > 0 {
            format!("{hours}:{minutes:02}:{secs:02}")
        } else {
            format!("{minutes}:{secs:02}")
        }
    }

    /// Format file size in bytes to human-readable string
    fn format_file_size(bytes: u64) -> String {
        // Use the unified file size formatting from the casting module
        abop_core::utils::casting::format_file_size_standard(bytes)
    }
}
