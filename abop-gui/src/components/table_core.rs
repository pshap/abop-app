// Core table component with Material Design 3 styling

use iced::widget::container::Style as ContainerStyle;
use iced::widget::{column, container, scrollable, text};
use iced::{Element, Length};
use std::collections::HashSet;

use abop_core::models::Audiobook;

use crate::messages::Message;
use crate::state::TableState;
use crate::styling::material::MaterialTokens;
use crate::styling::material::components::data;

use super::table_header::TableHeader;
use super::table_row::TableRow;

/// Main table component for displaying the audiobook library
pub struct AudiobookTable;

impl AudiobookTable {
    /// Create default configuration for audiobook table
    #[must_use]
    pub const fn default_config() -> data::DataTableConfig {
        data::DataTableConfig {
            selectable: true,
            hoverable: true,
            sticky_header: true,
            striped: false,
            virtual_scrolling: false,
            max_visible_rows: None,
            row_actions: false,
            resizable_columns: false,
            min_column_width: 120.0,
            density: data::TableDensity::Compact,
        }
    }

    /// Define columns for audiobook table
    #[must_use]
    pub fn define_columns() -> Vec<data::TableColumn> {
        vec![
            data::TableColumn::new("title", "Title")
                .width(data::ColumnWidth::Fill(6))
                .align(data::TextAlignment::Start)
                .sortable(true),
            data::TableColumn::new("author", "Author")
                .width(data::ColumnWidth::Fill(4))
                .align(data::TextAlignment::Start)
                .sortable(true),
            data::TableColumn::new("duration", "Duration")
                .width(data::ColumnWidth::Fill(2))
                .align(data::TextAlignment::End)
                .sortable(true),
            data::TableColumn::new("size", "Size")
                .width(data::ColumnWidth::Fill(2))
                .align(data::TextAlignment::End)
                .sortable(true),
        ]
    }
    /// Renders the enhanced Material Design audiobook table view
    #[must_use]
    pub fn view<'a>(
        audiobooks: &'a [Audiobook],
        selected: &'a HashSet<String>,
        table_state: &'a TableState,
        material_tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        log::debug!(
            "AudiobookTable::view called with {} audiobooks",
            audiobooks.len()
        );
        log::debug!(
            "Rendering table with {} audiobooks, {} selected",
            audiobooks.len(),
            selected.len()
        );
        let config = Self::default_config();
        let columns = Self::define_columns(); // Create the main table content
        let mut table = column![]; // Create header
        let header = TableHeader::create(&columns, table_state, material_tokens);
        table = table.push(header);
        log::debug!(
            "Table header created, audiobooks count: {}",
            audiobooks.len()
        );

        // Create table body
        if audiobooks.is_empty() {
            log::debug!("EMPTY STATE: No audiobooks found");
            // Empty state with Material Design styling
            let empty_state = container(
                text("No audiobooks found. Select a directory and scan to find audiobooks.")
                    .size(16)
                    .color(material_tokens.colors.on_surface),
            )
            .width(Length::Fill)
            .padding(material_tokens.spacing().md)
            .style(move |_theme| ContainerStyle {
                text_color: Some(material_tokens.colors.on_surface),
                ..Default::default()
            });

            table = table.push(empty_state);
        } else {
            log::debug!("CREATING ROWS: {} audiobooks", audiobooks.len());
            // Create rows for all audiobooks at once
            let rows =
                TableRow::create_rows(audiobooks, &columns, selected, material_tokens, &config);
            log::debug!("ROWS CREATED: {} rows", rows.len());

            // Add all rows at once instead of one by one to improve performance
            table = table.extend(rows);
            log::debug!("All rows added to table");
        }

        // Create a fixed height container for the table
        let table_container = if audiobooks.is_empty() {
            // Show a message when no audiobooks are available
            container(
                text("No audiobooks found. Click the folder icon to select a directory.")
                    .size(16)
                    .color(material_tokens.colors.on_surface),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .style(data::MaterialDataTable::table_container(material_tokens))
        } else {
            // Create a scrollable container for the table
            let scrollable_content = scrollable(
                container(table)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .style(data::MaterialDataTable::table_container(material_tokens)),
            )
            .width(Length::Fill)
            .height(Length::Fill);

            // Wrap in a container with explicit height
            container(scrollable_content)
                .width(Length::Fill)
                .height(Length::Fill)
        };

        table_container.into()
    }
}
