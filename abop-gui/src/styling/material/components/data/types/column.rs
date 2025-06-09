//! Column definition and builder methods

use super::core::{ColumnDataType, ColumnWidth, TextAlignment};

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

    /// Create a text column (most common case)
    pub fn text(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self::new(id, title).data_type(ColumnDataType::Text)
    }

    /// Create a numeric column
    pub fn number(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self::new(id, title)
            .data_type(ColumnDataType::Number)
            .align(TextAlignment::End)
    }

    /// Create a date column
    pub fn date(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self::new(id, title).data_type(ColumnDataType::Date)
    }

    /// Create a boolean column
    pub fn boolean(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self::new(id, title)
            .data_type(ColumnDataType::Boolean)
            .align(TextAlignment::Center)
    }

    /// Create a custom column
    pub fn custom(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self::new(id, title).data_type(ColumnDataType::Custom)
    }

    /// Set column width
    pub fn width(mut self, width: ColumnWidth) -> Self {
        self.width = width;
        self
    }

    /// Set fixed width in pixels
    pub fn fixed_width(mut self, width: f32) -> Self {
        self.width = ColumnWidth::Fixed(width);
        self
    }

    /// Set fill portion (relative width)
    pub fn fill(mut self, portion: u16) -> Self {
        self.width = ColumnWidth::FillPortion(portion);
        self
    }

    /// Set auto width based on content
    pub fn auto_width(mut self) -> Self {
        self.width = ColumnWidth::Auto;
        self
    }

    /// Set width ratio
    pub fn ratio(mut self, numerator: u32, denominator: u32) -> Self {
        self.width = ColumnWidth::Ratio(numerator, denominator);
        self
    }

    /// Set shrink to minimum required size
    pub fn shrink(mut self) -> Self {
        self.width = ColumnWidth::Shrink;
        self
    }

    /// Set column sortable
    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    /// Enable sorting for this column
    pub fn with_sorting(mut self) -> Self {
        self.sortable = true;
        self
    }

    /// Disable sorting for this column
    pub fn without_sorting(mut self) -> Self {
        self.sortable = false;
        self
    }

    /// Set column alignment
    pub fn align(mut self, align: TextAlignment) -> Self {
        self.alignment = align;
        self
    }

    /// Align content to start (left in LTR)
    pub fn align_start(mut self) -> Self {
        self.alignment = TextAlignment::Start;
        self
    }

    /// Align content to center
    pub fn align_center(mut self) -> Self {
        self.alignment = TextAlignment::Center;
        self
    }

    /// Align content to end (right in LTR)
    pub fn align_end(mut self) -> Self {
        self.alignment = TextAlignment::End;
        self
    }

    /// Set column data type
    pub fn data_type(mut self, data_type: ColumnDataType) -> Self {
        self.data_type = data_type;
        self
    }

    /// Make column sticky to the left
    pub fn sticky(mut self) -> Self {
        self.sticky = true;
        self
    }

    /// Remove sticky behavior
    pub fn not_sticky(mut self) -> Self {
        self.sticky = false;
        self
    }
}
