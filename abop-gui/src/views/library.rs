//! Library view module

use iced::Length;
use iced::widget::{column, container};

use crate::components::audio_toolbar::AudioToolbar;
use crate::components::status::{EnhancedStatusDisplayParams, StatusDisplay};
use crate::components::table_core::AudiobookTable;
use crate::messages::Message;
use crate::state::AppState;
use crate::styling::container::LayoutContainerStyles;
use crate::styling::material::{MaterialSurface, SurfaceVariant};

/// Creates the library management view with browsing, scanning, and audiobook list
#[must_use]
pub fn library_view(state: &AppState) -> iced::Element<'_, Message> {
    log::debug!("LIBRARY VIEW RENDER: {} audiobooks", state.library.audiobooks.len());
    
    let status_display = create_status_display(state);
    let table_content = create_audiobook_table(state);
    let footer = create_footer(state);
    
    let content_items = build_content_layout(state, status_display, table_content, footer);
    
    assemble_final_container(state, content_items)
}

/// Creates the status display component with current state information
fn create_status_display(state: &AppState) -> iced::Element<'_, Message> {
    StatusDisplay::enhanced_view(
        EnhancedStatusDisplayParams {
            scanning: state.library.scanning,
            scan_progress: state.library.scanner_progress.clone(),
            cached_scan_progress_text: None, // TODO: implement progress cache retrieval
            processing_audio: state.player.processing_audio,
            processing_progress: state.player.processing_progress,
            cached_processing_progress_text: None, // TODO: implement progress cache retrieval
            processing_status: state.player.processing_status.as_deref(),
            player_state: state.player.player_state.clone(),
            current_playing_file: state.player.current_playing_file.as_ref(),
            selected_count: state.library.selected_audiobooks.len(),
            total_count: state.library.audiobooks.len(),
            theme: state.ui.theme_mode,
        },
        &state.ui.material_tokens,
    )
}

/// Creates the audiobook table with styling
fn create_audiobook_table(state: &AppState) -> iced::Element<'_, Message> {
    log::debug!(
        "Creating table with {} audiobooks, {} selected",
        state.library.audiobooks.len(),
        state.library.selected_audiobooks.len()
    );

    let table_content = AudiobookTable::view(
        &state.library.audiobooks,
        &state.library.selected_audiobooks,
        &state.library.table_state,
        &state.ui.material_tokens,
    );

    log::debug!(
        "Creating table container with Material Design styling, {} audiobooks in table",
        state.library.audiobooks.len()
    );

    let debug_container = container(table_content)
        .width(Length::Fill)
        .height(Length::Fill);

    container(debug_container)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &iced::Theme| {
            MaterialSurface::new()
                .variant(SurfaceVariant::SurfaceContainerLow)
                .style(&state.ui.material_tokens)
        })
        .padding(iced::Padding {
            top: 8.0,
            right: 8.0,
            bottom: 8.0,
            left: 8.0,
        })
        .into()
}

/// Creates the footer component
fn create_footer(state: &AppState) -> iced::Element<'_, Message> {
    StatusDisplay::app_footer(state.library.audiobooks.len(), state.ui.theme_mode)
}

/// Builds the main content layout with optional toolbar
fn build_content_layout<'a>(
    state: &'a AppState,
    status_display: iced::Element<'a, Message>,
    table_content: iced::Element<'a, Message>,
    footer: iced::Element<'a, Message>,
) -> Vec<iced::Element<'a, Message>> {
    let mut content_items = vec![
        // Status display with fixed height
        container(status_display)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into(),
    ];

    // Only show audio toolbar when audiobooks are selected
    if !state.library.selected_audiobooks.is_empty() {
        let toolbar_element = create_audio_toolbar(state);
        content_items.push(toolbar_element);
    }

    // Add table content
    content_items.push(table_content);

    // Add footer
    content_items.push(footer);

    content_items
}

/// Creates the audio toolbar for selected audiobooks
fn create_audio_toolbar(state: &AppState) -> iced::Element<'_, Message> {
    log::debug!(
        "Creating audio toolbar for {} selected audiobooks, player state: {:?}", 
        state.library.selected_audiobooks.len(),
        state.player.player_state
    );
    
    let mut toolbar = AudioToolbar::new();
    toolbar.set_playing(matches!(
        state.player.player_state,
        abop_core::PlayerState::Playing
    ));

    container(toolbar.view(&state.ui.material_tokens))
        .width(Length::Fill)
        .height(Length::Fixed(state.ui.material_tokens.sizing().toolbar_height))
        .into()
}

/// Assembles the final container with styling
fn assemble_final_container<'a>(
    state: &'a AppState,
    content_items: Vec<iced::Element<'a, Message>>,
) -> iced::Element<'a, Message> {
    // Material Design 3: minimal vertical spacing between components
    const MD3_MINIMAL_SPACING: u16 = 4;
    
    let content = column(content_items)
        .spacing(MD3_MINIMAL_SPACING)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(state.ui.material_tokens.spacing.md);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(LayoutContainerStyles::content(state.ui.theme_mode))
        .padding(state.ui.material_tokens.spacing.sm)
        .into()
}
