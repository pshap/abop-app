//! Unified main toolbar component
//!
//! This component creates a single, streamlined toolbar that combines navigation
//! and library actions into one cohesive interface.

use iced::widget::{Space, container, row, text};
use iced::{Alignment, Element, Length};
use std::path::Path;

use crate::components::common::{create_button, filled_icon_button_semantic};
use crate::components::icons::icon_names;
use crate::messages::{Command, Message};
use crate::state::DirectoryInfo;
use crate::styling::material::MaterialTokens;
use crate::styling::material::components::widgets::{ButtonSize, MaterialButtonVariant};

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
        recent_dirs: &[DirectoryInfo],
        current_path: &Path,
        material_tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        // Debug: Log the current path being passed to MainToolbar
        log::info!(
            "MainToolbar::view: Received current_path: {}",
            current_path.display()
        );

        let mut toolbar = row![]
            .spacing(material_tokens.spacing().xs) // Use extra small spacing between toolbar items
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill);

        // App title/logo - more professional styling
        toolbar = toolbar.push(
            container(text("ABOP").size(16))
                .width(Length::Fixed(material_tokens.sizing().app_title_width))
                .align_x(iced::alignment::Horizontal::Left)
                .center_y(Length::Fill),
        );

        // Directory controls section
        let folder_button = filled_icon_button_semantic(
            icon_names::FOLDER_OPEN,
            ButtonSize::Medium,
            Message::ExecuteCommand(Command::BrowseDirectory),
            material_tokens,
        );

        toolbar = toolbar.push(folder_button);

        // Current path display - moved next to folder button
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

        // Add the path with minimal spacing next to folder button
        toolbar = toolbar.push(
            container(
                text(path_display)
                    .size(material_tokens.typography().label_small.size)
                    .width(Length::Shrink),
            )
            .padding([0.0, 8.0])
            .height(Length::Fill)
            .center_y(Length::Fill),
        );

        // Scan button with text
        let scan_button = create_button(
            "Scan",
            MaterialButtonVariant::Filled,
            Message::ExecuteCommand(Command::ScanLibrary {
                library_path: current_path.to_path_buf(),
            }),
            material_tokens,
        );

        // Debug: Log the path being used for the scan command
        log::warn!(
            "🔍 SCAN BUTTON CREATED: Will scan {}",
            current_path.display()
        );

        toolbar = toolbar.push(scan_button);

        // Recent directories dropdown if available
        if !recent_dirs.is_empty() {
            let recent_button = filled_icon_button_semantic(
                icon_names::DOWNLOAD,
                ButtonSize::Medium,
                Message::ShowSettings, // TODO: Replace with proper dropdown
                material_tokens,
            );

            toolbar = toolbar.push(recent_button);
        }

        // Add a flexible spacer to push settings button to the right
        toolbar = toolbar.push(Space::with_width(Length::Fill));

        // Settings button with icon - using filled variant
        let settings_button = filled_icon_button_semantic(
            "gear",
            ButtonSize::Medium,
            Message::ShowSettings,
            material_tokens,
        );

        // Add padding and center the button
        toolbar = toolbar.push(container(settings_button).padding(4).center_y(Length::Fill));

        // Wrap toolbar in container with unified toolbar height
        container(
            container(toolbar)
                .width(Length::Fill)
                .center_y(Length::Fill),
        )
        .height(Length::Fixed(material_tokens.sizing().toolbar_height))
        .width(Length::Fill)
        .padding([0.0, material_tokens.spacing().md])
        .into()
    }
}
