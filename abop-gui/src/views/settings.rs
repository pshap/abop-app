//! Enhanced Settings view module with improved styling
//!
//! This module provides an enhanced settings view with consistent styling
//! and improved user interaction patterns.

use iced::widget::{Space, column, container, row, text};
use iced::{Element, Length};

use crate::components::buttons;
use crate::components::buttons::builder::ButtonBuilder;
use crate::styling::material::components::selection::builder::CommonSelectionBuilder;
use crate::components::buttons::variants::ButtonVariant;
use crate::messages::Message;
use crate::state::UiState;
use crate::styling::container::dialog::DialogContainerStyles;

// Import Material Design 3 selection components
use crate::styling::material::components::selection::Switch;
use crate::styling::material::components::selection::common::{ComponentSize, SwitchState};

/// Standard width for settings dialogs
const SETTINGS_DIALOG_WIDTH: f32 = 400.0;

/// Creates the enhanced settings view with Material Design 3 selection components
#[must_use]
pub fn settings_view(state: &UiState) -> Element<'_, Message> {
    // Create switches for each setting using the Material Design 3 selection components
    let theme_switch = create_theme_switch(state);
    let auto_save_switch = create_auto_save_switch(state);
    let scan_subdirs_switch = create_scan_subdirs_switch(state);

    // Create the settings content with proper spacing
    let settings_content = column![
        text("Application Settings").size(state.material_tokens.typography().title_medium.size),
        // Theme Setting
        row![
            column![
                text("Theme").size(state.material_tokens.typography().label_large.size),
                text("Switch between light and dark theme")
                    .size(state.material_tokens.typography().body_small.size)
            ]
            .width(Length::Fill),
            theme_switch
        ]
        .spacing(state.material_tokens.spacing().md)
        .align_y(iced::Alignment::Center),
        // Auto-save Library Setting
        row![
            column![
                text("Auto-save Library").size(state.material_tokens.typography().label_large.size),
                text("Automatically save library changes")
                    .size(state.material_tokens.typography().body_small.size)
            ]
            .width(Length::Fill),
            auto_save_switch
        ]
        .spacing(state.material_tokens.spacing().md)
        .align_y(iced::Alignment::Center),
        // Scan Subdirectories Setting
        row![
            column![
                text("Scan Subdirectories")
                    .size(state.material_tokens.typography().label_large.size),
                text("Include subdirectories when scanning for audiobooks")
                    .size(state.material_tokens.typography().body_small.size)
            ]
            .width(Length::Fill),
            scan_subdirs_switch
        ]
        .spacing(state.material_tokens.spacing().md)
        .align_y(iced::Alignment::Center),
    ]
    .spacing(state.material_tokens.spacing().lg)
    .padding(state.material_tokens.spacing().lg); // Create the settings modal container with proper styling
    container(
        column![
            settings_content,
            // Close button row
            row![
                Space::new(Length::Fill, 0),
                buttons::create_button(
                    || ButtonBuilder::new(&state.material_tokens)
                        .label("Close")
                        .variant(ButtonVariant::Filled)
                        .on_press(Message::CloseSettings)
                        .build(),
                    "close settings",
                    Some("Close"),
                )
            ]
        ]
        .spacing(state.material_tokens.spacing().md),
    )
    .width(Length::Fixed(SETTINGS_DIALOG_WIDTH))
    .style(DialogContainerStyles::modal(state.theme_mode))
    .into()
}

/// Helper function to create MD3 switches with consistent styling
fn create_settings_switch<'a, ToggleHandler>(
    label: &'a str,
    state: &'a UiState,
    is_enabled: bool,
    on_toggle: ToggleHandler,
) -> Element<'a, Message>
where
    ToggleHandler: Fn(SwitchState) -> Message + 'static,
{
    let is_dark = matches!(state.theme_mode, crate::theme::ThemeMode::Dark);
    let switch_state = if is_enabled {
        SwitchState::On
    } else {
        SwitchState::Off
    };

    // Create Material Design 3 Switch component
    let md3_switch = Switch::builder(switch_state)
        .label(label)
        .size(ComponentSize::Medium)
        .build()
        .unwrap_or_else(|_| {
            // Fallback to a basic switch without customization
            // This should never fail as it uses minimal configuration
            Switch::off().build().unwrap_or_else(|_| {
                // Ultimate fallback - create a completely default switch
                Switch::default()
            })
        });

    // Use static MaterialColors to solve lifetime issues
    static DARK_COLORS: std::sync::LazyLock<crate::styling::material::MaterialColors> =
        std::sync::LazyLock::new(crate::styling::material::MaterialColors::dark_default);
    static LIGHT_COLORS: std::sync::LazyLock<crate::styling::material::MaterialColors> =
        std::sync::LazyLock::new(crate::styling::material::MaterialColors::light_default);

    let colors = if is_dark { &DARK_COLORS } else { &LIGHT_COLORS };
    md3_switch.view(on_toggle, colors)
}

/// Creates a switch for theme toggling using Material Design 3 Switch component
fn create_theme_switch(state: &UiState) -> Element<'_, Message> {
    let is_dark = matches!(state.theme_mode, crate::theme::ThemeMode::Dark);
    create_settings_switch("Dark Theme", state, is_dark, |_| Message::ToggleTheme)
}

/// Creates a switch for auto-save library setting using Material Design 3 Switch component
fn create_auto_save_switch(state: &UiState) -> Element<'_, Message> {
    create_settings_switch("Auto-save", state, state.auto_save_library, |_| {
        Message::ToggleAutoSaveLibrary
    })
}

/// Creates a switch for scan subdirectories setting using Material Design 3 Switch component
fn create_scan_subdirs_switch(state: &UiState) -> Element<'_, Message> {
    create_settings_switch(
        "Scan subdirectories",
        state,
        state.scan_subdirectories,
        |_| Message::ToggleScanSubdirectories,
    )
}
