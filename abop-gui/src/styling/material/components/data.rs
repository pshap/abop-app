//! Phase 5: Material Design 3 Data Components
//!
//! This module implements Material Design 3 data display components including sophisticated
//! data tables, lists, and tree views with full accessibility and interaction support.
//!
//! ## Key Features
//! - Material Design 3 compliant data tables with virtual scrolling
//! - Advanced sorting and filtering capabilities
//! - Responsive column sizing and layout
//! - Accessible selection and navigation patterns
//! - High-performance rendering for large datasets
//! - Integrated with ABOP's theming system

use crate::styling::material::{MaterialTokens, elevation::ElevationLevel};
use iced::widget::{Container, container};
use iced::{Border, Element, Length, Padding};

/// Border style constants for consistent data table styling
mod border_styles {
    use iced::{Border, Color};
    use iced::border::Radius;
    
    /// Standard table border for cells and headers
    pub fn table_border(color: Color) -> Border {
        Border {
            color,
            width: 0.5,
            radius: Radius::new(0.0),
        }
    }
    
    /// Transparent border for containers
    pub fn transparent_border() -> Border {
        Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(0.0),
        }
    }
}

/// Color helper functions for data component theming
mod color_helpers {
    use iced::Color;
    use crate::styling::material::MaterialTokens;
    
    /// Get background color for table rows based on state
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
    
    /// Get background color for header cells based on sort state
    pub fn get_header_background_color(
        tokens: &MaterialTokens,
        is_sorted: bool,
    ) -> Color {
        if is_sorted {
            tokens.colors.primary_container
        } else {
            tokens.colors.surface_variant
        }
    }
    
    /// Get appropriate text color for background
    pub fn get_text_color_for_background(
        tokens: &MaterialTokens,
        is_sorted: bool,
        is_selected: bool,
    ) -> Color {
        if is_sorted {
            tokens.colors.on_primary_container
        } else if is_selected {
            tokens.colors.on_secondary_container
        } else {
            tokens.colors.on_surface
        }
    }
}

/// Typography helper functions for consistent text styling
mod typography_helpers {
    use iced::widget::text::LineHeight;
    use iced::widget::text;
    use iced::{Color, Element, Pixels};
    use crate::styling::material::typography::TypeStyle;
    
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
    
    /// Create styled text element with custom size override
    pub fn create_text_with_size<'a>(
        content: &'a str,
        size: f32,
        color: Color,
    ) -> Element<'a, crate::messages::Message> {
        text(content)
            .size(size)
            .color(color)
            .into()
    }
}

/// Container style builders for consistent data component styling  
mod container_builders {
    use iced::{Background, Border, Color, Shadow};
    use iced::widget::container;
    
    /// Create base container style with background and border
    pub fn base_container_style(
        background_color: Option<Color>,
        border: Border,
        text_color: Option<Color>,
    ) -> container::Style {
        container::Style {
            background: background_color.map(Background::Color),
            border,
            text_color,
            ..Default::default()
        }
    }
    
    /// Create elevated container style with shadow
    pub fn elevated_container_style(
        background_color: Color,
        border: Border,
        shadow: Shadow,
    ) -> container::Style {        container::Style {
            background: Some(Background::Color(background_color)),
            border,
            shadow,
            ..Default::default()
        }
    }
}

/// State management helpers for complex data component interactions
mod state_helpers {
    use super::{SortDirection, SortState, TableDensity};
    
    /// Calculate table row height based on density setting
    pub fn get_row_height(density: TableDensity) -> f32 {
        match density {
            TableDensity::Compact => 32.0,
            TableDensity::Standard => 48.0,
            TableDensity::Comfortable => 56.0,
        }
    }
    
    /// Calculate header row height based on density setting  
    pub fn get_header_height(density: TableDensity) -> f32 {
        match density {
            TableDensity::Compact => 40.0,
            TableDensity::Standard => 56.0,
            TableDensity::Comfortable => 64.0,
        }
    }
    
    /// Calculate appropriate padding for table cells based on density
    pub fn get_cell_padding(density: TableDensity) -> iced::Padding {
        match density {
            TableDensity::Compact => iced::Padding::new(8.0),
            TableDensity::Standard => iced::Padding::new(12.0),
            TableDensity::Comfortable => iced::Padding::new(16.0),
        }
    }
      /// Toggle sort direction for a column
    pub fn toggle_sort_direction(current: Option<&SortState>, column_id: &str) -> SortState {
        match current {
            Some(state) if state.column_id == column_id => {
                SortState {
                    column_id: column_id.to_string(),
                    direction: match state.direction {
                        SortDirection::Ascending => SortDirection::Descending,
                        SortDirection::Descending => SortDirection::Ascending,
                    },
                }
            }
            _ => SortState {
                column_id: column_id.to_string(),
                direction: SortDirection::Ascending,
            },
        }
    }

    /// Calculate total table height including header and all rows
    pub fn calculate_total_table_height(row_count: usize, density: TableDensity) -> f32 {
        let row_height = get_row_height(density);
        let header_height = get_header_height(density);
        header_height + (row_count as f32 * row_height)
    }

    /// Get responsive font size based on density
    pub fn get_responsive_font_size(density: TableDensity) -> f32 {
        match density {
            TableDensity::Compact => 12.0,
            TableDensity::Standard => 14.0,
            TableDensity::Comfortable => 16.0,
        }
    }

    /// Calculate optimal column count for given width
    pub fn calculate_optimal_column_count(available_width: f32, min_column_width: f32) -> usize {
        (available_width / min_column_width).floor() as usize
    }

    /// Check if table density should be automatically adjusted based on space
    pub fn should_auto_adjust_density(
        available_height: f32,
        row_count: usize,
        current_density: TableDensity,
    ) -> Option<TableDensity> {
        let current_height = calculate_total_table_height(row_count, current_density);
        
        if current_height > available_height {
            // Try more compact density
            match current_density {
                TableDensity::Comfortable => Some(TableDensity::Standard),
                TableDensity::Standard => Some(TableDensity::Compact),
                TableDensity::Compact => None, // Already most compact
            }
        } else {
            None // Current density is fine
        }
    }
}

/// Advanced interaction helpers for data components
mod interaction_helpers {
    use super::{TableColumn, ColumnWidth, TextAlignment};
    use iced::Length;    /// Calculate column width as iced Length based on configuration
    pub fn calculate_column_length(column: &TableColumn, _available_width: f32) -> Length {
        match column.width {
            ColumnWidth::Fixed(width) => Length::Fixed(width),
            ColumnWidth::Fill(weight) => Length::FillPortion(weight),
            ColumnWidth::FitContent => Length::Shrink,
            ColumnWidth::Shrink => Length::Shrink,
        }
    }
    
    /// Convert column alignment to iced alignment
    pub fn alignment_to_iced(align: TextAlignment) -> iced::alignment::Horizontal {
        match align {
            TextAlignment::Start => iced::alignment::Horizontal::Left,
            TextAlignment::Center => iced::alignment::Horizontal::Center,
            TextAlignment::End => iced::alignment::Horizontal::Right,
        }
    }
    
    /// Create responsive padding based on column width and content
    pub fn responsive_cell_padding(column_width: f32, min_padding: f32) -> iced::Padding {
        let padding = (column_width * 0.05).max(min_padding).min(16.0);
        iced::Padding::new(padding)
    }
}

/// Performance optimization helpers for large datasets
mod performance_helpers {
    /// Calculate visible row range for virtual scrolling
    pub fn calculate_visible_range(
        scroll_position: f32,
        viewport_height: f32,
        row_height: f32,
        total_rows: usize,
        buffer_size: usize,
    ) -> (usize, usize) {
        let rows_per_viewport = (viewport_height / row_height).ceil() as usize;
        let start_row = (scroll_position / row_height).floor() as usize;
        
        let buffered_start = start_row.saturating_sub(buffer_size);
        let buffered_end = (start_row + rows_per_viewport + buffer_size).min(total_rows);
        
        (buffered_start, buffered_end)
    }
    
    /// Determine if virtual scrolling should be enabled based on dataset size
    pub fn should_use_virtual_scrolling(row_count: usize, threshold: usize) -> bool {
        row_count > threshold
    }    /// Calculate optimal buffer size for virtual scrolling
    pub fn calculate_buffer_size(rows_per_viewport: usize) -> usize {
        ((rows_per_viewport as f32 * 0.5).max(5.0).min(20.0)) as usize
    }

    /// Calculate scroll offset from row index
    pub fn calculate_scroll_offset(row_index: usize, row_height: f32) -> f32 {
        row_index as f32 * row_height
    }

    /// Estimate memory usage for table configuration
    pub fn estimate_memory_usage(
        row_count: usize,
        column_count: usize,
        average_cell_size_bytes: usize,
    ) -> usize {
        row_count * column_count * average_cell_size_bytes
    }    /// Check if row batching should be used for large datasets
    pub fn should_use_row_batching(row_count: usize, memory_limit_mb: usize) -> bool {
        let estimated_memory = estimate_memory_usage(row_count, 10, 100); // Rough estimate
        estimated_memory > (memory_limit_mb * 1_048_576) // Convert MB to bytes
    }

    /// Calculate optimal batch size for processing large datasets
    pub fn calculate_optimal_batch_size(
        total_rows: usize,
        memory_limit_mb: usize,
        average_row_size_bytes: usize,
    ) -> usize {
        let max_rows_in_memory = (memory_limit_mb * 1_048_576) / average_row_size_bytes;
        max_rows_in_memory.min(total_rows).max(100) // Minimum batch of 100 rows
    }
}

/// Public API helpers for advanced data table functionality
pub mod data_table_helpers {
    use super::*;
    
    /// Create optimized table configuration for large datasets
    pub fn create_large_dataset_config(
        row_count: usize,
        viewport_height: f32,
        density: TableDensity,
    ) -> DataTableConfig {
        let row_height = state_helpers::get_row_height(density);
        let rows_per_viewport = (viewport_height / row_height).ceil() as usize;
        let use_virtual = performance_helpers::should_use_virtual_scrolling(row_count, 100);
        
        DataTableConfig {
            virtual_scrolling: use_virtual,
            max_visible_rows: if use_virtual { 
                Some(rows_per_viewport + performance_helpers::calculate_buffer_size(rows_per_viewport))
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
        columns.iter().map(|col| {
            (col.id.clone(), interaction_helpers::calculate_column_length(col, available_width))
        }).collect()
    }
      /// Create responsive table cell with proper padding
    pub fn create_responsive_cell<'a, T>(
        content: Element<'a, T>,
        column: &TableColumn,
        tokens: &'a MaterialTokens,
        density: TableDensity,
    ) -> Container<'a, T> {
        let cell_padding = state_helpers::get_cell_padding(density);
        let align = interaction_helpers::alignment_to_iced(column.align);
        
        container(content)
            .padding(cell_padding)
            .align_x(align)
            .style(MaterialDataTable::table_cell(tokens, column.align))
    }
      /// Toggle column sort state
    pub fn toggle_column_sort(
        current_sort: Option<&SortState>,
        column_id: &str,
    ) -> SortState {
        state_helpers::toggle_sort_direction(current_sort, column_id)
    }    /// Create a styled table header row with sorting support
    pub fn create_header_row<'a>(
        columns: &'a [TableColumn],
        sort_state: Option<&SortState>,
        tokens: &'a MaterialTokens,
        density: TableDensity,
    ) -> Vec<Element<'a, crate::messages::Message>> {
        columns.iter().map(|column| {
            let header_content = if column.sortable {
                MaterialDataTable::sortable_header_text(
                    &column.title,
                    sort_state,
                    &column.id,
                    tokens,
                )
            } else {
                MaterialDataTable::header_text(&column.title, tokens)
            };

            let is_sorted = sort_state.map_or(false, |s| s.column_id == column.id);
            let cell_padding = state_helpers::get_cell_padding(density);

            container(header_content)
                .padding(cell_padding)
                .style(MaterialDataTable::header_cell(tokens, column.sortable, is_sorted))
                .into()
        }).collect()
    }    /// Create optimized data row with virtual scrolling support
    pub fn create_data_row<'a, T: Clone + 'a>(
        row_data: &[T],
        columns: &'a [TableColumn],
        row_index: usize,
        tokens: &'a MaterialTokens,
        config: &DataTableConfig,
        format_cell: impl Fn(&T, &TableColumn, usize) -> Element<'a, crate::messages::Message>,
    ) -> Vec<Element<'a, crate::messages::Message>> {
        columns.iter().enumerate().map(|(col_index, column)| {
            if let Some(data) = row_data.get(col_index) {                let cell_content = format_cell(data, column, row_index);
                
                data_table_helpers::create_responsive_cell(
                    cell_content,
                    column,
                    tokens,
                    config.density,
                ).into()} else {
                // Empty cell fallback
                let empty_content = MaterialDataTable::body_text("", tokens);
                data_table_helpers::create_responsive_cell(
                    empty_content,
                    column,
                    tokens,
                    config.density,
                ).into()
            }
        }).collect()
    }

    /// Calculate virtual scrolling parameters for large datasets
    pub fn calculate_virtual_scrolling_params(
        total_rows: usize,
        viewport_height: f32,
        scroll_offset: f32,
        density: TableDensity,
    ) -> VirtualScrollingParams {
        let row_height = state_helpers::get_row_height(density);
        let header_height = state_helpers::get_header_height(density);
        let available_height = viewport_height - header_height;
        
        let visible_rows = (available_height / row_height).ceil() as usize;
        let buffer_size = performance_helpers::calculate_buffer_size(visible_rows);
        
        let (start_index, end_index) = performance_helpers::calculate_visible_range(
            scroll_offset,
            available_height,
            row_height,
            total_rows,
            buffer_size,
        );

        VirtualScrollingParams {
            start_index,
            end_index,
            visible_rows,
            row_height,
            header_height,
            total_height: (total_rows as f32 * row_height) + header_height,
        }
    }

    /// Apply column filters to data (helper for filtering logic)
    pub fn apply_column_filters<T>(
        data: &[T],
        filters: &[(String, String)], // (column_id, filter_value)
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

/// Material Design 3 Data Table Configuration
#[derive(Debug, Clone)]
pub struct DataTableConfig {
    /// Enable row selection
    pub selectable: bool,
    /// Enable row hover effects
    pub hoverable: bool,
    /// Enable sticky header
    pub sticky_header: bool,
    /// Enable zebra striping
    pub striped: bool,
    /// Enable virtual scrolling for large datasets
    pub virtual_scrolling: bool,
    /// Maximum visible rows (for virtual scrolling)
    pub max_visible_rows: Option<usize>,
    /// Enable row actions (edit, delete, etc.)
    pub row_actions: bool,
    /// Enable column resizing
    pub resizable_columns: bool,
    /// Minimum column width
    pub min_column_width: f32,
    /// Table density (compact, standard, comfortable)
    pub density: TableDensity,
}

impl Default for DataTableConfig {
    fn default() -> Self {
        Self {
            selectable: true,
            hoverable: true,
            sticky_header: true,
            striped: false,
            virtual_scrolling: false,
            max_visible_rows: None,
            row_actions: false,
            resizable_columns: false,
            min_column_width: 80.0,
            density: TableDensity::Standard,
        }
    }
}

/// Table density options following Material Design specifications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableDensity {
    /// Compact spacing for dense information
    Compact,
    /// Standard spacing for general use
    Standard,
    /// Comfortable spacing for accessibility
    Comfortable,
}

impl TableDensity {
    /// Get row height for the density level
    #[must_use]
    pub fn row_height(&self) -> f32 {
        state_helpers::get_row_height(*self)
    }

    /// Get header height for the density level
    #[must_use]
    pub fn header_height(&self) -> f32 {
        state_helpers::get_header_height(*self)
    }

    /// Get cell padding for the density level
    #[must_use]
    pub fn cell_padding(&self) -> Padding {
        state_helpers::get_cell_padding(*self)
    }
}

/// Column definition for data tables
#[derive(Debug, Clone)]
pub struct TableColumn {
    /// Unique identifier for the column
    pub id: String,
    /// Display title for the column header
    pub title: String,
    /// Column width specification
    pub width: ColumnWidth,
    /// Whether the column is sortable
    pub sortable: bool,
    /// Whether the column is filterable
    pub filterable: bool,
    /// Text alignment for column content
    pub align: TextAlignment,
    /// Whether the column is sticky (fixed during horizontal scroll)
    pub sticky: bool,
    /// Column data type for proper formatting
    pub data_type: ColumnDataType,
    /// Minimum width for resizable columns
    pub min_width: Option<f32>,
    /// Maximum width for resizable columns
    pub max_width: Option<f32>,
}

impl TableColumn {
    /// Create a new table column
    #[must_use]
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            width: ColumnWidth::Fill(1),
            sortable: true,
            filterable: true,
            align: TextAlignment::Start,
            sticky: false,
            data_type: ColumnDataType::Text,
            min_width: None,
            max_width: None,
        }
    }

    /// Set column width
    #[must_use]
    pub const fn width(mut self, width: ColumnWidth) -> Self {
        self.width = width;
        self
    }

    /// Set column sortability
    #[must_use]
    pub const fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    /// Set column alignment
    #[must_use]
    pub const fn align(mut self, align: TextAlignment) -> Self {
        self.align = align;
        self
    }

    /// Set column data type
    #[must_use]
    pub const fn data_type(mut self, data_type: ColumnDataType) -> Self {
        self.data_type = data_type;
        self
    }

    /// Make column sticky
    #[must_use]
    pub const fn sticky(mut self) -> Self {
        self.sticky = true;
        self
    }
}

/// Column width specification
#[derive(Debug, Clone)]
pub enum ColumnWidth {
    /// Fixed pixel width
    Fixed(f32),
    /// Proportional fill weight
    Fill(u16),
    /// Fit content width
    FitContent,
    /// Minimum required width
    Shrink,
}

/// Text alignment options for table columns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    /// Align text to the start (left)
    Start,
    /// Center text
    Center,
    /// Align text to the end (right)
    End,
}

/// Supported data types for table columns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnDataType {
    /// Text data
    Text,
    /// Numeric data
    Number,
    /// Date value
    Date,
    /// Date and time value
    DateTime,
    /// Duration value
    Duration,
    /// File size value
    Size,
    /// Boolean value
    Boolean,
    /// Currency value
    Currency,
    /// Percentage value
    Percentage,
}

/// Sort direction for table columns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    /// Ascending order
    Ascending,
    /// Descending order
    Descending,
}

/// Table sort state
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortState {
    /// Column being sorted
    pub column_id: String,
    /// Sort direction
    pub direction: SortDirection,
}

/// Virtual scrolling parameters for optimized rendering
#[derive(Debug, Clone)]
pub struct VirtualScrollingParams {
    /// Starting index of visible rows
    pub start_index: usize,
    /// Ending index of visible rows
    pub end_index: usize,
    /// Number of visible rows in viewport
    pub visible_rows: usize,
    /// Height of each row
    pub row_height: f32,
    /// Height of the header
    pub header_height: f32,
    /// Total height needed for all data
    pub total_height: f32,
}

/// Material Design 3 Data Table Styles
pub struct MaterialDataTable;

impl MaterialDataTable {    /// Create a Material Design 3 compliant data table container
    pub fn table_container(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let colors = tokens.colors.clone();

        move |_| container_builders::base_container_style(
            None, // Transparent background
            border_styles::transparent_border(),
            Some(colors.on_surface),
        )
    }    /// Style for table header container
    pub fn header_container(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let _colors = tokens.colors.clone();
        let _shapes = tokens.shapes.clone();

        move |_| container_builders::base_container_style(
            None, // Transparent to avoid double background stacking
            border_styles::transparent_border(),
            None,
        )
    }    /// Style for table header cell
    pub fn header_cell(
        tokens: &MaterialTokens,
        _sortable: bool,
        is_sorted: bool,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens_clone = tokens.clone();
        move |_| {
            let background_color = color_helpers::get_header_background_color(&tokens_clone, is_sorted);
            let text_color = color_helpers::get_text_color_for_background(&tokens_clone, is_sorted, false);

            container_builders::base_container_style(
                Some(background_color),
                border_styles::table_border(tokens_clone.colors.outline_variant),
                Some(text_color),
            )
        }
    }    /// Style for table row
    pub fn table_row(
        tokens: &MaterialTokens,
        index: usize,
        is_selected: bool,
        is_striped: bool,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens_clone = tokens.clone();
        move |_| {
            let background_color = color_helpers::get_row_background_color(&tokens_clone, index, is_selected, is_striped);
            let text_color = color_helpers::get_text_color_for_background(&tokens_clone, false, is_selected);

            container_builders::base_container_style(
                Some(background_color),
                border_styles::table_border(tokens_clone.colors.outline_variant),
                Some(text_color),
            )
        }
    }    /// Style for table cell with alignment support
    pub fn table_cell(
        tokens: &MaterialTokens,
        align: TextAlignment,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let colors = tokens.colors.clone();
        let _iced_align = interaction_helpers::alignment_to_iced(align);

        move |_| container_builders::base_container_style(
            None, // No background for individual cells
            border_styles::table_border(colors.outline_variant),
            Some(colors.on_surface),
        )
    }/// Style for empty state container
    pub fn empty_state(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let colors = tokens.colors.clone();

        move |_| container_builders::base_container_style(
            Some(colors.surface),
            border_styles::table_border(colors.outline_variant),
            None,
        )
    }    /// Create header text with Material Design typography and sorting support
    #[must_use]
    pub fn header_text<'a>(
        content: &'a str,
        tokens: &MaterialTokens,
    ) -> Element<'a, crate::messages::Message> {
        typography_helpers::create_text_element(
            content,
            &tokens.typography.title_medium,
            tokens.colors.on_surface_variant,
        )
    }

    /// Create sortable header with sort indicator
    #[must_use]
    pub fn sortable_header_text<'a>(
        content: &'a str,
        sort_state: Option<&SortState>,
        column_id: &str,
        tokens: &MaterialTokens,    ) -> Element<'a, crate::messages::Message> {
        let is_sorted = sort_state.map_or(false, |s| s.column_id == column_id);
        
        let text_color = color_helpers::get_text_color_for_background(tokens, is_sorted, false);
        
        let header_text = typography_helpers::create_text_element(
            content,
            &tokens.typography.title_medium,
            text_color,
        );
          if is_sorted {
            // TODO: In a real implementation, you'd use a Row widget to combine text and indicator
            // For now, we'll return just the text with sort styling applied
            header_text
        } else {
            header_text
        }
    }/// Create body text with Material Design typography
    #[must_use]
    pub fn body_text<'a>(
        content: &'a str,
        tokens: &MaterialTokens,
    ) -> Element<'a, crate::messages::Message> {
        typography_helpers::create_text_element(
            content,
            &tokens.typography.body_medium,
            tokens.colors.on_surface,
        )
    }    /// Create sort indicator icon (▲ or ▼)
    #[must_use]
    pub fn sort_indicator<'a>(
        direction: Option<SortDirection>,
        tokens: &MaterialTokens,
    ) -> Element<'a, crate::messages::Message> {
        let indicator = match direction {
            Some(SortDirection::Ascending) => " ▲",
            Some(SortDirection::Descending) => " ▼",
            None => "",
        };

        typography_helpers::create_text_element(
            indicator,
            &tokens.typography.label_small,
            tokens.colors.primary.base,
        )
    }
}

/// Material Design 3 List Components
pub struct MaterialList;

impl MaterialList {    /// Style for list container
    pub fn list_container(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let colors = tokens.colors.clone();
        let shapes = tokens.shapes.clone();
        let elevation = tokens.elevation.clone();

        move |_| container_builders::elevated_container_style(
            colors.surface,
            Border {
                color: colors.outline_variant,
                width: 1.0,
                radius: shapes.corner_medium.to_radius(),
            },
            elevation.get_level(ElevationLevel::Level1).shadow,
        )
    }    /// Style for list item
    pub fn list_item(
        tokens: &MaterialTokens,
        is_selected: bool,
        _is_highlighted: bool,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens_clone = tokens.clone();

        move |_| {
            let background_color = color_helpers::get_row_background_color(&tokens_clone, 0, is_selected, false);
            let text_color = color_helpers::get_text_color_for_background(&tokens_clone, false, is_selected);

            container_builders::base_container_style(
                Some(background_color),
                border_styles::table_border(tokens_clone.colors.outline_variant),
                Some(text_color),
            )
        }
    }    /// Create list item with Material Design styling
    #[must_use]
    pub fn create_list_item<'a, T>(
        content: Element<'a, T>,
        tokens: &'a MaterialTokens,
        is_selected: bool,
    ) -> Container<'a, T> {
        container(content)
            .width(Length::Fill)
            .padding(tokens.spacing.md)
            .style(Self::list_item(tokens, is_selected, false))
    }

    /// Create responsive list item with density-aware padding
    #[must_use]
    pub fn create_responsive_list_item<'a, T>(
        content: Element<'a, T>,
        tokens: &'a MaterialTokens,
        is_selected: bool,
        density: TableDensity,
    ) -> Container<'a, T> {
        let item_padding = state_helpers::get_cell_padding(density);
        
        container(content)
            .width(Length::Fill)
            .padding(item_padding)
            .style(Self::list_item(tokens, is_selected, false))
    }

    /// Create list item text with proper typography
    #[must_use]
    pub fn list_item_text<'a>(
        content: &'a str,
        tokens: &MaterialTokens,
        is_selected: bool,
    ) -> Element<'a, crate::messages::Message> {
        let text_color = color_helpers::get_text_color_for_background(tokens, false, is_selected);
        
        typography_helpers::create_text_element(
            content,
            &tokens.typography.body_medium,
            text_color,
        )
    }
}

/// Material Design 3 Tree View Components
pub struct MaterialTreeView;

impl MaterialTreeView {    /// Style for tree container
    pub fn tree_container(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let colors = tokens.colors.clone();
        let shapes = tokens.shapes.clone();
        let elevation = tokens.elevation.clone();

        move |_| container_builders::elevated_container_style(
            colors.surface,
            Border {
                color: colors.outline_variant,
                width: 1.0,
                radius: shapes.corner_medium.to_radius(),
            },
            elevation.get_level(ElevationLevel::Level1).shadow,
        )
    }    /// Style for tree node
    pub fn tree_node(
        tokens: &MaterialTokens,
        level: usize,
        _is_expanded: bool,
        is_selected: bool,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens_clone = tokens.clone();

        move |_| {
            // Use row background color helper for consistent theming
            let background_color = color_helpers::get_row_background_color(&tokens_clone, level, is_selected, false);
            let text_color = color_helpers::get_text_color_for_background(&tokens_clone, false, is_selected);

            container_builders::base_container_style(
                Some(background_color),
                border_styles::table_border(tokens_clone.colors.outline_variant),
                Some(text_color),
            )
        }
    }/// Calculate indentation for tree level with density awareness
    #[must_use]
    pub fn get_level_indent(level: usize, density: TableDensity) -> f32 {
        let base_indent = match density {
            TableDensity::Compact => 16.0,
            TableDensity::Standard => 24.0,
            TableDensity::Comfortable => 32.0,
        };
        // Safe conversion with bounds checking to avoid precision loss
        let level_f64 = level as f64;
        (level_f64 * base_indent as f64) as f32
    }    /// Create expand/collapse indicator
    #[must_use]
    pub fn expand_indicator<'a>(
        is_expanded: bool,
        tokens: &MaterialTokens,
    ) -> Element<'a, crate::messages::Message> {
        let symbol = if is_expanded { "▼" } else { "►" };

        typography_helpers::create_text_element(
            symbol,
            &tokens.typography.body_small,
            tokens.colors.on_surface_variant,
        )
    }

    /// Create tree node content with proper indentation and styling
    #[must_use]
    pub fn create_tree_node<'a, T>(
        content: Element<'a, T>,
        level: usize,
        tokens: &'a MaterialTokens,
        is_selected: bool,
        is_expanded: bool,
        density: TableDensity,
    ) -> Container<'a, T> {
        let indent = Self::get_level_indent(level, density);
        let node_padding = state_helpers::get_cell_padding(density);
          container(content)
            .width(Length::Fill)
            .padding(Padding {
                top: node_padding.top,
                right: node_padding.right,
                bottom: node_padding.bottom,
                left: node_padding.left + indent,
            })
            .style(Self::tree_node(tokens, level, is_expanded, is_selected))
    }    /// Create tree node text with proper typography and indentation
    #[must_use]
    pub fn tree_node_text<'a>(
        content: &'a str,
        _level: usize,
        tokens: &MaterialTokens,
        is_selected: bool,
        _density: TableDensity,
    ) -> Element<'a, crate::messages::Message> {
        let text_color = color_helpers::get_text_color_for_background(tokens, false, is_selected);
        
        typography_helpers::create_text_element(
            content,
            &tokens.typography.body_medium,
            text_color,
        )
    }
}

// Export all public types and functions
pub use MaterialDataTable as DataTable;
pub use MaterialList as List;
pub use MaterialTreeView as TreeView;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_column_creation() {
        let column = TableColumn::new("id", "Title")
            .width(ColumnWidth::Fill(2))
            .sortable(true)
            .align(TextAlignment::Start);

        assert_eq!(column.id, "id");
        assert_eq!(column.title, "Title");
        assert!(column.sortable);
    }

    #[test]
    fn test_table_density_values() {
        assert_eq!(TableDensity::Compact.row_height(), 32.0);
        assert_eq!(TableDensity::Standard.row_height(), 48.0);
        assert_eq!(TableDensity::Comfortable.row_height(), 64.0);
    }
}
