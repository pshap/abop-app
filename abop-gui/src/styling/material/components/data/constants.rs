//! Constants for Material Design 3 data table components

/// Table density constants following Material Design specifications
pub mod density {
    /// Row height for compact density (minimal spacing)
    pub const COMPACT_ROW_HEIGHT: f32 = 32.0;
    /// Row height for standard density (balanced spacing)
    pub const STANDARD_ROW_HEIGHT: f32 = 48.0;
    /// Row height for comfortable density (generous spacing)
    pub const COMFORTABLE_ROW_HEIGHT: f32 = 56.0;

    /// Header height for compact density
    pub const COMPACT_HEADER_HEIGHT: f32 = 40.0;
    /// Header height for standard density
    pub const STANDARD_HEADER_HEIGHT: f32 = 56.0;
    /// Header height for comfortable density
    pub const COMFORTABLE_HEADER_HEIGHT: f32 = 64.0;

    /// Cell padding for compact density
    pub const COMPACT_CELL_PADDING: f32 = 8.0;
    /// Cell padding for standard density
    pub const STANDARD_CELL_PADDING: f32 = 12.0;
    /// Cell padding for comfortable density
    pub const COMFORTABLE_CELL_PADDING: f32 = 16.0;
}

/// Default configuration values
pub mod defaults {
    /// Default minimum column width in pixels
    pub const MIN_COLUMN_WIDTH: f32 = 80.0;
    /// Default minimum row height in pixels
    pub const MIN_ROW_HEIGHT: f32 = 32.0;
    /// Default border width in pixels
    pub const BORDER_WIDTH: f32 = 1.0;
    /// Default border radius in pixels
    pub const BORDER_RADIUS: f32 = 4.0;

    /// Minimal configuration minimum column width
    pub const MINIMAL_MIN_COLUMN_WIDTH: f32 = 60.0;
    /// Minimal configuration minimum row height
    pub const MINIMAL_MIN_ROW_HEIGHT: f32 = 28.0;

    /// Advanced configuration minimum column width
    pub const ADVANCED_MIN_COLUMN_WIDTH: f32 = 100.0;
    /// Advanced configuration minimum row height
    pub const ADVANCED_MIN_ROW_HEIGHT: f32 = 40.0;
    /// Advanced configuration maximum row height
    pub const ADVANCED_MAX_ROW_HEIGHT: f32 = 120.0;
    /// Advanced configuration header height
    pub const ADVANCED_HEADER_HEIGHT: f32 = 64.0;
    /// Advanced configuration footer height
    pub const ADVANCED_FOOTER_HEIGHT: f32 = 48.0;
    /// Advanced configuration border radius
    pub const ADVANCED_BORDER_RADIUS: f32 = 8.0;

    /// Default virtual scrolling max visible rows
    pub const DEFAULT_MAX_VISIBLE_ROWS: usize = 100;
}
