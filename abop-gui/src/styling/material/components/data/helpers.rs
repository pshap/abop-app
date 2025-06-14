//! Helper functions and utilities for Material Design 3 data components

use iced::alignment::Horizontal;
use iced::widget::Container;
use iced::{Element, Length, Padding};

use super::types::{
    ColumnDataType, ColumnWidth, DataTableConfig, SortDirection, SortState, TableColumn,
    TableDensity, TextAlignment,
};
use crate::styling::material::MaterialTokens;

/// Helper function to convert column width to iced Length
pub fn column_width_to_length(width: &ColumnWidth) -> Length {
    match width {
        ColumnWidth::Fixed(w) => Length::Fixed(*w),
        ColumnWidth::FillPortion(p) => Length::FillPortion(*p),
        ColumnWidth::Auto => Length::Shrink,
        ColumnWidth::Ratio(num, _den) => Length::FillPortion(*num as u16),
        ColumnWidth::Shrink => Length::Shrink,
    }
}

/// Performance optimization helpers
pub mod performance_helpers {
    /// Determine if virtual scrolling should be used based on data size
    pub fn should_use_virtual_scrolling(row_count: usize, threshold: usize) -> bool {
        row_count > threshold
    }

    /// Calculate buffer size for virtual scrolling
    pub fn calculate_buffer_size(visible_rows: usize) -> usize {
        (visible_rows as f32 * 0.5).ceil() as usize
    }
}

/// Interaction and user experience helpers
pub mod interaction_helpers {
    use super::*;

    /// Convert text alignment to iced horizontal alignment
    pub fn alignment_to_iced(align: TextAlignment) -> Horizontal {
        match align {
            TextAlignment::Start => Horizontal::Left,
            TextAlignment::Center => Horizontal::Center,
            TextAlignment::End => Horizontal::Right,
        }
    }

    /// Calculate responsive cell padding
    pub fn responsive_cell_padding(density: TableDensity) -> Padding {
        match density {
            TableDensity::Compact => Padding::new(6.0),
            TableDensity::Standard => Padding::new(8.0),
            TableDensity::Comfortable => Padding::new(12.0),
        }
    }
}

/// Public API helpers for advanced data table functionality
pub mod data_table_helpers {
    use super::super::table::MaterialDataTable;
    use super::{
        ColumnWidth, Container, DataTableConfig, Element, Length, MaterialTokens, SortDirection,
        SortState, TableColumn, TableDensity, column_width_to_length, interaction_helpers,
        performance_helpers,
    };

    /// Create optimized table configuration for large datasets
    pub fn create_large_dataset_config(
        row_count: usize,
        viewport_height: f32,
        density: TableDensity,
    ) -> DataTableConfig {
        let row_height = density.row_height();
        let rows_per_viewport = (viewport_height / row_height).ceil() as usize;
        let use_virtual = performance_helpers::should_use_virtual_scrolling(row_count, 100);

        DataTableConfig {
            virtual_scrolling: use_virtual,
            max_visible_rows: if use_virtual {
                Some(
                    rows_per_viewport
                        + performance_helpers::calculate_buffer_size(rows_per_viewport),
                )
            } else {
                None
            },
            density,
            ..Default::default()
        }
    }

    /// Calculate column widths for a set of columns
    pub fn calculate_column_widths(
        columns: &[TableColumn],
        available_width: f32,
    ) -> Vec<(String, Length)> {
        let mut widths = Vec::new();
        let mut remaining_width = available_width;
        let mut fill_portions = 0u16;

        // First pass: handle fixed widths
        for column in columns {
            match &column.width {
                ColumnWidth::Fixed(w) => {
                    widths.push((column.id.clone(), Length::Fixed(*w)));
                    remaining_width -= w;
                }
                ColumnWidth::FillPortion(p) => {
                    fill_portions += p;
                    widths.push((column.id.clone(), Length::FillPortion(*p)));
                }
                ColumnWidth::Auto => {
                    // Auto columns get minimum viable width for now
                    let auto_width = 120.0;
                    widths.push((column.id.clone(), Length::Fixed(auto_width)));
                    remaining_width -= auto_width;
                }
                ColumnWidth::Shrink => {
                    widths.push((column.id.clone(), Length::Shrink));
                }
                ColumnWidth::Ratio(num, den) => {
                    let ratio_width = (remaining_width * (*num as f32)) / (*den as f32);
                    widths.push((column.id.clone(), Length::Fixed(ratio_width)));
                    remaining_width -= ratio_width;
                }
            }
        }

        widths
    }

    /// Create responsive table cell with proper padding
    pub fn create_responsive_cell<'a, T>(
        content: Element<'a, T>,
        column: &TableColumn,
        tokens: &'a MaterialTokens,
        density: TableDensity,
    ) -> Container<'a, T> {
        let padding = interaction_helpers::responsive_cell_padding(density);
        let align = interaction_helpers::alignment_to_iced(column.alignment);

        Container::new(content)
            .padding(padding)
            .width(column_width_to_length(&column.width))
            .align_x(align)
            .style(MaterialDataTable::table_cell(tokens, column.alignment))
    }

    /// Create optimized data row with virtual scrolling support
    pub fn create_data_row<'a, T: Clone + 'a>(
        row_data: &[T],
        columns: &'a [TableColumn],
        row_index: usize,
        tokens: &'a MaterialTokens,
        config: &DataTableConfig,
        format_cell: impl Fn(&T, &TableColumn, usize) -> Element<'a, crate::messages::Message>,
    ) -> Vec<Element<'a, crate::messages::Message>> {
        columns
            .iter()
            .enumerate()
            .map(|(col_index, column)| {
                if let Some(data) = row_data.get(col_index) {
                    let cell_content = format_cell(data, column, row_index);
                    create_responsive_cell(cell_content, column, tokens, config.density).into()
                } else {
                    let empty_content = MaterialDataTable::body_text("", tokens);
                    create_responsive_cell(empty_content, column, tokens, config.density).into()
                }
            })
            .collect()
    }

    /// Filter and sort data for table display
    pub fn process_table_data<T: Clone>(
        data: &[T],
        sort_state: Option<&SortState>,
        filters: &[(String, String)],
        get_cell_value: impl Fn(&T, &str) -> String,
        compare_values: impl Fn(&T, &T, &str) -> std::cmp::Ordering,
    ) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..data.len()).collect();

        // Apply filters first
        if !filters.is_empty() {
            indices = apply_filters(data, filters, &get_cell_value);
        }

        // Apply sorting
        if let Some(sort) = sort_state {
            indices.sort_by(|&a, &b| {
                let ordering = compare_values(&data[a], &data[b], &sort.column_id);
                match sort.direction {
                    SortDirection::Ascending => ordering,
                    SortDirection::Descending => ordering.reverse(),
                }
            });
        }

        indices
    }

    /// Apply filters to data and return filtered indices
    fn apply_filters<T>(
        data: &[T],
        filters: &[(String, String)],
        get_cell_value: impl Fn(&T, &str) -> String,
    ) -> Vec<usize> {
        data.iter()
            .enumerate()
            .filter(|(_, item)| {
                filters.iter().all(|(column_id, filter_value)| {
                    if filter_value.is_empty() {
                        return true;
                    }
                    let cell_value = get_cell_value(item, column_id).to_lowercase();
                    cell_value.contains(&filter_value.to_lowercase())
                })
            })
            .map(|(index, _)| index)
            .collect()
    }
}

/// Utility functions for table operations
pub mod table_utils {
    use super::*;

    /// Create a standard table column configuration
    pub fn create_standard_column(
        id: &str,
        title: &str,
        width: ColumnWidth,
        sortable: bool,
    ) -> TableColumn {
        TableColumn::new(id, title)
            .width(width)
            .sortable(sortable)
            .align(TextAlignment::Start)
            .data_type(ColumnDataType::Text)
    }

    /// Create a numeric column with right alignment
    pub fn create_numeric_column(id: &str, title: &str, width: ColumnWidth) -> TableColumn {
        TableColumn::new(id, title)
            .width(width)
            .sortable(true)
            .align(TextAlignment::End)
            .data_type(ColumnDataType::Number)
    }

    /// Create a date column with center alignment
    pub fn create_date_column(id: &str, title: &str, width: ColumnWidth) -> TableColumn {
        TableColumn::new(id, title)
            .width(width)
            .sortable(true)
            .align(TextAlignment::Center)
            .data_type(ColumnDataType::Date)
    }

    /// Get default table configuration optimized for performance
    pub fn get_performance_config() -> DataTableConfig {
        DataTableConfig {
            virtual_scrolling: true,
            max_visible_rows: Some(50),
            density: TableDensity::Standard,
            striped: true,
            hoverable: true,
            sticky_header: true,
            resizable_columns: true,
            ..Default::default()
        }
    }

    /// Calculate optimal table dimensions
    pub fn calculate_table_dimensions(
        row_count: usize,
        column_count: usize,
        density: TableDensity,
    ) -> (f32, f32) {
        let row_height = density.row_height();
        let header_height = density.header_height();
        let min_column_width = 120.0;

        let total_height = header_height + (row_count as f32 * row_height);
        let total_width = column_count as f32 * min_column_width;

        (total_width, total_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_width_conversion() {
        assert_eq!(
            column_width_to_length(&ColumnWidth::Fixed(100.0)),
            Length::Fixed(100.0)
        );
        assert_eq!(
            column_width_to_length(&ColumnWidth::FillPortion(2)),
            Length::FillPortion(2)
        );
        assert_eq!(column_width_to_length(&ColumnWidth::Auto), Length::Shrink);
    }

    #[test]
    fn test_text_alignment_conversion() {
        assert_eq!(
            interaction_helpers::alignment_to_iced(TextAlignment::Start),
            Horizontal::Left
        );
        assert_eq!(
            interaction_helpers::alignment_to_iced(TextAlignment::Center),
            Horizontal::Center
        );
        assert_eq!(
            interaction_helpers::alignment_to_iced(TextAlignment::End),
            Horizontal::Right
        );
    }
}
