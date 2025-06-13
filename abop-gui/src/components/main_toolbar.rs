//! Unified main toolbar component
//!
//! This component creates a single, streamlined toolbar that combines navigation
//! and library actions into one cohesive interface following Material Design 3
//! guidelines for app bars and toolbars.

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
/// Provides a comprehensive toolbar with:
/// - App branding (ABOP title)
/// - Directory controls (folder browser, scan button, path display)
/// - Settings access
///
/// Layout follows Material Design 3 principles with proper spacing and alignment.
pub struct MainToolbar;

impl MainToolbar {
    /// Renders the unified main toolbar
    ///
    /// The toolbar is organized with logical grouping:
    /// - Left: App title, folder button, scan button, current path
    /// - Right: Settings button (pushed right with flexible spacing)
    ///
    /// # Arguments
    /// * `_recent_dirs` - List of recently used directories (reserved for future dropdown)
    /// * `current_path` - The current library path to display and scan
    /// * `material_tokens` - Material Design 3 tokens for consistent styling
    ///
    /// # Returns
    /// A properly styled toolbar element that handles directory selection and scanning
    #[must_use]
    pub fn view<'a>(
        _recent_dirs: &[DirectoryInfo],
        current_path: &Path,
        material_tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        // === Button Creation ===

        // Folder browser button - opens directory selection dialog
        let folder_button = buttons::button(material_tokens)
            .icon_only("folder-open", ButtonSize::Medium)
            .variant(ButtonVariant::FilledTonal)
            .on_press(Message::command(crate::messages::Command::BrowseDirectory))
            .build()
            .unwrap_or_else(|_| text("📁").size(16).into()); // Fallback to emoji if icon fails

        // Scan button - initiates library scanning of current path
        let scan_button = buttons::button(material_tokens)
            .label("Scan")
            .variant(ButtonVariant::Filled)
            .on_press(if !current_path.as_os_str().is_empty() {
                Message::command(crate::messages::Command::ScanLibrary {
                    library_path: current_path.to_path_buf(),
                })
            } else {
                Message::command(crate::messages::Command::BrowseDirectory) // Fallback if no path selected
            })
            .build()
            .unwrap_or_else(|_| text("Scan").size(14).into());

        // Settings button - opens application settings
        let settings_button = buttons::button(material_tokens)
            .icon_only("gear", ButtonSize::Medium)
            .variant(ButtonVariant::FilledTonal)
            .on_press(Message::ShowSettings)
            .build()
            .unwrap_or_else(|_| text("⚙").size(16).into());

        // === Path Display ===

        // Format current path for display (show folder name or placeholder)
        let path_display = if current_path.as_os_str().is_empty() {
            "No directory selected".to_string()
        } else {
            // Show just the folder name for compactness in toolbar
            current_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Invalid path")
                .to_string()
        };

        // === Toolbar Layout ===

        // Organize toolbar with logical grouping:
        // [App Title] [Folder] [Scan] [Path Display] ... [Settings]
        let toolbar_row = row![
            // App branding - fixed width for consistent layout
            text("ABOP").size(16).width(Length::Fixed(60.0)),
            // Directory controls group - logically related actions
            folder_button,
            scan_button,
            text(path_display)
                .size(material_tokens.typography().label_small.size)
                .width(Length::Fill), // Expands to fill available space
            // Flexible spacer - pushes settings button to the right
            Space::with_width(Length::Fill),
            // Settings access - positioned on the right for easy access
            settings_button,
        ]
        .spacing(material_tokens.spacing().sm) // Use Material Design spacing tokens
        .align_y(Alignment::Center); // Center all items vertically

        // === Container Wrapper ===

        // Wrap toolbar in container with proper Material Design height and spacing
        container(toolbar_row)
            .height(Length::Fixed(material_tokens.sizing().toolbar_height))
            .width(Length::Fill)
            .padding([0.0, material_tokens.spacing().md]) // Horizontal padding only
            .align_y(Alignment::Center) // Center content within container height
            .into()
    }
}
