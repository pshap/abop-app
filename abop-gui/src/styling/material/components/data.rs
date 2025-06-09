//! Material Design 3 Data Components
//!
//! This module implements Material Design 3 data display components including:
//! - Data tables with sorting and filtering
//! - Lists with selection support
//! - Tree views for hierarchical data
//!
//! These components are built on top of Iced and follow Material Design 3 guidelines.

use crate::styling::material::MaterialTokens;
use iced::widget::container;
use iced::{Element, Length, Padding};

/// Re-export commonly used types for convenience
pub use iced::alignment::Horizontal;

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

/// Sort direction for table columns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    /// Sort in ascending order (A-Z, 0-9)
    Ascending,
    /// Sort in descending order (Z-A, 9-0)
    Descending,
}

/// Table density options following Material Design specifications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableDensity {
    /// Compact density with minimal spacing (32px row height)
    Compact,
    /// Standard density with balanced spacing (48px row height)
    Standard,
    /// Comfortable density with generous spacing (56px row height)
    Comfortable,
}

impl TableDensity {
    /// Get row height for the density level
    pub fn row_height(&self) -> f32 {
        match self {
            Self::Compact => 32.0,
            Self::Standard => 48.0,
            Self::Comfortable => 56.0,
        }
    }
    
    /// Get header height for the density level
    pub fn header_height(&self) -> f32 {
        match self {
            Self::Compact => 40.0,
            Self::Standard => 56.0,
            Self::Comfortable => 64.0,
        }
    }
    
    /// Get cell padding for the density level
    pub fn cell_padding(&self) -> Padding {
        match self {
            Self::Compact => Padding::new(8.0),
            Self::Standard => Padding::new(12.0),
            Self::Comfortable => Padding::new(16.0),
        }
    }
}

/// Text alignment options for table cells
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    /// Align text to the start (left in LTR languages)
    Start,
    /// Center align text
    Center,
    /// Align text to the end (right in LTR languages)
    End,
}

/// Column data type for proper rendering and sorting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnDataType {
    /// Text data type for string content
    Text,
    /// Numeric data type for numbers
    Number,
    /// Date data type for date/time values
    Date,
    /// Boolean data type for true/false values
    Boolean,
    /// Custom data type for specialized content
    Custom,
}

/// Column definition for data tables
#[derive(Debug, Clone)]
pub struct TableColumn {
    /// Unique identifier for the column
    pub id: String,
    /// Display title shown in the header
    pub title: String,
    /// Width specification for the column
    pub width: ColumnWidth,
    /// Whether the column can be sorted
    pub sortable: bool,
    /// Text alignment within the column
    pub alignment: TextAlignment,
    /// Data type for proper rendering and sorting
    pub data_type: ColumnDataType,
    /// Whether the column should stick to the left side
    pub sticky: bool,
}

impl TableColumn {
    /// Create a new table column
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            width: ColumnWidth::Auto,
            sortable: false,
            alignment: TextAlignment::Start,
            data_type: ColumnDataType::Text,
            sticky: false,
        }
    }
    
    /// Set column width
    pub fn width(mut self, width: ColumnWidth) -> Self {
        self.width = width;
        self
    }
    
    /// Set column sortable
    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }
    
    /// Set column alignment
    pub fn align(mut self, align: TextAlignment) -> Self {
        self.alignment = align;
        self
    }
    
    /// Set column data type
    pub fn data_type(mut self, data_type: ColumnDataType) -> Self {
        self.data_type = data_type;
        self
    }
    
    /// Make column sticky
    pub fn sticky(mut self) -> Self {
        self.sticky = true;
        self
    }
}

/// Column width specification
#[derive(Debug, Clone)]
pub enum ColumnWidth {
    /// Fixed width in pixels
    Fixed(f32),
    /// Fill portion of available space
    FillPortion(u16),
    /// Alias for FillPortion for backward compatibility
    Fill(u16), // Alias for FillPortion for backward compatibility
    /// Automatic width based on content
    Auto,
    /// Ratio of width (numerator, denominator)
    Ratio(u32, u32),
    /// Shrink to minimum required size
    Shrink,
    /// Alias for Shrink for backward compatibility
    FitContent, // Alias for Shrink for backward compatibility
}

impl From<f32> for ColumnWidth {
    fn from(value: f32) -> Self {
        Self::Fixed(value)
    }
}

impl From<u32> for ColumnWidth {
    fn from(value: u32) -> Self {
        Self::Fixed(value as f32)
    }
}

/// Table sort state
#[derive(Debug, Clone)]
pub struct SortState {
    /// ID of the column being sorted
    pub column_id: String,
    /// Direction of the sort
    pub direction: SortDirection,
}

/// Material Design 3 Data Table Configuration
#[derive(Debug, Clone)]
pub struct DataTableConfig {
    // Table behavior
    /// Whether rows can be selected
    pub selectable: bool,
    /// Whether rows show hover effects
    pub hoverable: bool,
    /// Whether the header sticks to the top
    pub sticky_header: bool,
    /// Whether alternating rows have different backgrounds
    pub striped: bool,
    /// Whether to use virtual scrolling for large datasets
    pub virtual_scrolling: bool,
    /// Maximum number of visible rows before scrolling
    pub max_visible_rows: Option<usize>,
    /// Whether rows have action buttons
    pub row_actions: bool,
    /// Whether columns can be resized by the user
    pub resizable_columns: bool,
    /// Minimum width for columns in pixels
    pub min_column_width: f32,
    
    // Table styling
    /// Visual density of the table
    pub density: TableDensity,
    /// Whether to use striped row styling
    pub is_striped: bool,
    /// Whether rows react to hover
    pub is_hoverable: bool,
    /// Whether to show the table header
    pub show_header: bool,
    /// Whether to show the table footer
    pub show_footer: bool,
    /// Whether rows are selectable
    pub is_selectable: bool,
    /// Whether columns are sortable
    pub is_sortable: bool,
    /// Whether columns are resizable
    pub is_resizable: bool,
    /// Whether to use virtual scrolling
    pub is_virtual_scroll: bool,
    /// Minimum height for rows in pixels
    pub min_row_height: f32,
    /// Maximum height for rows in pixels
    pub max_row_height: Option<f32>,
    /// Fixed height for the header in pixels
    pub header_height: Option<f32>,
    /// Fixed height for the footer in pixels
    pub footer_height: Option<f32>,
    /// Border color for the table
    pub border_color: Option<iced::Color>,
    /// Border width in pixels
    pub border_width: f32,
    /// Border radius in pixels
    pub border_radius: f32,
}

impl Default for DataTableConfig {
    fn default() -> Self {
        Self {
            // Table behavior
            selectable: true,
            hoverable: true,
            sticky_header: true,
            striped: false,
            virtual_scrolling: true,
            max_visible_rows: None,
            row_actions: false,
            resizable_columns: true,
            min_column_width: 80.0,
            
            // Table styling
            density: TableDensity::Standard,
            is_striped: true,
            is_hoverable: true,
            show_header: true,
            show_footer: false,
            is_selectable: true,
            is_sortable: true,
            is_resizable: true,
            is_virtual_scroll: true,
            min_row_height: 32.0,
            max_row_height: None,
            header_height: None,
            footer_height: None,
            border_color: None,
            border_width: 1.0,
            border_radius: 4.0,
        }
    }
}



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
    pub fn get_header_background_color(
        tokens: &MaterialTokens,
        is_sorted: bool,
    ) -> Color {
        if is_sorted {
            tokens.colors.primary_container        } else {
            tokens.colors.surface_variant
        }
    }
}

mod typography_helpers {
    use iced::widget::text::LineHeight;
    use iced::{Color, Pixels};
    use crate::styling::material::typography::TypeStyle;
    use iced::widget::text;
    use iced::Element;

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

/// Performance optimization helpers
mod performance_helpers {
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
mod interaction_helpers {
    use super::*;
    use iced::alignment::Horizontal;

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



/// Material Design 3 Data Table
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
    pub fn table_cell(tokens: &MaterialTokens, _align: TextAlignment) -> impl Fn(&iced::Theme) -> container::Style {
        let _tokens = tokens.clone();
        move |_theme| container::Style {
            background: None,
            border: transparent_border(),
            ..Default::default()
        }
    }
      /// Create header text
    pub fn header_text<'a>(content: &'a str, tokens: &MaterialTokens) -> Element<'a, crate::messages::Message> {
        typography_helpers::create_text_element(
            content,
            &tokens.typography.body_medium,
            tokens.colors.on_surface,
        )
    }
    
    /// Create body text
    pub fn body_text<'a>(content: &'a str, tokens: &MaterialTokens) -> Element<'a, crate::messages::Message> {
        typography_helpers::create_text_element(
            content,
            &tokens.typography.body_small,
            tokens.colors.on_surface,
        )
    }    /// Style for header cell (backward compatibility)
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
    }    /// Style for table row (backward compatibility)
    pub fn table_row(
        tokens: &MaterialTokens,
        index: usize,
        is_selected: bool,
        is_striped: bool,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens = tokens.clone();
        move |_theme| {
            let background = color_helpers::get_row_background_color(&tokens, index, is_selected, is_striped);
            container::Style {
                background: Some(iced::Background::Color(background)),
                border: transparent_border(),
                ..Default::default()
            }
        }
    }
}

/// Material Design 3 List Component
pub struct MaterialList;

impl MaterialList {
    /// Create a Material List widget
    pub fn new() -> Self {
        Self
    }
    
    /// Style for list container
    pub fn list_container(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens = tokens.clone();
        move |_theme| container::Style {
            background: Some(iced::Background::Color(tokens.colors.surface)),
            border: transparent_border(),
            ..Default::default()
        }
    }
    
    /// Style for list item
    pub fn list_item(
        tokens: &MaterialTokens,
        is_selected: bool,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens = tokens.clone();
        move |_theme| {
            let background = if is_selected {
                tokens.colors.surface_container_high
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
}

/// Material Design 3 Tree View Component
pub struct MaterialTreeView;

impl MaterialTreeView {
    /// Create a Material Tree View widget
    pub fn new() -> Self {
        Self
    }
    
    /// Style for tree container
    pub fn tree_container(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens = tokens.clone();
        move |_theme| container::Style {
            background: Some(iced::Background::Color(tokens.colors.surface)),
            border: transparent_border(),
            ..Default::default()
        }
    }
    
    /// Style for tree node
    pub fn tree_node(
        tokens: &MaterialTokens,
        is_selected: bool,
        _level: usize,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens = tokens.clone();
        move |_theme| {
            let background = if is_selected {
                tokens.colors.surface_container_high
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
}

/// Helper function to convert column width to iced Length
pub fn column_width_to_length(width: &ColumnWidth) -> Length {
    match width {
        ColumnWidth::Fixed(w) => Length::Fixed(*w),
        ColumnWidth::FillPortion(p) => Length::FillPortion(*p),
        ColumnWidth::Fill(p) => Length::FillPortion(*p), // Alias for FillPortion
        ColumnWidth::Auto => Length::Shrink,
        ColumnWidth::Ratio(num, _den) => Length::FillPortion(*num as u16),
        ColumnWidth::Shrink => Length::Shrink,
        ColumnWidth::FitContent => Length::Shrink, // Alias for Shrink
    }
}

/// Helper function to convert text alignment to iced horizontal alignment
pub fn text_alignment_to_horizontal(align: TextAlignment) -> Horizontal {
    match align {
        TextAlignment::Start => Horizontal::Left,
        TextAlignment::Center => Horizontal::Center,
        TextAlignment::End => Horizontal::Right,
    }
}

/// Public API helpers for advanced data table functionality
pub mod data_table_helpers {
    use super::*;
    use iced::widget::Container;

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
        let mut widths = Vec::new();
        let mut remaining_width = available_width;
        let mut fill_portions = 0u16;        // First pass: handle fixed widths
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
                ColumnWidth::Fill(p) => {
                    fill_portions += p;
                    widths.push((column.id.clone(), Length::FillPortion(*p)));
                }
                ColumnWidth::Auto => {
                    // Auto columns get minimum viable width for now
                    let auto_width = 120.0;
                    widths.push((column.id.clone(), Length::Fixed(auto_width)));
                    remaining_width -= auto_width;
                }
                ColumnWidth::Shrink | ColumnWidth::FitContent => {
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
        columns.iter().enumerate().map(|(col_index, column)| {
            if let Some(data) = row_data.get(col_index) {
                let cell_content = format_cell(data, column, row_index);
                data_table_helpers::create_responsive_cell(
                    cell_content,
                    column,
                    tokens,
                    config.density,
                ).into()
            } else {
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
    pub fn create_numeric_column(
        id: &str,
        title: &str,
        width: ColumnWidth,
    ) -> TableColumn {
        TableColumn::new(id, title)
            .width(width)
            .sortable(true)
            .align(TextAlignment::End)
            .data_type(ColumnDataType::Number)
    }

    /// Create a date column with center alignment
    pub fn create_date_column(
        id: &str,
        title: &str,
        width: ColumnWidth,
    ) -> TableColumn {
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
    fn test_table_density_values() {
        assert_eq!(TableDensity::Compact.row_height(), 32.0);
        assert_eq!(TableDensity::Standard.row_height(), 48.0);
        assert_eq!(TableDensity::Comfortable.row_height(), 56.0);
    }

    #[test]
    fn test_column_creation() {
        let column = TableColumn::new("id", "Title")
            .width(ColumnWidth::Fixed(100.0))
            .sortable(true)
            .align(TextAlignment::Center);

        assert_eq!(column.id, "id");
        assert_eq!(column.title, "Title");
        assert!(column.sortable);
        assert_eq!(column.alignment, TextAlignment::Center);
    }

    #[test]
    fn test_column_width_conversion() {
        assert_eq!(column_width_to_length(&ColumnWidth::Fixed(100.0)), Length::Fixed(100.0));
        assert_eq!(column_width_to_length(&ColumnWidth::FillPortion(2)), Length::FillPortion(2));
        assert_eq!(column_width_to_length(&ColumnWidth::Auto), Length::Shrink);
    }
}
