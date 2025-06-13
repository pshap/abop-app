//! Unified main toolbar component
//!
//! This component creates a single, streamlined toolbar that combines navigation
//! and library actions into one cohesive interface.

use iced::widget::{Space, container, row, text};
use iced::{Alignment, Element, Length};
use std::path::Path;

use crate::components::buttons::variants::ButtonSize;
use crate::components::buttons::{self, ButtonVariant};
use crate::messages::Message;
use crate::state::DirectoryInfo;
use crate::styling::material::MaterialTokens;

/// Unified main toolbar component
///
/// Combines navigation tabs and library actions into a single, compact toolbar
/// following Material Design 3 guidelines for app bars and toolbars.
pub struct MainToolbar;

impl MainToolbar {
    /// Renders the unified main toolbar
    ///
    /// # Arguments
    /// * `recent_dirs` - List of recently used directories
    /// * `current_path` - The current library path
    /// * `material_tokens` - Material Design 3 tokens for styling
    #[must_use]
    pub fn view<'a>(
        _recent_dirs: &[DirectoryInfo],
        current_path: &Path,
        material_tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        // FINAL: Organize toolbar with logical grouping - directory controls on left, settings on right
        
        log::warn!("🔧 TOOLBAR DEBUG: Creating final organized toolbar layout");
        
        // Create folder button with proper message
        let folder_button = buttons::button(material_tokens)
            .icon_only("folder-open", ButtonSize::Medium)
            .variant(ButtonVariant::FilledTonal)
            .on_press(Message::command(crate::messages::Command::BrowseDirectory))
            .build()
            .unwrap_or_else(|_| text("📁").size(16).into());

        // Create scan button with proper message
        let scan_button = buttons::button(material_tokens)
            .label("Scan")
            .variant(ButtonVariant::Filled)
            .on_press(if !current_path.as_os_str().is_empty() {
                Message::command(crate::messages::Command::ScanLibrary {
                    library_path: current_path.to_path_buf(),
                })
            } else {
                Message::command(crate::messages::Command::BrowseDirectory) // Fallback if no path
            })
            .build()
            .unwrap_or_else(|_| text("Scan").size(14).into());

        // Create settings button
        let settings_button = buttons::button(material_tokens)
            .icon_only("gear", ButtonSize::Medium)
            .variant(ButtonVariant::FilledTonal)
            .on_press(Message::ShowSettings)
            .build()
            .unwrap_or_else(|_| text("⚙").size(16).into());

        // Use actual path data
        let path_display = if current_path.as_os_str().is_empty() {
            "No directory selected".to_string()
        } else {
            // Show just the folder name for compactness
            current_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Invalid path")
                .to_string()
        };
        
        // Create a row with logical grouping: [App Title] [Folder] [Scan] [Path] ... [Settings]
        let toolbar_row = row![
            // App branding
            text("ABOP")
                .size(16)
                .width(Length::Fixed(60.0)),
            
            // Directory controls group
            folder_button,
            scan_button,
            text(path_display)
                .size(12)
                .width(Length::Fill), // Path expands to fill available space
            
            // Flexible spacer to push settings to the right
            Space::with_width(Length::Fill),
            
            // Settings
            settings_button,
        ]
        .spacing(material_tokens.spacing().sm) // Use proper Material spacing
        .align_y(Alignment::Center);

        log::warn!("🔧 TOOLBAR DEBUG: Created final organized toolbar layout");

        // Wrap in container with proper height and padding using Material tokens
        container(toolbar_row)
            .height(Length::Fixed(material_tokens.sizing().toolbar_height))
            .width(Length::Fill)
            .padding([0.0, material_tokens.spacing().md])
            .align_y(Alignment::Center)
            .into()
    }
}
