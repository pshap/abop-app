//! Core enums and basic types for Material Design 3 data components

use iced::Padding;
use super::super::constants::density;

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
            Self::Compact => density::COMPACT_ROW_HEIGHT,
            Self::Standard => density::STANDARD_ROW_HEIGHT,
            Self::Comfortable => density::COMFORTABLE_ROW_HEIGHT,
        }
    }

    /// Get header height for the density level
    pub fn header_height(&self) -> f32 {
        match self {
            Self::Compact => density::COMPACT_HEADER_HEIGHT,
            Self::Standard => density::STANDARD_HEADER_HEIGHT,
            Self::Comfortable => density::COMFORTABLE_HEADER_HEIGHT,
        }
    }

    /// Get cell padding for the density level
    pub fn cell_padding(&self) -> Padding {
        match self {
            Self::Compact => Padding::new(density::COMPACT_CELL_PADDING),
            Self::Standard => Padding::new(density::STANDARD_CELL_PADDING),
            Self::Comfortable => Padding::new(density::COMFORTABLE_CELL_PADDING),
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

/// Column width specification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColumnWidth {
    /// Fixed width in pixels
    Fixed(f32),
    /// Fill portion of available space
    FillPortion(u16),
    /// Automatic width based on content
    Auto,
    /// Ratio of width (numerator, denominator)
    Ratio(u32, u32),
    /// Shrink to minimum required size
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
    /// ID of the column being sorted
    pub column_id: String,
    /// Direction of the sort
    pub direction: SortDirection,
}
