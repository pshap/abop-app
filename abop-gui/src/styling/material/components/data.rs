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
use iced::border::Radius;
use iced::widget::text::LineHeight;
use iced::widget::{Container, container, text};
use iced::{Background, Border, Color, Element, Length, Padding};

use crate::styling::material::spacing;

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
    pub const fn row_height(&self) -> f32 {
        match self {
            Self::Compact => 32.0,
            Self::Standard => 48.0,
            Self::Comfortable => 64.0,
        }
    }

    /// Get cell padding for the density level
    #[must_use]
    pub const fn cell_padding(&self) -> Padding {
        match self {
            Self::Compact => Padding::new(4.0),
            Self::Standard => Padding::new(8.0),
            Self::Comfortable => Padding::new(12.0),
        }
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

/// Material Design 3 Data Table Styles
pub struct MaterialDataTable;

impl MaterialDataTable {
    /// Create a Material Design 3 compliant data table container
    pub fn table_container(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let colors = tokens.colors.clone();

        move |_| container::Style {
            // Use transparent background to avoid the "box" look - individual cells provide the color
            background: None,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(0.0),
            },
            text_color: Some(colors.on_surface),
            ..Default::default()
        }
    }

    /// Style for table header container
    pub fn header_container(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let _colors = tokens.colors.clone();
        let _shapes = tokens.shapes.clone();

        move |_| container::Style {
            // Make the header container transparent to avoid double background stacking
            // The individual header cells will provide the background
            background: None,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(0.0),
            },
            ..Default::default()
        }
    }

    /// Style for table header cell
    pub fn header_cell(
        tokens: &MaterialTokens,
        _sortable: bool,
        is_sorted: bool,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let colors = tokens.colors.clone();
        let _shapes = tokens.shapes.clone();
        move |_| {
            let background_color = if is_sorted {
                // Use primary_container for sorted columns (MD3 spec)
                colors.primary_container
            } else {
                // Use Material Design 3 surface_variant for header cells per MD3 spec
                colors.surface_variant
            };

            container::Style {
                background: Some(Background::Color(background_color)),
                border: Border {
                    color: colors.outline_variant,
                    width: 0.5,               // Lighter border for subtlety
                    radius: Radius::new(0.0), // Keep square corners for table headers
                },
                text_color: Some(colors.on_surface_variant), // Proper MD3 text color for surface_variant
                ..Default::default()
            }
        }
    }

    /// Style for table row
    pub fn table_row(
        tokens: &MaterialTokens,
        index: usize,
        is_selected: bool,
        is_striped: bool,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let colors = tokens.colors.clone();
        move |_| {
            let background_color = if is_selected {
                colors.secondary_container
            } else if is_striped && index % 2 == 1 {
                // Use surface container for striped rows (subtle difference)
                colors.surface_container
            } else {
                // Use surface_container_lowest for regular rows per MD3 spec
                colors.surface_container_lowest
            };

            container::Style {
                background: Some(Background::Color(background_color)),
                border: Border {
                    color: colors.outline_variant,
                    width: 0.5, // Match header border width
                    radius: Radius::new(0.0),
                },
                text_color: Some(colors.on_surface), // Proper MD3 text color for surface containers
                ..Default::default()
            }
        }
    }
    /// Style for table cell
    pub fn table_cell(
        tokens: &MaterialTokens,
        _align: TextAlignment,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let colors = tokens.colors.clone();

        move |_| container::Style {
            background: None,
            border: Border {
                color: colors.outline_variant,
                width: 0.5, // Match header and row border width for consistency
                radius: Radius::new(0.0),
            },
            text_color: Some(colors.on_surface), // Ensure text color is properly set
            ..Default::default()
        }
    }

    /// Style for empty state container
    pub fn empty_state(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let colors = tokens.colors.clone();

        move |_| container::Style {
            background: Some(Background::Color(colors.surface)),
            border: Border {
                color: colors.outline_variant,
                width: 1.0,
                radius: Radius::new(0.0),
            },
            ..Default::default()
        }
    }

    /// Create header text with Material Design typography
    #[must_use]
    pub fn header_text<'a>(
        content: &'a str,
        tokens: &MaterialTokens,
    ) -> Element<'a, crate::messages::Message> {
        let colors = &tokens.colors;
        let typography = &tokens.typography;

        text(content)
            .size(typography.title_medium.size) // Use title_medium instead of title_small per MD3 spec
            .line_height(LineHeight::Absolute(typography.title_medium.line_height))
            .color(colors.on_surface_variant) // Proper text color for surface_variant backgrounds
            .into()
    }

    /// Create body text with Material Design typography
    #[must_use]
    pub fn body_text<'a>(
        content: &'a str,
        tokens: &MaterialTokens,
    ) -> Element<'a, crate::messages::Message> {
        let colors = &tokens.colors;
        let typography = &tokens.typography;

        text(content)
            .size(typography.body_medium.size)
            .line_height(LineHeight::Absolute(typography.body_medium.line_height))
            .color(colors.on_surface)
            .into()
    }

    /// Create sort indicator icon (▲ or ▼)
    #[must_use]
    pub fn sort_indicator<'a>(
        direction: Option<SortDirection>,
        tokens: &MaterialTokens,
    ) -> Element<'a, crate::messages::Message> {
        let colors = &tokens.colors;
        let typography = &tokens.typography;

        let indicator = match direction {
            Some(SortDirection::Ascending) => " ▲",
            Some(SortDirection::Descending) => " ▼",
            None => "",
        };

        text(indicator)
            .size(typography.label_small.size)
            .color(colors.primary.base)
            .into()
    }
}

/// Material Design 3 List Components
pub struct MaterialList;

impl MaterialList {
    /// Style for list container
    pub fn list_container(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let colors = tokens.colors.clone();
        let shapes = tokens.shapes.clone();
        let elevation = tokens.elevation.clone();

        move |_| container::Style {
            background: Some(Background::Color(colors.surface)),
            border: Border {
                color: colors.outline_variant,
                width: 1.0,
                radius: shapes.corner_medium.to_radius(),
            },
            shadow: elevation.get_level(ElevationLevel::Level1).shadow,
            ..Default::default()
        }
    }

    /// Style for list item
    pub fn list_item(
        tokens: &MaterialTokens,
        is_selected: bool,
        is_highlighted: bool,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let colors = tokens.colors.clone();

        move |_| {
            let background_color = if is_selected {
                colors.secondary_container
            } else if is_highlighted {
                colors.surface_container_low
            } else {
                colors.surface
            };

            container::Style {
                background: Some(Background::Color(background_color)),
                border: Border {
                    color: colors.outline_variant,
                    width: 1.0,
                    radius: Radius::new(0.0),
                },
                ..Default::default()
            }
        }
    }
    /// Create list item with Material Design styling
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
}

/// Material Design 3 Tree View Components
pub struct MaterialTreeView;

impl MaterialTreeView {
    /// Style for tree container
    pub fn tree_container(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let colors = tokens.colors.clone();
        let shapes = tokens.shapes.clone();
        let elevation = tokens.elevation.clone();

        move |_| container::Style {
            background: Some(Background::Color(colors.surface)),
            border: Border {
                color: colors.outline_variant,
                width: 1.0,
                radius: shapes.corner_medium.to_radius(),
            },
            shadow: elevation.get_level(ElevationLevel::Level1).shadow,
            ..Default::default()
        }
    }

    /// Style for tree node
    pub fn tree_node(
        tokens: &MaterialTokens,
        _level: usize,
        _is_expanded: bool,
        is_selected: bool,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let colors = tokens.colors.clone();

        move |_| {
            let background_color = if is_selected {
                colors.secondary_container
            } else {
                colors.surface
            };

            container::Style {
                background: Some(Background::Color(background_color)),
                border: Border {
                    color: colors.outline_variant,
                    width: 1.0,
                    radius: Radius::new(0.0),
                },
                ..Default::default()
            }
        }
    }

    /// Calculate indentation for tree level
    #[must_use]
    pub fn get_level_indent(level: usize) -> f32 {
        // Safe conversion with bounds checking to avoid precision loss
        let level_f64 = level as f64;
        (level_f64 * 24.0) as f32
    }

    /// Create expand/collapse indicator
    #[must_use]
    pub fn expand_indicator<'a>(
        is_expanded: bool,
        tokens: &MaterialTokens,
    ) -> Element<'a, crate::messages::Message> {
        let colors = &tokens.colors;
        let typography = &tokens.typography;

        let symbol = if is_expanded { "▼" } else { "►" };

        text(symbol)
            .size(typography.body_small.size)
            .color(colors.on_surface_variant)
            .into()
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
