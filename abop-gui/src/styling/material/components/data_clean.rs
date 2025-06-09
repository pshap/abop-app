//! Material Design 3 Data Components
//!
//! This module implements Material Design 3 data display components including:
//! - Data tables with sorting and filtering
//! - Lists with selection support
//! - Tree views for hierarchical data
//!
//! These components are built on top of Iced and follow Material Design 3 guidelines.

use crate::styling::material::MaterialTokens;
use iced::widget::{Container, container, text, Row};
use iced::{Border, Element, Length, Padding, Alignment};

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
    Ascending,
    Descending,
}

/// Table density options following Material Design specifications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableDensity {
    Compact,
    Standard,
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
    Start,
    Center,
    End,
}

/// Column data type for proper rendering and sorting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnDataType {
    Text,
    Number,
    Date,
    Boolean,
    Custom,
}

/// Column definition for data tables
#[derive(Debug, Clone)]
pub struct TableColumn {
    pub id: String,
    pub title: String,
    pub width: ColumnWidth,
    pub sortable: bool,
    pub alignment: TextAlignment,
    pub data_type: ColumnDataType,
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
    Fixed(f32),
    FillPortion(u16),
    Auto,
    Ratio(u32, u32),
    Shrink,
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
    pub column_id: String,
    pub direction: SortDirection,
}

/// Material Design 3 Data Table Configuration
#[derive(Debug, Clone)]
pub struct DataTableConfig {
    // Table behavior
    pub selectable: bool,
    pub hoverable: bool,
    pub sticky_header: bool,
    pub striped: bool,
    pub virtual_scrolling: bool,
    pub max_visible_rows: Option<usize>,
    pub row_actions: bool,
    pub resizable_columns: bool,
    pub min_column_width: f32,
    
    // Table styling
    pub density: TableDensity,
    pub is_striped: bool,
    pub is_hoverable: bool,
    pub show_header: bool,
    pub show_footer: bool,
    pub is_selectable: bool,
    pub is_sortable: bool,
    pub is_resizable: bool,
    pub is_virtual_scroll: bool,
    pub min_row_height: f32,
    pub max_row_height: Option<f32>,
    pub header_height: Option<f32>,
    pub footer_height: Option<f32>,
    pub border_color: Option<iced::Color>,
    pub border_width: f32,
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

/// Helper modules for creating Material Design components
mod border_styles {
    use iced::Border;

    /// Create standard table border
    pub fn table_border(color: iced::Color) -> Border {
        Border {
            color,
            width: 0.5,
            radius: 0.0.into(),
        }
    }

    /// Create transparent border
    pub fn transparent_border() -> Border {
        Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
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
            tokens.colors.primary_container
        } else {
            tokens.colors.surface_variant
        }
    }

    /// Get text color for background
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

mod typography_helpers {
    use super::*;
    use iced::widget::text::LineHeight;
    use iced::{Color, Pixels};
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
        let tokens = tokens.clone();
        move |_theme| container::Style {
            background: None,
            border: transparent_border(),
            ..Default::default()
        }
    }
    
    /// Create header text
    pub fn header_text(content: &str, tokens: &MaterialTokens) -> Element<crate::messages::Message> {
        typography_helpers::create_text_element(
            content,
            &tokens.typography.body_medium,
            tokens.colors.on_surface,
        )
    }
    
    /// Create body text
    pub fn body_text(content: &str, tokens: &MaterialTokens) -> Element<crate::messages::Message> {
        typography_helpers::create_text_element(
            content,
            &tokens.typography.body_small,
            tokens.colors.on_surface,
        )
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
        ColumnWidth::Auto => Length::Shrink,
        ColumnWidth::Ratio(num, _den) => Length::FillPortion(*num as u16),
        ColumnWidth::Shrink => Length::Shrink,
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
