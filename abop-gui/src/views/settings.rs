//! Enhanced Settings view module with improved styling
//!
//! This module provides an enhanced settings view with consistent styling
//! and improved user interaction patterns.

use iced::widget::{Space, column, container, row, text};
use iced::{Element, Length};

use crate::components::common::create_button;
use crate::messages::Message;
use crate::state::UiState;
use crate::styling::container::dialog::DialogContainerStyles;
use crate::styling::material::components::widgets::MaterialButtonVariant;

// Import Material Design 3 selection components
use crate::styling::material::MaterialColors;
use crate::styling::material::components::selection::Switch;
use crate::styling::material::components::selection::builder::ComponentBuilder;
use crate::styling::material::components::selection::common::{ComponentSize, SwitchState};

/// Creates the enhanced settings view with Material Design 3 selection components
#[must_use]
pub fn settings_view(state: &UiState) -> Element<'_, Message> {
    // Create switches for each setting using the Material Design 3 selection components
    let theme_switch = create_theme_switch(state);
    let auto_save_switch = create_auto_save_switch(state);
    let scan_subdirs_switch = create_scan_subdirs_switch(state);

    // Create the settings content with proper spacing
    let settings_content = column![
        text("Application Settings").size(20),
        // Theme Setting
        row![
            column![
                text("Theme").size(16),
                text("Switch between light and dark theme").size(12)
            ]
            .width(Length::Fill),
            theme_switch
        ]
        .spacing(state.material_tokens.spacing().md)
        .align_y(iced::Alignment::Center),
        // Auto-save Library Setting
        row![
            column![
                text("Auto-save Library").size(16),
                text("Automatically save library changes").size(12)
            ]
            .width(Length::Fill),
            auto_save_switch
        ]
        .spacing(state.material_tokens.spacing().md)
        .align_y(iced::Alignment::Center),
        // Scan Subdirectories Setting
        row![
            column![
                text("Scan Subdirectories").size(16),
                text("Include subdirectories when scanning for audiobooks").size(12)
            ]
            .width(Length::Fill),
            scan_subdirs_switch
        ]
        .spacing(state.material_tokens.spacing().md)
        .align_y(iced::Alignment::Center),
    ]
    .spacing(state.material_tokens.spacing().lg)
    .padding(state.material_tokens.spacing().lg);    // Create the settings modal container with proper styling
    container(
        column![
            settings_content,
            // Close button row
            row![
                Space::new(Length::Fill, 0),
                create_button(
                    "Close",
                    MaterialButtonVariant::Filled,
                    Message::CloseSettings,
                    &state.material_tokens,
                )
            ]
        ]
        .spacing(state.material_tokens.spacing().md),
    )
    .width(Length::Fixed(400.0))
    .style(DialogContainerStyles::modal(state.theme_mode))
    .into()
}

/// Helper function to create MaterialColors based on theme
#[allow(dead_code)]
fn get_material_colors(is_dark: bool) -> MaterialColors {
    if is_dark {
        MaterialColors::dark_default()
    } else {
        MaterialColors::light_default()
    }
}

/// Creates a switch for theme toggling using Material Design 3 Switch component
fn create_theme_switch(state: &UiState) -> Element<'_, Message> {
    let is_dark = matches!(state.theme_mode, crate::theme::ThemeMode::Dark);
    let switch_state = if is_dark {
        SwitchState::On
    } else {
        SwitchState::Off
    };
    // Create Material Design 3 Switch component
    let md3_switch = Switch::builder(switch_state)
        .label("Dark Theme")
        .size(ComponentSize::Medium)
        .build()
        .unwrap_or_else(|_| {
            // Fallback to off state if build fails
            Switch::off().build().expect("Default switch should build")
        }); // Use static MaterialColors to solve lifetime issues
    if is_dark {
        static DARK_COLORS: std::sync::LazyLock<crate::styling::material::MaterialColors> =
            std::sync::LazyLock::new(crate::styling::material::MaterialColors::dark_default);
        md3_switch.view(move |_state| Message::ToggleTheme, &DARK_COLORS)
    } else {
        static LIGHT_COLORS: std::sync::LazyLock<crate::styling::material::MaterialColors> =
            std::sync::LazyLock::new(crate::styling::material::MaterialColors::light_default);
        md3_switch.view(move |_state| Message::ToggleTheme, &LIGHT_COLORS)
    }
}

/// Creates a switch for auto-save library setting using Material Design 3 Switch component
fn create_auto_save_switch(state: &UiState) -> Element<'_, Message> {
    let is_dark = matches!(state.theme_mode, crate::theme::ThemeMode::Dark);
    let switch_state = if state.auto_save_library {
        SwitchState::On
    } else {
        SwitchState::Off
    };
    // Create Material Design 3 Switch component
    let md3_switch = Switch::builder(switch_state)
        .label("Auto-save")
        .size(ComponentSize::Medium)
        .build()
        .unwrap_or_else(|_| {
            // Fallback to off state if build fails
            Switch::off().build().expect("Default switch should build")
        }); // Use static MaterialColors to solve lifetime issues
    if is_dark {
        static DARK_COLORS: std::sync::LazyLock<crate::styling::material::MaterialColors> =
            std::sync::LazyLock::new(crate::styling::material::MaterialColors::dark_default);
        md3_switch.view(move |_state| Message::ToggleAutoSaveLibrary, &DARK_COLORS)
    } else {
        static LIGHT_COLORS: std::sync::LazyLock<crate::styling::material::MaterialColors> =
            std::sync::LazyLock::new(crate::styling::material::MaterialColors::light_default);
        md3_switch.view(move |_state| Message::ToggleAutoSaveLibrary, &LIGHT_COLORS)
    }
}

/// Creates a switch for scan subdirectories setting using Material Design 3 Switch component
fn create_scan_subdirs_switch(state: &UiState) -> Element<'_, Message> {
    let is_dark = matches!(state.theme_mode, crate::theme::ThemeMode::Dark);
    let switch_state = if state.scan_subdirectories {
        SwitchState::On
    } else {
        SwitchState::Off
    };
    // Create Material Design 3 Switch component
    let md3_switch = Switch::builder(switch_state)
        .label("Scan subdirectories")
        .size(ComponentSize::Medium)
        .build()
        .unwrap_or_else(|_| {
            // Fallback to off state if build fails
            Switch::off().build().expect("Default switch should build")
        }); // Use static MaterialColors to solve lifetime issues
    if is_dark {
        static DARK_COLORS: std::sync::LazyLock<crate::styling::material::MaterialColors> =
            std::sync::LazyLock::new(crate::styling::material::MaterialColors::dark_default);
        md3_switch.view(
            move |_state| Message::ToggleScanSubdirectories,
            &DARK_COLORS,
        )
    } else {
        static LIGHT_COLORS: std::sync::LazyLock<crate::styling::material::MaterialColors> =
            std::sync::LazyLock::new(crate::styling::material::MaterialColors::light_default);
        md3_switch.view(
            move |_state| Message::ToggleScanSubdirectories,
            &LIGHT_COLORS,
        )
    }
}
