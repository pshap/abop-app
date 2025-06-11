//! Builder patterns and helper utilities for creating Material Design 3 data tables
//!
//! This module provides ergonomic APIs for building complex table configurations
//! with compile-time safety and runtime flexibility.

use super::types::*;
use iced::Color;

/// Ergonomic table builder with semantic presets
pub struct TableConfigPresets;

impl TableConfigPresets {
    /// Create a simple read-only table for displaying data
    /// Perfect for dashboards and reports
    pub fn readonly() -> DataTableConfig {
        DataTableConfig::minimal()
            .without_selection()
            .without_sorting()
            .without_row_actions()
            .without_resizable_columns()
            .compact()
    }

    /// Create an interactive table for data manipulation
    /// Includes selection, sorting, and row actions
    pub fn interactive() -> DataTableConfig {
        DataTableConfig::new()
            .with_selection()
            .with_sorting()
            .with_row_actions()
            .with_resizable_columns()
            .with_stripes()
            .standard()
    }

    /// Create a table optimized for large datasets
    /// Uses virtual scrolling and minimal features for performance
    pub fn large_dataset(max_visible: usize) -> DataTableConfig {
        DataTableConfig::new()
            .with_virtual_scrolling(Some(max_visible))
            .compact()
            .without_stripes()
            .without_row_actions()
            .min_column_width(60.0)
    }

    /// Create a table for editing data inline
    /// Optimized for comfortable editing experience
    pub fn editable() -> DataTableConfig {
        DataTableConfig::advanced()
            .comfortable()
            .with_selection()
            .with_row_actions()
            .max_row_height(80.0)
            .min_row_height(56.0)
    }

    /// Create a compact table for mobile or constrained spaces
    pub fn mobile() -> DataTableConfig {
        DataTableConfig::minimal()
            .compact()
            .without_resizable_columns()
            .min_column_width(40.0)
            .without_borders()
    }

    /// Create a table with Material Design emphasis on visual hierarchy
    pub fn material_emphasized() -> DataTableConfig {
        DataTableConfig::new()
            .comfortable()
            .with_stripes()
            .with_sticky_header()
            .border(Color::from_rgb(0.0, 0.0, 0.0), 1.0, 8.0)
            .header_height(64.0)
    }
}

/// Column builder with smart defaults for common data types
pub struct ColumnBuilder;

impl ColumnBuilder {
    /// Create a sortable text column with auto width
    pub fn text_auto(id: &str, title: &str) -> TableColumn {
        TableColumn::text(id, title).with_sorting().auto_width()
    }

    /// Create a fixed-width text column
    pub fn text_fixed(id: &str, title: &str, width: f32) -> TableColumn {
        TableColumn::text(id, title)
            .with_sorting()
            .fixed_width(width)
    }

    /// Create a number column with right alignment
    pub fn number_column(id: &str, title: &str) -> TableColumn {
        TableColumn::number(id, title)
            .with_sorting()
            .fixed_width(100.0)
    }

    /// Create a date column with appropriate width
    pub fn date_column(id: &str, title: &str) -> TableColumn {
        TableColumn::date(id, title)
            .with_sorting()
            .fixed_width(120.0)
    }

    /// Create a boolean column with center alignment
    pub fn checkbox_column(id: &str, title: &str) -> TableColumn {
        TableColumn::boolean(id, title)
            .align_center()
            .fixed_width(60.0)
            .without_sorting()
    }

    /// Create an action column (typically for buttons)
    pub fn actions_column(title: &str) -> TableColumn {
        TableColumn::custom("actions", title)
            .align_center()
            .fixed_width(120.0)
            .without_sorting()
    }

    /// Create an ID column (typically hidden or minimal)
    pub fn id_column() -> TableColumn {
        TableColumn::text("id", "ID").shrink().without_sorting()
    }

    /// Create a priority sticky column (stays visible when scrolling)
    pub fn sticky_column(id: &str, title: &str, width: f32) -> TableColumn {
        TableColumn::text(id, title)
            .fixed_width(width)
            .sticky()
            .with_sorting()
    }
}

/// Fluent API for building complex table layouts
pub struct TableLayoutBuilder {
    columns: Vec<TableColumn>,
    config: DataTableConfig,
}

impl TableLayoutBuilder {
    /// Start building a new table layout
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            config: DataTableConfig::default(),
        }
    }

    /// Start with a preset configuration
    pub fn from_preset(config: DataTableConfig) -> Self {
        Self {
            columns: Vec::new(),
            config,
        }
    }

    /// Add a column to the table
    pub fn column(mut self, column: TableColumn) -> Self {
        self.columns.push(column);
        self
    }

    /// Add multiple columns at once
    pub fn columns(mut self, columns: Vec<TableColumn>) -> Self {
        self.columns.extend(columns);
        self
    }

    /// Add a text column with auto-configuration
    pub fn text_column(mut self, id: &str, title: &str) -> Self {
        self.columns.push(ColumnBuilder::text_auto(id, title));
        self
    }

    /// Add a number column with auto-configuration
    pub fn number_column(mut self, id: &str, title: &str) -> Self {
        self.columns.push(ColumnBuilder::number_column(id, title));
        self
    }

    /// Add a date column with auto-configuration
    pub fn date_column(mut self, id: &str, title: &str) -> Self {
        self.columns.push(ColumnBuilder::date_column(id, title));
        self
    }

    /// Add a checkbox column
    pub fn checkbox_column(mut self, id: &str, title: &str) -> Self {
        self.columns.push(ColumnBuilder::checkbox_column(id, title));
        self
    }

    /// Add an actions column
    pub fn actions_column(mut self, title: &str) -> Self {
        self.columns.push(ColumnBuilder::actions_column(title));
        self
    }

    /// Modify the table configuration
    pub fn configure<F>(mut self, f: F) -> Self
    where
        F: FnOnce(DataTableConfig) -> DataTableConfig,
    {
        self.config = f(self.config);
        self
    }

    /// Build the final table layout
    pub fn build(self) -> TableLayout {
        TableLayout {
            columns: self.columns,
            config: self.config,
        }
    }
}

impl Default for TableLayoutBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete table layout with columns and configuration
#[derive(Debug, Clone, Default)]
pub struct TableLayout {
    /// List of table columns to display
    pub columns: Vec<TableColumn>,
    /// Configuration for table behavior and styling
    pub config: DataTableConfig,
}

impl TableLayout {
    /// Create a new table layout builder
    pub fn builder() -> TableLayoutBuilder {
        TableLayoutBuilder::new()
    }

    /// Create a readonly table layout
    pub fn readonly() -> TableLayoutBuilder {
        TableLayoutBuilder::from_preset(TableConfigPresets::readonly())
    }

    /// Create an interactive table layout
    pub fn interactive() -> TableLayoutBuilder {
        TableLayoutBuilder::from_preset(TableConfigPresets::interactive())
    }

    /// Create a layout for large datasets
    pub fn large_dataset(max_visible: usize) -> TableLayoutBuilder {
        TableLayoutBuilder::from_preset(TableConfigPresets::large_dataset(max_visible))
    }

    /// Create an editable table layout
    pub fn editable() -> TableLayoutBuilder {
        TableLayoutBuilder::from_preset(TableConfigPresets::editable())
    }

    /// Get all column IDs
    pub fn column_ids(&self) -> Vec<&str> {
        self.columns.iter().map(|col| col.id.as_str()).collect()
    }

    /// Get total fixed width (excluding auto and fill columns)
    pub fn fixed_width(&self) -> f32 {
        self.columns
            .iter()
            .filter_map(|col| match col.width {
                ColumnWidth::Fixed(width) => Some(width),
                _ => None,
            })
            .sum()
    }

    /// Get number of fill portions
    pub fn fill_portions(&self) -> u16 {
        self.columns
            .iter()
            .filter_map(|col| match col.width {
                ColumnWidth::FillPortion(portion) => Some(portion),
                _ => None,
            })
            .sum()
    }

    /// Validate the table layout
    pub fn validate(&self) -> Result<(), String> {
        if self.columns.is_empty() {
            return Err("Table must have at least one column".to_string());
        }

        // Check for duplicate column IDs
        let mut ids = std::collections::HashSet::new();
        for column in &self.columns {
            if !ids.insert(&column.id) {
                return Err(format!("Duplicate column ID: {}", column.id));
            }
        }

        // Validate column widths
        let has_auto = self
            .columns
            .iter()
            .any(|col| matches!(col.width, ColumnWidth::Auto));
        let has_fill = self
            .columns
            .iter()
            .any(|col| matches!(col.width, ColumnWidth::FillPortion(_)));

        if has_auto && has_fill {
            return Err("Cannot mix Auto width with FillPortion width".to_string());
        }

        Ok(())
    }
}

/// Type-safe table configuration macros for common patterns
#[macro_export]
macro_rules! table_config {
    // Simple readonly table
    (readonly) => {
        $crate::styling::material::components::data::builders::TableConfigPresets::readonly()
    };

    // Interactive table
    (interactive) => {
        $crate::styling::material::components::data::builders::TableConfigPresets::interactive()
    };

    // Large dataset table
    (large_dataset, $max_visible:expr) => {
        $crate::styling::material::components::data::builders::TableConfigPresets::large_dataset($max_visible)
    };

    // Editable table
    (editable) => {
        $crate::styling::material::components::data::builders::TableConfigPresets::editable()
    };

    // Mobile table
    (mobile) => {
        $crate::styling::material::components::data::builders::TableConfigPresets::mobile()
    };

    // Material emphasized table
    (material) => {
        $crate::styling::material::components::data::builders::TableConfigPresets::material_emphasized()
    };
}

/// Type-safe column definition macros
#[macro_export]
macro_rules! column {
    // Text column with auto width
    (text: $id:expr, $title:expr) => {
        $crate::styling::material::components::data::builders::ColumnBuilder::text_auto($id, $title)
    };

    // Text column with fixed width
    (text: $id:expr, $title:expr, $width:expr) => {
        $crate::styling::material::components::data::builders::ColumnBuilder::text_fixed(
            $id, $title, $width,
        )
    };

    // Number column
    (number: $id:expr, $title:expr) => {
        $crate::styling::material::components::data::builders::ColumnBuilder::number_column(
            $id, $title,
        )
    };

    // Date column
    (date: $id:expr, $title:expr) => {
        $crate::styling::material::components::data::builders::ColumnBuilder::date_column(
            $id, $title,
        )
    };

    // Checkbox column
    (checkbox: $id:expr, $title:expr) => {
        $crate::styling::material::components::data::builders::ColumnBuilder::checkbox_column(
            $id, $title,
        )
    };

    // Actions column
    (actions: $title:expr) => {
        $crate::styling::material::components::data::builders::ColumnBuilder::actions_column($title)
    };

    // ID column
    (id) => {
        $crate::styling::material::components::data::builders::ColumnBuilder::id_column()
    };

    // Sticky column
    (sticky: $id:expr, $title:expr, $width:expr) => {
        $crate::styling::material::components::data::builders::ColumnBuilder::sticky_column(
            $id, $title, $width,
        )
    };
}

/// Complete table layout macro for declaring tables inline
#[macro_export]
macro_rules! table_layout {
    (
        config: $config:expr,
        columns: [
            $($column:expr),* $(,)?
        ]
    ) => {
        $crate::styling::material::components::data::builders::TableLayout {
            config: $config,
            columns: vec![$($column),*],
        }
    };

    (
        preset: $preset:ident,
        columns: [
            $($column:expr),* $(,)?
        ]
    ) => {
        $crate::styling::material::components::data::builders::TableLayout {
            config: table_config!($preset),
            columns: vec![$($column),*],
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_configurations() {
        let readonly = TableConfigPresets::readonly();
        assert!(!readonly.selectable);
        assert!(!readonly.sortable);
        assert!(!readonly.row_actions);

        let interactive = TableConfigPresets::interactive();
        assert!(interactive.selectable);
        assert!(interactive.sortable);
        assert!(interactive.row_actions);

        let large = TableConfigPresets::large_dataset(100);
        assert!(large.virtual_scrolling);
        assert_eq!(large.max_visible_rows, Some(100));
    }

    #[test]
    fn test_column_builders() {
        let text_col = ColumnBuilder::text_auto("name", "Name");
        assert_eq!(text_col.id, "name");
        assert_eq!(text_col.title, "Name");
        assert!(text_col.sortable);
        assert_eq!(text_col.data_type, ColumnDataType::Text);

        let num_col = ColumnBuilder::number_column("age", "Age");
        assert_eq!(num_col.alignment, TextAlignment::End);
        assert_eq!(num_col.data_type, ColumnDataType::Number);

        let date_col = ColumnBuilder::date_column("created", "Created");
        assert_eq!(date_col.data_type, ColumnDataType::Date);

        let bool_col = ColumnBuilder::checkbox_column("active", "Active");
        assert_eq!(bool_col.alignment, TextAlignment::Center);
        assert!(!bool_col.sortable);
    }

    #[test]
    fn test_table_layout_builder() {
        let layout = TableLayout::interactive()
            .text_column("name", "Name")
            .number_column("age", "Age")
            .date_column("created", "Created")
            .actions_column("Actions")
            .configure(|config| config.compact())
            .build();

        assert_eq!(layout.columns.len(), 4);
        assert_eq!(layout.config.density, TableDensity::Compact);
        assert!(layout.config.selectable);

        layout.validate().expect("Layout should be valid");
    }

    #[test]
    fn test_layout_validation() {
        // Empty table should fail
        let empty_layout = TableLayout::default();
        assert!(empty_layout.validate().is_err());

        // Duplicate column IDs should fail
        let duplicate_layout = TableLayout {
            columns: vec![
                ColumnBuilder::text_auto("name", "Name"),
                ColumnBuilder::text_auto("name", "Other Name"),
            ],
            config: DataTableConfig::default(),
        };
        assert!(duplicate_layout.validate().is_err());

        // Valid layout should pass
        let valid_layout = TableLayout {
            columns: vec![
                ColumnBuilder::text_auto("name", "Name"),
                ColumnBuilder::number_column("age", "Age"),
            ],
            config: DataTableConfig::default(),
        };
        assert!(valid_layout.validate().is_ok());
    }
}
