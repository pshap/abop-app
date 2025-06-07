// Table header component with Material Design 3 styling

use iced::widget::{button, container, row, text};
use iced::{Background, Color, Element, Length, Padding};

use crate::messages::Message;
use crate::state::TableState;
use crate::styling::material::MaterialTokens;
use crate::styling::material::components::data;

/// Component for creating table headers with Material Design styling
pub struct TableHeader;

impl TableHeader {
    /// Create a header element from columns and state
    #[must_use]
    pub fn create<'a>(
        columns: &[data::TableColumn],
        state: &TableState,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        let header_row = columns
            .iter()
            .enumerate()
            .map(|(index, column)| Self::create_header_cell(column, index, state, tokens));

        container(
            row(header_row)
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

        let cell_content: Element<'a, Message> = if column.sortable {
            button(
                text(header_text)
                    .size(tokens.typography().title_medium.size) // Use title_medium per MD3 spec
                    .color(tokens.colors.on_surface_variant),
            )
            .style(move |theme, status| Self::header_button_style(theme, status, tokens))
            .on_press(Message::SortBy(column.id.clone()))
            .into()
        } else {
            text(header_text)
                .size(tokens.typography().title_medium.size) // Use title_medium per MD3 spec
                .color(tokens.colors.on_surface_variant)
                .into()
        };
        let width = match column.width {
            data::ColumnWidth::Fixed(w) => Length::Fixed(w),
            data::ColumnWidth::Fill(factor) => Length::FillPortion(factor),
            data::ColumnWidth::Shrink | data::ColumnWidth::FitContent => Length::Shrink, // FitContent behaves like Shrink in Iced
        };

        container(
            container(cell_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_y(Length::Fill),
        )
        .width(width)
        .height(Length::Fill)
        .padding(Padding::from([8.0, 8.0])) // MD3 standard: 8px vertical, 8px horizontal
        .style(data::MaterialDataTable::header_cell(
            tokens,
            column.sortable,
            is_sorted,
        ))
        .align_y(iced::Alignment::Center)
        .into()
    }

    /// Style for sortable header buttons
    fn header_button_style(
        _theme: &iced::Theme,
        status: button::Status,
        tokens: &MaterialTokens,
    ) -> button::Style {
        let header_text_color = tokens.colors.on_surface_variant;

        let background_color = match status {
            button::Status::Hovered => Some(Background::Color(Color {
                a: 0.08,
                ..header_text_color
            })),
            button::Status::Pressed => Some(Background::Color(Color {
                a: 0.12,
                ..header_text_color
            })),
            _ => None,
        };

        button::Style {
            background: background_color,
            text_color: header_text_color,
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        }
    }
}
