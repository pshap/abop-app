//! Material Design 3 Components
//!
//! This module provides Material Design 3 component implementations that integrate
//! seamlessly with the Iced framework and ABOP's existing architecture.

/// Material Design 3 button styling system for centralized button styling
pub mod button_style;
/// Material Design 3 container components including cards, surfaces, dividers, and layout containers
pub mod containers;
/// Material Design 3 data components including tables, lists, and tree views
pub mod data;
/// Material Design 3 feedback components including progress indicators, badges, dialogs, and notifications
pub mod feedback;
/// Material Design 3 input components including text fields and form elements
pub mod inputs;
/// Material Design 3 menu constants for centralized menu component values
pub mod menu_constants;
/// Material Design 3 menu container styling system for centralized menu container styling
pub mod menu_container_style;
/// Material Design 3 menu item styling system for consistent menu button styling
pub mod menu_item_style;
/// Material Design 3 menu components including dropdown menus, context menus, and autocomplete
pub mod menus;
/// Material Design 3 navigation components - modularized with tab bars and breadcrumbs for audiobook apps
pub mod navigation;
/// Material Design 3 selection component styling system
///
/// Provides a centralized styling system for all selection components (Checkbox, Radio, Switch, Chip)
/// using the strategy pattern for consistent theming and behavior across components.
pub mod selection_style;

/// Material Design 3 selection components with state-based design
///
/// Comprehensive selection components including Checkbox, Switch, Radio, and Chip
/// with modern builder patterns, validation, and Material Design 3 compliance.
pub mod selection;

// Re-export selection components and styles
pub use selection_style::{
    CheckboxStrategy, ChipStrategy, RadioStrategy, SelectionColors, SelectionSize, SelectionState,
    SelectionStyleBuilder, SelectionStyleError, SelectionStyling, SelectionVariant, SwitchStrategy,
    checkbox_style, chip_style, radio_style, switch_style,
};

// Re-export selection components
pub use selection::{
    Checkbox, CheckboxBuilder, Switch, SwitchBuilder,
};
pub use selection::common::{CheckboxState, ComponentSize, SwitchState};
/// Phase 3: Complete Material Design 3 widget implementations as proper Iced widgets
pub mod widgets;

// Re-export specific items from each module to avoid ambiguity
pub use button_style::{
    ButtonColors, ButtonSizeVariant, ButtonStyleVariant, ButtonStyling, create_button_icon,
    create_button_style, get_button_size_properties, get_button_styling, get_icon_size_for_button,
};

// Re-export container components (specific items only)
pub use containers::{
    CardVariant, DividerOrientation, MaterialCard, MaterialDivider, MaterialSurface, SurfaceVariant,
};

// Re-export data components (avoiding glob conflicts)
pub use data::helpers as data_helpers;
pub use data::{
    ColumnDataType, ColumnWidth, DataTableConfig, MaterialDataTable, MaterialList,
    MaterialTreeView, SortDirection, SortState, TableColumn, TableDensity, TextAlignment,
};

// Re-export feedback components (specific items only)
pub use feedback::{MaterialProgressIndicator, ProgressSize, ProgressState, ProgressVariant};

// Re-export input components (specific items only)
pub use inputs::{
    MaterialSearchField, MaterialTextField, TextFieldSize, TextFieldState, TextFieldVariant,
};

// Re-export menu components (specific items only)
pub use menus::MaterialMenu;

// Re-export navigation components
pub use navigation::{
    BreadcrumbItem, MaterialBreadcrumbs, MaterialTabBar, Tab, helpers as nav_helpers,
};

// Re-export widget components
pub use widgets::{ButtonSize, IconPosition, MaterialButton, MaterialButtonVariant};
