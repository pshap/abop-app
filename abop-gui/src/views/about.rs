//! About view module

use iced::Length;
use iced::widget::container;

use crate::messages::Message;
use crate::state::UiState;
use crate::styling::container::LayoutContainerStyles;

/// Creates the about view
#[must_use]
pub fn about_view(state: &UiState) -> iced::Element<'_, Message> {
    // Use the AboutView component
    let about_content = crate::components::about::AboutView::view(state.theme_mode);
    container(about_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(LayoutContainerStyles::content(state.theme_mode))
        .padding(state.material_tokens.spacing().md) // Use state.material_tokens.spacing().md where needed
        .into()
}
