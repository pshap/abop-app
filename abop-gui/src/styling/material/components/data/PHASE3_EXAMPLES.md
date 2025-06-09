# Material Design 3 Data Components - Phase 3 Builder Patterns

This document demonstrates the enhanced builder patterns and type-safe APIs implemented in Phase 3 of the refactoring.

## Table of Contents
1. [Configuration Presets](#configuration-presets)
2. [Builder Patterns](#builder-patterns)
3. [Type-Safe APIs](#type-safe-apis)
4. [Fluent Column Building](#fluent-column-building)
5. [Complete Layout Examples](#complete-layout-examples)
6. [Migration Guide](#migration-guide)

## Configuration Presets

### Quick Start with Presets

```rust
use crate::styling::material::components::data::*;

// Readonly table for dashboards
let readonly_config = TableConfigPresets::readonly();

// Interactive table for data manipulation
let interactive_config = TableConfigPresets::interactive();

// Optimized for large datasets
let large_dataset_config = TableConfigPresets::large_dataset(1000);

// For editing data inline
let editable_config = TableConfigPresets::editable();

// Mobile-optimized table
let mobile_config = TableConfigPresets::mobile();

// Material Design emphasized styling
let material_config = TableConfigPresets::material_emphasized();
```

### Using Preset Macros

```rust
// More concise syntax using macros
let config1 = table_config!(readonly);
let config2 = table_config!(interactive);
let config3 = table_config!(large_dataset, 500);
let config4 = table_config!(editable);
let config5 = table_config!(mobile);
let config6 = table_config!(material);
```

## Builder Patterns

### DataTableConfig Builder

```rust
// Fluent configuration building
let config = DataTableConfig::new()
    .with_selection()
    .with_hover()
    .with_stripes()
    .with_sticky_header()
    .comfortable()
    .border(Color::from_rgb(0.8, 0.8, 0.8), 1.0, 4.0)
    .max_visible_rows(50);

// Starting from presets and customizing
let custom_config = DataTableConfig::minimal()
    .with_selection()
    .standard()
    .border_radius(8.0);
```

### DataTableBuilder (Alternative Approach)

```rust
// Using the dedicated builder
let config = DataTableBuilder::new()
    .selectable(true)
    .hoverable(true)
    .striped(true)
    .compact()
    .with_virtual_scrolling(Some(100))
    .border_width(2.0)
    .build();

// Starting with presets
let config = DataTableBuilder::advanced()
    .without_stripes()
    .compact()
    .max_visible_rows(200)
    .build();
```

## Type-Safe APIs

### Compile-Time Configuration Guarantees

```rust
// Configuration with compile-time type checking
let typed_config: TypedDataTableConfig<true, true, false> = 
    TypedDataTableConfig::new(
        DataTableConfig::new()
            .with_selection()    // SELECTABLE = true
            .with_sorting()      // SORTABLE = true
            // No virtual scrolling, so VIRTUAL = false
    );

// These methods are only available when SELECTABLE = true
typed_config.select_all();
typed_config.clear_selection();

// These methods are only available when SORTABLE = true
typed_config.sort_by_column("name", SortDirection::Ascending);
typed_config.clear_sort();

// This would NOT compile because VIRTUAL = false
// typed_config.scroll_to_row(10); // Compilation error!
```

### Type-Safe Column Building

```rust
// Different constructors for different data types
let text_column = TableColumn::text("name", "Full Name")
    .with_sorting()
    .fixed_width(200.0);

let number_column = TableColumn::number("age", "Age")
    .with_sorting()
    .align_end(); // Automatically right-aligned

let date_column = TableColumn::date("created", "Created At")
    .with_sorting()
    .fixed_width(150.0);

let boolean_column = TableColumn::boolean("active", "Active")
    .align_center() // Automatically center-aligned
    .fixed_width(80.0);

let custom_column = TableColumn::custom("actions", "Actions")
    .without_sorting()
    .shrink();
```

## Fluent Column Building

### Column Builder Utilities

```rust
use crate::styling::material::components::data::builders::ColumnBuilder;

// Smart defaults for common column types
let columns = vec![
    ColumnBuilder::text_auto("name", "Name"),           // Auto-width, sortable
    ColumnBuilder::text_fixed("email", "Email", 250.0), // Fixed width
    ColumnBuilder::number_column("age", "Age"),          // Right-aligned, sortable
    ColumnBuilder::date_column("created", "Created"),    // Appropriate width
    ColumnBuilder::checkbox_column("active", "Active"),  // Center-aligned, not sortable
    ColumnBuilder::actions_column("Actions"),            // For buttons/actions
    ColumnBuilder::id_column(),                         // Minimal ID column
    ColumnBuilder::sticky_column("priority", "Priority", 100.0), // Sticky to left
];
```

### Column Macros

```rust
// Concise column definition syntax
let columns = vec![
    column!(text: "name", "Name"),
    column!(text: "email", "Email", 250.0),
    column!(number: "age", "Age"),
    column!(date: "created", "Created"),
    column!(checkbox: "active", "Active"),
    column!(actions: "Actions"),
    column!(id),
    column!(sticky: "priority", "Priority", 100.0),
];
```

## Complete Layout Examples

### TableLayoutBuilder

```rust
// Building complete table layouts
let layout = TableLayout::interactive()
    .text_column("id", "ID")
    .text_column("name", "Name")
    .number_column("age", "Age")
    .date_column("created", "Created At")
    .checkbox_column("active", "Active")
    .actions_column("Actions")
    .configure(|config| 
        config.comfortable()
              .with_stripes()
              .border_radius(8.0)
    )
    .build();

// Validate the layout
layout.validate().expect("Layout should be valid");

// Extract information
let column_ids = layout.column_ids();
let fixed_width = layout.fixed_width();
let fill_portions = layout.fill_portions();
```

### Using Layout Presets

```rust
// Different layout patterns
let readonly_layout = TableLayout::readonly()
    .text_column("title", "Title")
    .text_column("description", "Description")
    .date_column("updated", "Last Updated")
    .build();

let large_data_layout = TableLayout::large_dataset(500)
    .column(ColumnBuilder::id_column())
    .text_column("name", "Name")
    .number_column("score", "Score")
    .configure(|config| config.min_column_width(50.0))
    .build();

let editable_layout = TableLayout::editable()
    .text_column("title", "Title")
    .text_column("content", "Content")
    .checkbox_column("published", "Published")
    .actions_column("Edit")
    .build();
```

### Table Layout Macro

```rust
// Declarative table layout syntax
let layout = table_layout!(
    preset: interactive,
    columns: [
        column!(text: "name", "Name", 200.0),
        column!(number: "age", "Age"),
        column!(date: "created", "Created"),
        column!(actions: "Actions"),
    ]
);

// Custom configuration with macro
let layout = table_layout!(
    config: DataTableConfig::new()
        .comfortable()
        .with_stripes()
        .with_selection(),
    columns: [
        column!(sticky: "id", "ID", 80.0),
        column!(text: "title", "Title"),
        column!(checkbox: "active", "Active"),
        column!(actions: "Actions"),
    ]
);
```

## Migration Guide

### From Old API to New Builder Patterns

#### Before (Manual Configuration)
```rust
// Old way - manual field setting
let mut config = DataTableConfig::default();
config.selectable = true;
config.hoverable = true;
config.striped = true;
config.density = TableDensity::Comfortable;
config.border_radius = 8.0;

let mut column = TableColumn::new("name", "Name");
column.sortable = true;
column.width = ColumnWidth::Fixed(200.0);
column.alignment = TextAlignment::Start;
```

#### After (Builder Patterns)
```rust
// New way - fluent builders
let config = DataTableConfig::new()
    .with_selection()
    .with_hover()
    .with_stripes()
    .comfortable()
    .border_radius(8.0);

let column = TableColumn::text("name", "Name")
    .with_sorting()
    .fixed_width(200.0)
    .align_start();

// Or even more concise
let column = ColumnBuilder::text_fixed("name", "Name", 200.0);
```

### From Individual Components to Layout Building

#### Before (Separate Components)
```rust
// Old way - managing columns and config separately
let columns = vec![
    TableColumn::new("name", "Name"),
    TableColumn::new("age", "Age"),
];
let config = DataTableConfig::default();

// Separate validation and processing
```

#### After (Integrated Layout)
```rust
// New way - integrated layout with validation
let layout = TableLayout::interactive()
    .text_column("name", "Name")
    .number_column("age", "Age")
    .build();

layout.validate().expect("Valid layout");
```

### Backward Compatibility

All existing APIs continue to work:

```rust
// These still work exactly as before
let config = DataTableConfig::default();
let column = TableColumn::new("id", "ID");
let width = ColumnWidth::Fixed(100.0);

// New builder methods are additive, not replacing
let enhanced_config = config.with_selection().comfortable();
let enhanced_column = column.with_sorting().fixed_width(150.0);
```

## Best Practices

1. **Use presets for common patterns**: Start with `TableConfigPresets` or layout presets
2. **Leverage type safety**: Use `TypedDataTableConfig` when you need compile-time guarantees
3. **Validate layouts**: Always call `layout.validate()` on complex layouts
4. **Use column builders**: Prefer `ColumnBuilder` for standard column types
5. **Combine fluent APIs**: Chain builder methods for readable configuration
6. **Use macros sparingly**: Macros are great for repetitive patterns but can reduce IDE support

## Performance Considerations

- Builder patterns have zero runtime cost - they're compile-time constructs
- Type-safe configurations use const generics for zero-cost abstractions
- Layout validation is optional and can be disabled in release builds
- Preset configurations are pre-computed and optimized
