//! Enhanced Settings view module with Material Design 3 selection components
//!
//! This module provides an enhanced settings view that uses Material Design 3 
//! selection components (switches, checkboxes) for a modern, interactive experience.

use iced::widget::{button, column, container, row, text, Space};
use iced::{Element, Length, Padding};

use crate::messages::Message;
use crate::state::UiState;
use crate::styling::container::dialog::DialogContainerStyles;

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
            ].width(Length::Fill),
            theme_switch        ].spacing(state.material_tokens.spacing().md)
        .align_y(iced::Alignment::Center),
        
        // Auto-save Library Setting
        row![
            column![
                text("Auto-save Library").size(16),
                text("Automatically save library changes").size(12)
            ].width(Length::Fill),
            auto_save_switch
        ].spacing(state.material_tokens.spacing().md)
        .align_y(iced::Alignment::Center),
        
        // Scan Subdirectories Setting
        row![
            column![
                text("Scan Subdirectories").size(16),
                text("Include subdirectories when scanning for audiobooks").size(12)
            ].width(Length::Fill),
            scan_subdirs_switch
        ].spacing(state.material_tokens.spacing().md)
        .align_y(iced::Alignment::Center),
    ]    .spacing(state.material_tokens.spacing().lg)
    .padding(state.material_tokens.spacing().lg);

    // Create the settings modal container with proper styling
    container(
        column![
            settings_content,
              // Close button row
            row![
                Space::new(Length::Fill, 0),
                button("Close")
                    .on_press(Message::CloseSettings)
                    .padding([8, 16])
            ]
        ]
        .spacing(state.material_tokens.spacing().md)
    )
    .width(Length::Fixed(400.0))
    .style(DialogContainerStyles::modal(state.theme_mode))
    .into()
}

/// Creates a switch for theme toggling
fn create_theme_switch(state: &UiState) -> Element<'_, Message> {
    // Create a simple button that acts as a switch for now
    // TODO: Replace with actual Material Design 3 Switch component when widget integration is complete
    let is_dark = matches!(state.theme_mode, crate::theme::ThemeMode::Dark);
    let switch_text = if is_dark { "Dark" } else { "Light" };
    
    button(text(switch_text))
        .on_press(Message::ToggleTheme)
        .padding(Padding::from([8, 16]))
        .into()
}

/// Creates a switch for auto-save library setting
fn create_auto_save_switch(state: &UiState) -> Element<'_, Message> {
    // Create a simple button that acts as a switch for now
    let switch_text = if state.auto_save_library { "On" } else { "Off" };
    
    button(text(switch_text))
        .on_press(Message::ToggleAutoSaveLibrary)
        .padding(Padding::from([8, 16]))
        .into()
}

/// Creates a switch for scan subdirectories setting
fn create_scan_subdirs_switch(state: &UiState) -> Element<'_, Message> {
    // Create a simple button that acts as a switch for now
    let switch_text = if state.scan_subdirectories { "On" } else { "Off" };
    
    button(text(switch_text))
        .on_press(Message::ToggleScanSubdirectories)
        .padding(Padding::from([8, 16]))
        .into()
}
