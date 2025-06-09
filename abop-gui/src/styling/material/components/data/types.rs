//! Core types and data structures for Material Design 3 data components

use iced::Padding;

use super::constants::{density, defaults};

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
    /// Whether to show the table header
    pub show_header: bool,
    /// Whether to show the table footer
    pub show_footer: bool,
    /// Whether columns are sortable by default
    pub sortable: bool,
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
            min_column_width: defaults::MIN_COLUMN_WIDTH,

            // Table styling
            density: TableDensity::Standard,
            show_header: true,
            show_footer: false,
            sortable: true,
            min_row_height: defaults::MIN_ROW_HEIGHT,
            max_row_height: None,
            header_height: None,
            footer_height: None,
            border_color: None,
            border_width: defaults::BORDER_WIDTH,
            border_radius: defaults::BORDER_RADIUS,
        }
    }
}

impl DataTableConfig {
    /// Create a new data table configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a minimal table configuration for simple use cases
    pub fn minimal() -> Self {
        Self {
            selectable: false,
            hoverable: true,
            sticky_header: false,
            striped: false,
            virtual_scrolling: false,
            max_visible_rows: None,
            row_actions: false,
            resizable_columns: false,
            min_column_width: defaults::MINIMAL_MIN_COLUMN_WIDTH,
            density: TableDensity::Compact,
            show_header: true,
            show_footer: false,
            sortable: false,
            min_row_height: defaults::MINIMAL_MIN_ROW_HEIGHT,
            max_row_height: None,
            header_height: None,
            footer_height: None,
            border_color: None,
            border_width: 0.0,
            border_radius: 0.0,
        }
    }

    /// Create a feature-rich table configuration for complex data
    pub fn advanced() -> Self {
        Self {
            selectable: true,
            hoverable: true,
            sticky_header: true,
            striped: true,
            virtual_scrolling: true,
            max_visible_rows: Some(defaults::DEFAULT_MAX_VISIBLE_ROWS),
            row_actions: true,
            resizable_columns: true,
            min_column_width: defaults::ADVANCED_MIN_COLUMN_WIDTH,
            density: TableDensity::Comfortable,
            show_header: true,
            show_footer: true,
            sortable: true,
            min_row_height: defaults::ADVANCED_MIN_ROW_HEIGHT,
            max_row_height: Some(defaults::ADVANCED_MAX_ROW_HEIGHT),
            header_height: Some(defaults::ADVANCED_HEADER_HEIGHT),
            footer_height: Some(defaults::ADVANCED_FOOTER_HEIGHT),
            border_color: Some(iced::Color::from_rgb(0.9, 0.9, 0.9)),
            border_width: defaults::BORDER_WIDTH,
            border_radius: defaults::ADVANCED_BORDER_RADIUS,
        }
    }

    // Behavior builder methods

    /// Set whether rows can be selected
    pub fn set_selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    /// Enable row selection
    pub fn with_selection(mut self) -> Self {
        self.selectable = true;
        self
    }

    /// Disable row selection
    pub fn without_selection(mut self) -> Self {
        self.selectable = false;
        self
    }

    /// Set whether rows show hover effects
    pub fn set_hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }

    /// Enable hover effects
    pub fn with_hover(mut self) -> Self {
        self.hoverable = true;
        self
    }

    /// Disable hover effects
    pub fn without_hover(mut self) -> Self {
        self.hoverable = false;
        self
    }

    /// Set whether the header sticks to the top
    pub fn set_sticky_header(mut self, sticky: bool) -> Self {
        self.sticky_header = sticky;
        self
    }

    /// Enable sticky header
    pub fn with_sticky_header(mut self) -> Self {
        self.sticky_header = true;
        self
    }

    /// Disable sticky header
    pub fn without_sticky_header(mut self) -> Self {
        self.sticky_header = false;
        self
    }

    /// Set whether alternating rows have different backgrounds
    pub fn set_striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    /// Enable striped rows
    pub fn with_stripes(mut self) -> Self {
        self.striped = true;
        self
    }

    /// Disable striped rows
    pub fn without_stripes(mut self) -> Self {
        self.striped = false;
        self
    }
    /// Set whether to use virtual scrolling
    pub fn enable_virtual_scrolling(mut self, virtual_enabled: bool) -> Self {
        self.virtual_scrolling = virtual_enabled;
        self
    }

    /// Enable virtual scrolling with optional max visible rows
    pub fn with_virtual_scrolling(mut self, max_visible: Option<usize>) -> Self {
        self.virtual_scrolling = true;
        self.max_visible_rows = max_visible;
        self
    }

    /// Disable virtual scrolling
    pub fn without_virtual_scrolling(mut self) -> Self {
        self.virtual_scrolling = false;
        self.max_visible_rows = None;
        self
    }

    /// Set maximum number of visible rows
    pub fn max_visible_rows(mut self, max: usize) -> Self {
        self.max_visible_rows = Some(max);
        self
    }

    /// Set whether rows have action buttons
    pub fn set_row_actions(mut self, actions: bool) -> Self {
        self.row_actions = actions;
        self
    }

    /// Enable row actions
    pub fn with_row_actions(mut self) -> Self {
        self.row_actions = true;
        self
    }

    /// Disable row actions
    pub fn without_row_actions(mut self) -> Self {
        self.row_actions = false;
        self
    }

    /// Set whether columns can be resized
    pub fn set_resizable_columns(mut self, resizable: bool) -> Self {
        self.resizable_columns = resizable;
        self
    }

    /// Enable column resizing
    pub fn with_resizable_columns(mut self) -> Self {
        self.resizable_columns = true;
        self
    }

    /// Disable column resizing
    pub fn without_resizable_columns(mut self) -> Self {
        self.resizable_columns = false;
        self
    }

    /// Set minimum column width
    pub fn min_column_width(mut self, width: f32) -> Self {
        self.min_column_width = width;
        self
    }

    // Styling builder methods

    /// Set table density
    pub fn density(mut self, density: TableDensity) -> Self {
        self.density = density;
        self
    }

    /// Use compact density
    pub fn compact(mut self) -> Self {
        self.density = TableDensity::Compact;
        self
    }

    /// Use standard density
    pub fn standard(mut self) -> Self {
        self.density = TableDensity::Standard;
        self
    }

    /// Use comfortable density
    pub fn comfortable(mut self) -> Self {
        self.density = TableDensity::Comfortable;
        self
    }

    /// Set whether to show the header
    pub fn set_show_header(mut self, show: bool) -> Self {
        self.show_header = show;
        self
    }

    /// Show the header
    pub fn with_header(mut self) -> Self {
        self.show_header = true;
        self
    }

    /// Hide the header
    pub fn without_header(mut self) -> Self {
        self.show_header = false;
        self
    }

    /// Set whether to show the footer
    pub fn set_show_footer(mut self, show: bool) -> Self {
        self.show_footer = show;
        self
    }

    /// Show the footer
    pub fn with_footer(mut self) -> Self {
        self.show_footer = true;
        self
    }

    /// Hide the footer
    pub fn without_footer(mut self) -> Self {
        self.show_footer = false;
        self
    }

    /// Set whether columns are sortable by default
    pub fn set_sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    /// Enable sorting by default
    pub fn with_sorting(mut self) -> Self {
        self.sortable = true;
        self
    }

    /// Disable sorting by default
    pub fn without_sorting(mut self) -> Self {
        self.sortable = false;
        self
    }

    /// Set row height constraints
    pub fn row_height(mut self, min: f32, max: Option<f32>) -> Self {
        self.min_row_height = min;
        self.max_row_height = max;
        self
    }

    /// Set minimum row height
    pub fn min_row_height(mut self, height: f32) -> Self {
        self.min_row_height = height;
        self
    }

    /// Set maximum row height
    pub fn max_row_height(mut self, height: f32) -> Self {
        self.max_row_height = Some(height);
        self
    }

    /// Set fixed header height
    pub fn header_height(mut self, height: f32) -> Self {
        self.header_height = Some(height);
        self
    }

    /// Set fixed footer height
    pub fn footer_height(mut self, height: f32) -> Self {
        self.footer_height = Some(height);
        self
    }

    /// Set border styling
    pub fn border(mut self, color: iced::Color, width: f32, radius: f32) -> Self {
        self.border_color = Some(color);
        self.border_width = width;
        self.border_radius = radius;
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: iced::Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Set border width
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = width;
        self
    }

    /// Set border radius
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Remove borders
    pub fn without_borders(mut self) -> Self {
        self.border_color = None;
        self.border_width = 0.0;
        self
    }
}

/// Type-safe table configuration with compile-time guarantees
///
/// This generic struct provides compile-time guarantees about the table's capabilities
/// through const generics:
/// - `SELECTABLE`: Whether rows can be selected
/// - `SORTABLE`: Whether columns can be sorted
/// - `VIRTUAL`: Whether virtual scrolling is enabled
#[derive(Debug, Clone)]
pub struct TypedDataTableConfig<const SELECTABLE: bool, const SORTABLE: bool, const VIRTUAL: bool> {
    config: DataTableConfig,
}

impl<const SELECTABLE: bool, const SORTABLE: bool, const VIRTUAL: bool>
    TypedDataTableConfig<SELECTABLE, SORTABLE, VIRTUAL>
{
    /// Create a new typed configuration with the specified compile-time guarantees
    ///
    /// # Arguments
    /// * `config` - The underlying table configuration
    ///
    /// # Returns
    /// A new `TypedDataTableConfig` instance with the specified capabilities
    pub fn new(config: DataTableConfig) -> Self {
        Self { config }
    }

    /// Get a reference to the underlying configuration
    ///
    /// # Returns
    /// A reference to the internal `DataTableConfig`
    pub fn config(&self) -> &DataTableConfig {
        &self.config
    }

    /// Consume this typed configuration and return the underlying untyped configuration
    ///
    /// # Returns
    /// The inner `DataTableConfig` without type parameters
    pub fn into_config(self) -> DataTableConfig {
        self.config
    }
}

// Compile-time methods only available for specific configurations
impl<const SORTABLE: bool, const VIRTUAL: bool> TypedDataTableConfig<true, SORTABLE, VIRTUAL> {
    /// Selects all rows in the table
    ///
    /// # Returns
    /// `true` if the selection was changed, `false` otherwise
    ///
    /// # Note
    /// This method is only available when `SELECTABLE` is `true`
    pub fn select_all(&self) -> bool {
        // Implementation would be here
        true
    }

    /// Clears the current selection of rows
    ///
    /// # Note
    /// This method is only available when `SELECTABLE` is `true`
    pub fn clear_selection(&self) {
        // Implementation would be here
    }
}

impl<const SELECTABLE: bool, const VIRTUAL: bool> TypedDataTableConfig<SELECTABLE, true, VIRTUAL> {
    /// Sorts the table by the specified column and direction
    ///
    /// # Arguments
    /// * `_column_id` - The ID of the column to sort by
    /// * `_direction` - The sort direction (Ascending or Descending)
    ///
    /// # Note
    /// This method is only available when `SORTABLE` is `true`
    pub fn sort_by_column(&self, _column_id: &str, _direction: SortDirection) {
        // Implementation would be here
    }

    /// Clears the current sort order of the table
    ///
    /// # Note
    /// This method is only available when `SORTABLE` is `true`
    pub fn clear_sort(&self) {
        // Implementation would be here
    }
}

impl<const SELECTABLE: bool, const SORTABLE: bool>
    TypedDataTableConfig<SELECTABLE, SORTABLE, true>
{
    /// Scrolls the table to make the specified row visible
    ///
    /// # Arguments
    /// * `_row_index` - The index of the row to scroll to
    ///
    /// # Note
    /// This method is only available when `VIRTUAL` is `true`
    pub fn scroll_to_row(&self, _row_index: usize) {
        // Implementation would be here
    }

    /// Gets the range of currently visible row indices
    ///
    /// # Returns
    /// A tuple containing the (start_index, end_index) of visible rows
    ///
    /// # Note
    /// This method is only available when `VIRTUAL` is `true`
    pub fn get_visible_range(&self) -> (usize, usize) {
        // Implementation would be here
        (0, 10)
    }
}
