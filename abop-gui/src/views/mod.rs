//! View modules for the application

pub mod about;
pub mod audio_processing;
pub mod library;
pub mod settings;

#[cfg(test)]
mod tests;

use iced::widget::{center, column, container, mouse_area, opaque, stack};
use iced::{Color, Element, Length};

use crate::components::main_toolbar::MainToolbar;
use crate::components::status_bar::StatusBar;
use crate::messages::Message;
use crate::state::AppState;

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
                            a: 0.55,
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

/// View function that renders the application UI based on current state and route
#[must_use]
pub fn view(state: &AppState, route: crate::router::Route) -> Element<'_, Message> {
    let tokens = &state.ui.material_tokens;

    // Route-specific content (avoid double rendering settings)
    let content = match route {
        crate::router::Route::Library => library_view(state),
        crate::router::Route::Player => audio_processing_view(state),
        crate::router::Route::Settings if !state.ui.settings_open => settings_view(state),
        crate::router::Route::Settings => {
            // Settings are handled as modal, show library instead to avoid empty content
            library_view(state)
        }
        crate::router::Route::About => about_view(state),
    };

    let main_content = container(column![
        MainToolbar::view(state),
        container(content).width(Length::Fill).height(Length::Fill),
        StatusBar::view(state),
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_theme| container::Style {
        background: Some(tokens.colors.surface.into()),
        text_color: Some(tokens.colors.on_surface),
        ..container::Style::default()
    });

    // Modal overlays
    if let Some(audiobook_id) = &state.ui.renaming_id {
        modal(
            main_content,
            crate::components::rename_dialog::view(
                audiobook_id.clone(),
                &state.ui.rename_input,
                &state.ui.material_tokens,
            ),
            Message::CancelRename,
        )
    } else if state.ui.settings_open {
        modal(main_content, settings_view(state), Message::CloseSettings)
    } else {
        main_content.into()
    }
}
