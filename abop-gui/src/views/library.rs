//! Library view module

use iced::{
    widget::{column, container},
    Element, Length, Theme,
};

use crate::{
    messages::Message,
    state::GuiState,
    library::scanner::ScannerProgress,
    components::{
        status::StatusDisplay,
        audio_toolbar::AudioToolbar,
        audiobook_table::AudiobookTable,
    },
    styling::{
        material::MaterialSurface,
        layout::LayoutContainerStyles,
        material::components::surface::SurfaceVariant,
    },
};

pub struct LibraryView {
    scanner_progress: ScannerProgress,
}

impl Default for LibraryView {
    fn default() -> Self {
        Self {
            scanner_progress: ScannerProgress::default(),
        }
    }
}

impl LibraryView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view(&self, state: &GuiState) -> Element<Message> {
        // Use the enhanced StatusDisplay component with detailed progress information
        let status_display = StatusDisplay::enhanced_view(
            state.scanner_progress.progress.as_ref(),
            state.scanner_progress.state,
            &state.material_tokens,
        );

        // Use the AudiobookTable component with Material Design tokens
        let table_content = AudiobookTable::view(
            &state.core_state.data.audiobooks,
            &state.core_state.data.selected_audiobooks,
            &state.table_state,
            &state.material_tokens,
        );

        // Create content with proper constraints
        let content = column![
            // Status display with fixed height
            container(status_display)
                .width(Length::Fill)
                .height(Length::Shrink),
            // Audio toolbar with fixed height
            {
                let mut toolbar = AudioToolbar::new();
                toolbar.set_playing(matches!(
                    state.core_state.data.player_state,
                    abop_core::PlayerState::Playing
                ));

                container(toolbar.view(&state.material_tokens))
                    .width(Length::Fill)
                    .height(Length::Fixed(state.material_tokens.sizing().toolbar_height))
            },
            // Table content that will scroll
            container(table_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_theme: &Theme| {
                    MaterialSurface::new()
                        .variant(SurfaceVariant::SurfaceContainerLow)
                        .style(&state.material_tokens)
                })
                .padding(iced::Padding {
                    top: 8.0,
                    right: 8.0,
                    bottom: 8.0,
                    left: 8.0,
                }),
            // Footer
            StatusDisplay::app_footer(
                state.core_state.data.audiobooks.len(),
                state.theme_mode
            ),
        ]
        .spacing(state.material_tokens.spacing.xs)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(state.material_tokens.spacing.md);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(LayoutContainerStyles::content(state.theme_mode))
            .padding(state.material_tokens.spacing.sm)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ScanProgress(progress) => {
                self.scanner_progress.update(progress);
            }
            Message::ScanStateChanged(state) => {
                self.scanner_progress.set_state(state);
            }
            _ => {}
        }
    }
}

/// Creates the library management view with browsing, scanning, and audiobook list
#[must_use]
pub fn library_view(state: &GuiState) -> iced::Element<Message> {
    println!(
        "=== LIBRARY VIEW RENDER: {} audiobooks ===",
        state.audiobooks.len()
    );

    // Use the enhanced StatusDisplay component with detailed progress information
    let status_display = StatusDisplay::enhanced_view(
        EnhancedStatusDisplayParams {
            scanning: state.scanning,
            scan_progress: state.enhanced_scan_progress.clone(),
            processing_audio: state.processing_audio,
            processing_progress: state.processing_progress,
            processing_status: state.processing_status.as_deref(),
            player_state: state.player_state.clone(),
            current_playing_file: state.current_playing_file.as_ref(),
            selected_count: state.selected_audiobooks.len(),
            total_count: state.audiobooks.len(),
            theme: state.theme_mode,
        },
        &state.material_tokens,
    );

    log::debug!(
        "Creating table with {} audiobooks, {} selected",
        state.audiobooks.len(),
        state.selected_audiobooks.len()
    );

    // Use the AudiobookTable component with Material Design tokens
    let table_content = AudiobookTable::view(
        &state.audiobooks,
        &state.selected_audiobooks,
        &state.table_state,
        &state.material_tokens,
    );

    println!(
        "=== TABLE CONTENT CREATED: {} audiobooks ===",
        state.audiobooks.len()
    );
    // Combine components into the library view with proper space allocation
    let footer = StatusDisplay::app_footer(state.audiobooks.len(), state.theme_mode);

    // Create content without redundant toolbar (now integrated into main toolbar)
    // Create content with proper constraints
    let content = column![
        // Status display with fixed height
        container(status_display)
            .width(Length::Fill)
            .height(Length::Shrink),
        // Audio toolbar with fixed height
        {
            let mut toolbar = AudioToolbar::new();
            toolbar.set_playing(matches!(
                state.player_state,
                abop_core::PlayerState::Playing
            ));

            container(toolbar.view(&state.material_tokens))
                .width(Length::Fill)
                .height(Length::Fixed(state.material_tokens.sizing().toolbar_height)) // Use unified toolbar height
        },
        // Table content that will scroll - wrapped in a fixed height container
        {
            println!("=== CREATING TABLE CONTAINER ===");

            // Debug container with border and background
            let debug_container = container(table_content)
                .width(Length::Fill)
                .height(Length::Fill);

            container(debug_container)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_theme: &iced::Theme| {
                    MaterialSurface::new()
                        .variant(SurfaceVariant::SurfaceContainerLow)
                        .style(&state.material_tokens)
                })
                .padding(iced::Padding {
                    top: 8.0,
                    right: 8.0,
                    bottom: 8.0,
                    left: 8.0,
                })
        },
        // Footer with fixed height (no need for additional container)
        footer,
        self.scanner_progress.view().map(Message::Scanner),
    ]
    .spacing(state.material_tokens.spacing.xs) // Reduced from SM (8px) to XS (4px)
    .width(Length::Fill)
    .height(Length::Fill) // Fill available height
    .padding(state.material_tokens.spacing.md); // Add some padding around the content

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(LayoutContainerStyles::content(state.theme_mode))
        .padding(state.material_tokens.spacing.sm) // Reduced from MD (16px) to SM (8px)
        .into()
}
