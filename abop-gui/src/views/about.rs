//! About view module

use iced::Length;
use iced::widget::container;

use crate::messages::Message;
use crate::state::AppState;
use crate::styling::container::LayoutContainerStyles;

/// Creates the about view
#[must_use]
pub fn about_view(state: &AppState) -> iced::Element<'_, Message> {
    // Use the AboutView component
    let about_content = crate::components::about::AboutView::view(state.ui.theme_mode);
    container(about_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(LayoutContainerStyles::content(state.ui.theme_mode))
        .padding(state.ui.material_tokens.spacing().md) // Use state.material_tokens.spacing().md where needed
        .into()
}
