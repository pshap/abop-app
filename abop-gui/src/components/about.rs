// About view component

use iced::Element;
use iced::Length;
use iced::widget::{column, container, text};

use crate::messages::Message;
use crate::styling::container::LayoutContainerStyles;
use crate::styling::material::MaterialTokens;
use crate::theme::ThemeMode;

/// About dialog and application information display component
///
/// This component renders the About dialog, showing application name, version,
/// and credits. It is used in the About view and accessible from the navigation bar.
///
/// # Examples
/// ```
/// use abop_gui::components::about::AboutView;
/// use abop_gui::theme::ThemeMode;
///
/// let theme_mode = ThemeMode::Light;
/// let about = AboutView::view(theme_mode);
/// ```
pub struct AboutView;

impl AboutView {
    /// Renders the About dialog view
    ///
    /// # Arguments
    /// * `theme` - The current theme mode for styling
    ///
    /// # Returns
    /// An Iced `Element` representing the About dialog UI
    #[must_use]
    pub fn view<'a>(theme: ThemeMode) -> Element<'a, Message> {
        let tokens = MaterialTokens::default();
        let content = column![
            text("About ABOP").size(24),
            text("Audiobook Organizer & Processor").size(18),
            text("Built with Rust and Iced").size(14),
            text("Version 1.0.0").size(14),
            text("© 2024 ABOP Team").size(12),
        ]
        .spacing(tokens.spacing.md);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(LayoutContainerStyles::content(theme))
            .padding(tokens.spacing.md)
            .into()
    }
}
