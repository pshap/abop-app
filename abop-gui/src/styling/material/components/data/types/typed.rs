//! Type-safe data table configuration with compile-time guarantees

use super::config::DataTableConfig;

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
    /// Select multiple rows at once (only available for selectable tables)
    ///
    /// # Arguments
    /// * `indices` - Collection of row indices to select
    ///
    /// # Type Safety
    /// This method is only available when `SELECTABLE` is `true` at compile time
    pub fn select_rows(&self, indices: &[usize]) {
        // Implementation would interact with table state
        // This is a compile-time safe method
        println!("Selecting rows: {:?}", indices);
    }

    /// Get currently selected rows (only available for selectable tables)
    ///
    /// # Returns
    /// A vector of selected row indices
    ///
    /// # Type Safety
    /// This method is only available when `SELECTABLE` is `true` at compile time
    pub fn selected_rows(&self) -> Vec<usize> {
        // Implementation would return actual selected rows
        // This is a placeholder
        vec![]
    }
}

impl<const SELECTABLE: bool, const VIRTUAL: bool> TypedDataTableConfig<SELECTABLE, true, VIRTUAL> {
    /// Sort by column (only available for sortable tables)
    ///
    /// # Arguments
    /// * `column_id` - The ID of the column to sort by
    /// * `direction` - The sort direction
    ///
    /// # Type Safety
    /// This method is only available when `SORTABLE` is `true` at compile time
    pub fn sort_by_column(&self, column_id: &str, direction: super::core::SortDirection) {
        // Implementation would interact with table sorting
        // This is a compile-time safe method
        println!("Sorting by column '{}' in direction {:?}", column_id, direction);
    }

    /// Get current sort state (only available for sortable tables)
    ///
    /// # Returns
    /// Current sort column and direction
    ///
    /// # Type Safety
    /// This method is only available when `SORTABLE` is `true` at compile time
    pub fn sort_state(&self) -> Option<(String, super::core::SortDirection)> {
        // Implementation would return actual sort state
        // This is a placeholder
        None
    }
}

impl<const SELECTABLE: bool, const SORTABLE: bool>
    TypedDataTableConfig<SELECTABLE, SORTABLE, true>
{
    /// Scroll to specific row (only available for virtual scrolling tables)
    ///
    /// # Arguments
    /// * `row_index` - The index of the row to scroll to
    ///
    /// # Type Safety
    /// This method is only available when `VIRTUAL` is `true` at compile time
    pub fn scroll_to_row(&self, row_index: usize) {
        // Implementation would handle virtual scrolling
        // This is a compile-time safe method
        println!("Scrolling to row {}", row_index);
    }

    /// Get visible row range (only available for virtual scrolling tables)
    ///
    /// # Returns
    /// The range of currently visible row indices
    ///
    /// # Type Safety
    /// This method is only available when `VIRTUAL` is `true` at compile time
    pub fn visible_range(&self) -> std::ops::Range<usize> {
        // Implementation would return actual visible range
        // This is a placeholder
        0..10
    }

    /// Set virtual scrolling viewport (only available for virtual scrolling tables)
    ///
    /// # Arguments
    /// * `start_index` - First visible row index
    /// * `visible_count` - Number of visible rows
    ///
    /// # Type Safety
    /// This method is only available when `VIRTUAL` is `true` at compile time
    pub fn set_viewport(&self, start_index: usize, visible_count: usize) {
        // Implementation would update virtual scrolling viewport
        // This is a compile-time safe method
        println!("Setting viewport: start={}, count={}", start_index, visible_count);
    }
}

// Type aliases for common configurations
/// Read-only table configuration (no selection, no sorting, no virtual scrolling)
pub type ReadOnlyTableConfig = TypedDataTableConfig<false, false, false>;

/// Interactive table configuration (with selection and sorting, no virtual scrolling)
pub type InteractiveTableConfig = TypedDataTableConfig<true, true, false>;

/// Large dataset table configuration (with virtual scrolling, may have selection and sorting)
pub type VirtualTableConfig = TypedDataTableConfig<true, true, true>;

/// Simple data display table (with sorting but no selection or virtual scrolling)
pub type DisplayTableConfig = TypedDataTableConfig<false, true, false>;

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::core::{TableDensity, SortDirection};

    #[test]
    fn test_typed_config_creation() {
        let base_config = DataTableConfig::new();
        let typed_config: ReadOnlyTableConfig = TypedDataTableConfig::new(base_config);
        
        assert!(typed_config.config().density == TableDensity::Standard);
    }

    #[test]
    fn test_selectable_methods() {
        let base_config = DataTableConfig::new().with_selection();
        let selectable_config: InteractiveTableConfig = TypedDataTableConfig::new(base_config);
        
        // These methods should compile since SELECTABLE = true
        selectable_config.select_rows(&[0, 1, 2]);
        let _selected = selectable_config.selected_rows();
    }

    #[test]
    fn test_sortable_methods() {
        let base_config = DataTableConfig::new().with_sorting();
        let sortable_config: InteractiveTableConfig = TypedDataTableConfig::new(base_config);
        
        // These methods should compile since SORTABLE = true
        sortable_config.sort_by_column("name", SortDirection::Ascending);
        let _sort_state = sortable_config.sort_state();
    }

    #[test]
    fn test_virtual_methods() {
        let base_config = DataTableConfig::new().with_virtual_scrolling(Some(100));
        let virtual_config: VirtualTableConfig = TypedDataTableConfig::new(base_config);
        
        // These methods should compile since VIRTUAL = true
        virtual_config.scroll_to_row(50);
        let _range = virtual_config.visible_range();
        virtual_config.set_viewport(10, 20);
    }

    #[test]
    fn test_config_round_trip() {
        let original_config = DataTableConfig::new()
            .with_selection()
            .with_sorting()
            .compact();
        
        let typed_config: InteractiveTableConfig = TypedDataTableConfig::new(original_config.clone());
        let recovered_config = typed_config.into_config();
        
        assert!(recovered_config.selectable == original_config.selectable);
        assert!(recovered_config.sortable == original_config.sortable);
        assert!(recovered_config.density == original_config.density);
    }
}
