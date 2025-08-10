//! UI-specific state management
//!
//! This module handles pure UI concerns like theme, dialogs, and rendering flags.

use crate::styling::material::MaterialTokens;
use crate::theme::ThemeMode;

/// UI-specific state (theme, dialogs, rendering)
#[derive(Debug, Clone)]
pub struct UiState {
    /// Current theme mode for the GUI (light or dark)
    pub theme_mode: ThemeMode,
    /// Material Design tokens for UI styling
    pub material_tokens: MaterialTokens,
    /// Whether the settings dialog is currently open
    pub settings_open: bool,
    /// Whether the recent directories dropdown is currently open
    pub recent_directories_open: bool,
    /// Whether task history dialog is open
    pub show_task_history: bool,
    /// Flag to force a UI redraw when state changes
    pub needs_redraw: bool,
}

impl UiState {
    /// Create new UI state with specified theme
    #[must_use]
    pub fn with_theme(theme_mode: ThemeMode) -> Self {
        let material_tokens = match theme_mode {
            ThemeMode::Dark | ThemeMode::System | ThemeMode::MaterialDark => MaterialTokens::dark(),
            ThemeMode::Light | ThemeMode::MaterialLight => MaterialTokens::light(),
            ThemeMode::MaterialDynamic => {
                // Use purple seed color for dynamic Material Design theme
                let seed_color = iced::Color::from_rgb(0.4, 0.2, 0.8);
                MaterialTokens::from_seed_color(seed_color, true)
            }
        };

        Self {
            theme_mode,
            material_tokens,
            settings_open: false,
            recent_directories_open: false,
            show_task_history: false,
            needs_redraw: false,
        }
    }

    /// Update the theme mode and regenerate material tokens
    pub fn set_theme_mode(&mut self, theme_mode: ThemeMode) {
        if self.theme_mode != theme_mode {
            self.theme_mode = theme_mode;
            self.material_tokens = match theme_mode {
                ThemeMode::Dark | ThemeMode::System | ThemeMode::MaterialDark => MaterialTokens::dark(),
                ThemeMode::Light | ThemeMode::MaterialLight => MaterialTokens::light(),
                ThemeMode::MaterialDynamic => {
                    // Use purple seed color for dynamic Material Design theme
                    let seed_color = iced::Color::from_rgb(0.4, 0.2, 0.8);
                    MaterialTokens::from_seed_color(seed_color, true)
                }
            };
            self.needs_redraw = true;
        }
    }

    /// Update theme with a seed color for dynamic Material Design themes
    pub fn set_seed_color(&mut self, seed: iced::Color, is_dark: bool) {
        self.theme_mode = ThemeMode::MaterialDynamic;
        self.material_tokens = MaterialTokens::from_seed_color(seed, is_dark);
        self.needs_redraw = true;
    }

    /// Open the settings dialog
    pub fn open_settings(&mut self) {
        if !self.settings_open {
            self.settings_open = true;
            self.needs_redraw = true;
        }
    }

    /// Close the settings dialog
    pub fn close_settings(&mut self) {
        if self.settings_open {
            self.settings_open = false;
            self.needs_redraw = true;
        }
    }

    /// Toggle the settings dialog
    pub fn toggle_settings(&mut self) {
        self.settings_open = !self.settings_open;
        self.needs_redraw = true;
    }

    /// Open the recent directories dropdown
    pub fn open_recent_directories(&mut self) {
        if !self.recent_directories_open {
            self.recent_directories_open = true;
            self.needs_redraw = true;
        }
    }

    /// Close the recent directories dropdown
    pub fn close_recent_directories(&mut self) {
        if self.recent_directories_open {
            self.recent_directories_open = false;
            self.needs_redraw = true;
        }
    }

    /// Toggle the recent directories dropdown
    pub fn toggle_recent_directories(&mut self) {
        self.recent_directories_open = !self.recent_directories_open;
        self.needs_redraw = true;
    }

    /// Show task history dialog
    pub fn show_task_history(&mut self) {
        if !self.show_task_history {
            self.show_task_history = true;
            self.needs_redraw = true;
        }
    }

    /// Hide task history dialog
    pub fn hide_task_history(&mut self) {
        if self.show_task_history {
            self.show_task_history = false;
            self.needs_redraw = true;
        }
    }

    /// Toggle task history dialog
    pub fn toggle_task_history(&mut self) {
        self.show_task_history = !self.show_task_history;
        self.needs_redraw = true;
    }

    /// Check if the UI state needs a redraw
    #[must_use]
    pub const fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    /// Clear the redraw flag (typically called after redraw is complete)
    pub fn clear_redraw_flag(&mut self) {
        self.needs_redraw = false;
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self::with_theme(ThemeMode::Dark)
    }
}