//! View modules for the application

pub mod about;
pub mod audio_processing;
pub mod library;
pub mod settings;

use iced::widget::{center, column, container, mouse_area, opaque, stack};
use iced::{Color, Element};

use crate::components::main_toolbar::MainToolbar;
use crate::messages::Message;
use crate::state::UiState;

pub use about::about_view;
pub use audio_processing::audio_processing_view;
pub use library::library_view;
pub use settings::settings_view;

/// Creates a modal overlay with the given content over a base element
fn modal<'a>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message> {
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}

/// View function that renders the application UI based on current state
#[must_use]
pub fn view(state: &UiState) -> Element<'_, Message> {
    // Unified toolbar at the top combining navigation and actions
    let toolbar = MainToolbar::view(
        &state.recent_directories,
        &state.library_path,
        &state.material_tokens,
    );
    let content = library_view(state);

    let main_content = column![toolbar, content]
        .spacing(state.material_tokens.spacing().sm) // Reduced from LG (24px) to SM (8px)
        .padding(state.material_tokens.spacing().md); // Reduced from LG to MD (16px)    // If settings dialog is open, show it as a modal overlay
    if state.settings_open {
        modal(main_content, settings_view(state), Message::CloseSettings)
    } else {
        main_content.into()
    }
}
