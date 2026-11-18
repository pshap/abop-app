// Core table component with Material Design 3 styling

use iced::widget::{column, rule, scrollable};
use iced::{Element, Length};
use std::collections::HashSet;
use std::sync::Arc;

use abop_core::models::Audiobook;

use crate::messages::Message;
use crate::state::TableState;
use crate::styling::material::MaterialTokens;
use crate::styling::material::components::data;
use crate::utils::image_cache::ImageCache;

use super::table_header::TableHeader;
use super::table_row::{RowContext, TableRow};

/// Main table component for displaying the audiobook library
pub struct AudiobookTable;

impl AudiobookTable {
    /// Named constant for cover art column width
    const COVER_ART_COLUMN_WIDTH: f32 = 56.0;
    /// Create default configuration for audiobook table
    #[must_use]
    pub fn default_config() -> data::DataTableConfig {
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
            density: data::TableDensity::Standard,
            // Use Default for the remaining fields
            ..Default::default()
        }
    }

    /// Define columns for audiobook table
    #[must_use]
    pub fn define_columns() -> Vec<data::TableColumn> {
        vec![
            data::TableColumn::new("cover_art", "")
                .width(data::ColumnWidth::Fixed(Self::COVER_ART_COLUMN_WIDTH))
                .align(data::TextAlignment::Center)
                .sortable(false),
            data::TableColumn::new("title", "Title")
                .width(data::ColumnWidth::FillPortion(6))
                .align(data::TextAlignment::Start)
                .sortable(true),
            data::TableColumn::new("author", "Author")
                .width(data::ColumnWidth::FillPortion(4))
                .align(data::TextAlignment::Start)
                .sortable(true),
            data::TableColumn::new("duration", "Duration")
                .width(data::ColumnWidth::FillPortion(2))
                .align(data::TextAlignment::End)
                .sortable(true),
            data::TableColumn::new("size", "Size")
                .width(data::ColumnWidth::FillPortion(2))
                .align(data::TextAlignment::End)
                .sortable(true),
            data::TableColumn::new("actions", "")
                .width(data::ColumnWidth::Fixed(64.0))
                .align(data::TextAlignment::Center)
                .sortable(false),
        ]
    }
    /// Renders the audiobook table: a fixed header above a scrollable list of rows
    ///
    /// Empty, scanning and no-results states are handled by the library view.
    #[must_use]
    pub fn view<'a>(
        audiobooks: &'a [Audiobook],
        selected: &'a HashSet<String>,
        table_state: &'a TableState,
        material_tokens: &'a MaterialTokens,
        image_cache: &'a Arc<ImageCache>,
        now_playing: Option<&std::path::Path>,
    ) -> Element<'a, Message> {
        let config = Self::default_config();
        let columns = Self::define_columns();

        let header = TableHeader::create(
            &columns,
            table_state,
            material_tokens,
            selected,
            audiobooks.len(),
        );

        let rows = TableRow::create_rows(
            audiobooks,
            &RowContext {
                columns: &columns,
                selected,
                config: &config,
                now_playing,
                tokens: material_tokens,
                image_cache,
            },
        );
        let divider = move || -> Element<'a, Message> {
            rule::Rule::horizontal(1)
                .style(move |_theme| rule::Style {
                    color: material_tokens.colors.outline_variant,
                    width: 1,
                    radius: 0.0.into(),
                    fill_mode: rule::FillMode::Full,
                })
                .into()
        };
        let mut body = column![].width(Length::Fill);
        for row in rows {
            body = body.push(row).push(divider());
        }

        column![
            header,
            divider(),
            scrollable(body).width(Length::Fill).height(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
