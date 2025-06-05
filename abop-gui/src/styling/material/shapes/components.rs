//! Component types and shape recommendations for the Material Design 3 Shape System
//!
//! This module defines the different UI component types that have specific
//! corner radius recommendations in Material Design 3.

use super::core::ShapeSize;

/// Component types for shape recommendations
///
/// Defines the different UI component types that have specific
/// corner radius recommendations in Material Design 3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    /// Standard filled button
    Button,
    /// Outlined button variant
    OutlinedButton,
    /// Text-only button variant
    TextButton,
    /// Floating action button
    FloatingActionButton,
    /// Extended floating action button with text
    ExtendedFab,
    /// Standard elevated card
    Card,
    /// Elevated card variant
    ElevatedCard,
    /// Outlined card variant
    OutlinedCard,
    /// Standard chip component
    Chip,
    /// Filter chip variant
    FilterChip,
    /// Input chip variant
    InputChip,
    /// Standard text field
    TextField,
    /// Outlined text field variant
    OutlinedTextField,
    /// Filled text field variant
    FilledTextField,
    /// Context menu
    Menu,
    /// Tooltip component
    Tooltip,
    /// Modal dialog
    Dialog,
    /// Bottom sheet
    BottomSheet,
    /// Navigation drawer
    NavigationDrawer,
    /// Top app bar
    AppBar,
    /// Bottom navigation bar
    BottomNavigationBar,
    /// Navigation rail
    NavigationRail,
    /// Badge indicator
    Badge,
    /// User avatar
    Avatar,
    /// Toggle switch
    Switch,
    /// Checkbox input
    Checkbox,
    /// Radio button input
    RadioButton,
    /// Visual divider
    Divider,
    /// Progress indicator
    ProgressIndicator,
    /// Slider input
    Slider,
}

/// Component categories for grouping related components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentCategory {
    /// Button-like interactive elements
    Buttons,
    /// Card and container elements
    Containers,
    /// Input and form elements
    Inputs,
    /// Navigation elements
    Navigation,
    /// Feedback and status elements
    Feedback,
    /// Small indicator elements
    Indicators,
}

impl ComponentType {
    /// Get all component types
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Button,
            Self::OutlinedButton,
            Self::TextButton,
            Self::FloatingActionButton,
            Self::ExtendedFab,
            Self::Card,
            Self::ElevatedCard,
            Self::OutlinedCard,
            Self::Chip,
            Self::FilterChip,
            Self::InputChip,
            Self::TextField,
            Self::OutlinedTextField,
            Self::FilledTextField,
            Self::Menu,
            Self::Tooltip,
            Self::Dialog,
            Self::BottomSheet,
            Self::NavigationDrawer,
            Self::AppBar,
            Self::BottomNavigationBar,
            Self::NavigationRail,
            Self::Badge,
            Self::Avatar,
            Self::Switch,
            Self::Checkbox,
            Self::RadioButton,
            Self::Divider,
            Self::ProgressIndicator,
            Self::Slider,
        ]
    }

    /// Get the component category
    #[must_use]
    pub const fn category(&self) -> ComponentCategory {
        match self {
            Self::Button
            | Self::OutlinedButton
            | Self::TextButton
            | Self::FloatingActionButton
            | Self::ExtendedFab => ComponentCategory::Buttons,

            Self::Card
            | Self::ElevatedCard
            | Self::OutlinedCard
            | Self::Dialog
            | Self::BottomSheet
            | Self::Menu
            | Self::Tooltip => ComponentCategory::Containers,

            Self::TextField
            | Self::OutlinedTextField
            | Self::FilledTextField
            | Self::Chip
            | Self::FilterChip
            | Self::InputChip
            | Self::Switch
            | Self::Checkbox
            | Self::RadioButton
            | Self::Slider => ComponentCategory::Inputs,

            Self::NavigationDrawer
            | Self::AppBar
            | Self::BottomNavigationBar
            | Self::NavigationRail => ComponentCategory::Navigation,

            Self::ProgressIndicator | Self::Divider => ComponentCategory::Feedback,

            Self::Badge | Self::Avatar => ComponentCategory::Indicators,
        }
    }

    /// Get the component name as a string
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::OutlinedButton => "outlined-button",
            Self::TextButton => "text-button",
            Self::FloatingActionButton => "floating-action-button",
            Self::ExtendedFab => "extended-fab",
            Self::Card => "card",
            Self::ElevatedCard => "elevated-card",
            Self::OutlinedCard => "outlined-card",
            Self::Chip => "chip",
            Self::FilterChip => "filter-chip",
            Self::InputChip => "input-chip",
            Self::TextField => "text-field",
            Self::OutlinedTextField => "outlined-text-field",
            Self::FilledTextField => "filled-text-field",
            Self::Menu => "menu",
            Self::Tooltip => "tooltip",
            Self::Dialog => "dialog",
            Self::BottomSheet => "bottom-sheet",
            Self::NavigationDrawer => "navigation-drawer",
            Self::AppBar => "app-bar",
            Self::BottomNavigationBar => "bottom-navigation-bar",
            Self::NavigationRail => "navigation-rail",
            Self::Badge => "badge",
            Self::Avatar => "avatar",
            Self::Switch => "switch",
            Self::Checkbox => "checkbox",
            Self::RadioButton => "radio-button",
            Self::Divider => "divider",
            Self::ProgressIndicator => "progress-indicator",
            Self::Slider => "slider",
        }
    }

    /// Get the recommended shape size for this component type
    #[must_use]
    pub const fn recommended_shape(&self) -> ShapeSize {
        match self {
            Self::Button | Self::OutlinedButton | Self::TextButton => ShapeSize::Small,
            Self::FloatingActionButton | Self::ExtendedFab => ShapeSize::Large,
            Self::Card | Self::ElevatedCard | Self::OutlinedCard => ShapeSize::Medium,
            Self::Chip | Self::FilterChip | Self::InputChip => ShapeSize::Small,
            Self::TextField
            | Self::OutlinedTextField
            | Self::FilledTextField
            | Self::Menu
            | Self::Tooltip => ShapeSize::ExtraSmall,
            Self::Dialog | Self::BottomSheet => ShapeSize::ExtraLarge,
            Self::NavigationDrawer
            | Self::AppBar
            | Self::BottomNavigationBar
            | Self::NavigationRail
            | Self::Divider => ShapeSize::None,
            Self::Badge
            | Self::Avatar
            | Self::Switch
            | Self::RadioButton
            | Self::ProgressIndicator
            | Self::Slider => ShapeSize::Full,
            Self::Checkbox => ShapeSize::ExtraSmall,
        }
    }
}

impl ComponentCategory {
    /// Get all component categories
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Buttons,
            Self::Containers,
            Self::Inputs,
            Self::Navigation,
            Self::Feedback,
            Self::Indicators,
        ]
    }

    /// Get the category name as a string
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Buttons => "buttons",
            Self::Containers => "containers",
            Self::Inputs => "inputs",
            Self::Navigation => "navigation",
            Self::Feedback => "feedback",
            Self::Indicators => "indicators",
        }
    }

    /// Get description of the component category
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Buttons => "Button-like interactive elements",
            Self::Containers => "Card and container elements",
            Self::Inputs => "Input and form elements",
            Self::Navigation => "Navigation elements",
            Self::Feedback => "Feedback and status elements",
            Self::Indicators => "Small indicator elements",
        }
    }

    /// Get the default shape size for this category
    #[must_use]
    pub const fn default_shape(&self) -> ShapeSize {
        match self {
            Self::Buttons => ShapeSize::Small,
            Self::Containers => ShapeSize::Medium,
            Self::Inputs => ShapeSize::ExtraSmall,
            Self::Navigation => ShapeSize::None,
            Self::Feedback => ShapeSize::None,
            Self::Indicators => ShapeSize::Full,
        }
    }
}
