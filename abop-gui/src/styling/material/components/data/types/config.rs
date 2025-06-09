//! Data table configuration and builder methods

use super::core::TableDensity;
use super::super::constants::defaults;

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
