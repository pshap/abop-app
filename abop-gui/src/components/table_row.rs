// Table row component with Material Design 3 styling

use iced::widget::{button, container, row, text};
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

    /// Create a single row element
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

        let cells: Vec<Element<'a, Message>> = columns
            .iter()
            .map(|column| {
                log::debug!("Creating cell for column: {}", column.id);
                Self::create_cell(audiobook, column, tokens)
            })
            .collect();
        log::debug!("Created {} cells for row {}", cells.len(), row_index);

        let row_content = row(cells);

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
                .on_press(Message::SelectAudiobook(audiobook.id.clone()))
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
            data::ColumnWidth::Fill(factor) => Length::FillPortion(factor),
            data::ColumnWidth::Shrink | data::ColumnWidth::FitContent => Length::Shrink, // FitContent behaves like Shrink in Iced
        }; // Use consistent color with material tokens
        let text_color = iced::Color::WHITE; // Force white for visibility
        log::debug!("Table cell text: {} (column: {})", cell_text, column.id);
        container(
            text(cell_text)
                .size(tokens.typography().body_medium.size) // Restore original size
                .color(text_color),
        )
        .width(width)
        .padding(Padding::from([4, 8])) // MD3 compact: 4px vertical, 8px horizontal
        .into()
    }

    /// Style for selectable row buttons
    fn row_button_style(
        _theme: &iced::Theme,
        status: button::Status,
        tokens: &MaterialTokens,
        is_selected: bool,
        is_striped: bool,
        row_index: usize,
    ) -> button::Style {
        // Get the base style from MaterialDataTable
        let container_style =
            data::MaterialDataTable::table_row(tokens, row_index, is_selected, is_striped)(
                &iced::Theme::Dark,
            );

        // Convert container style to button style, adding hover effects
        let base_background = container_style.background;

        let background_color = match status {
            button::Status::Hovered => {
                // Add a subtle hover effect over the base color
                if let Some(Background::Color(base_color)) = base_background {
                    Some(Background::Color(Color {
                        a: (base_color.a + tokens.ui().hover_opacity_adjustment).min(1.0),
                        r: base_color.r,
                        g: base_color.g,
                        b: base_color.b,
                    }))
                } else {
                    Some(Background::Color(Color::from_rgba(
                        0.0,
                        0.0,
                        0.0,
                        tokens.ui().hover_opacity_adjustment,
                    )))
                }
            }
            button::Status::Pressed => {
                // Add a stronger press effect over the base color
                if let Some(Background::Color(base_color)) = base_background {
                    Some(Background::Color(Color {
                        a: (base_color.a + tokens.ui().pressed_opacity_adjustment).min(1.0),
                        r: base_color.r,
                        g: base_color.g,
                        b: base_color.b,
                    }))
                } else {
                    Some(Background::Color(Color::from_rgba(
                        0.0,
                        0.0,
                        0.0,
                        tokens.ui().pressed_opacity_adjustment,
                    )))
                }
            }
            _ => base_background,
        }; // Use the consistent text color from material tokens
        let text_color = tokens.colors.on_surface;

        button::Style {
            background: background_color,
            text_color,
            border: container_style.border,
            shadow: container_style.shadow,
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
