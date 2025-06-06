//! View modules for the application

pub mod about;
pub mod audio_processing;
pub mod library;
pub mod settings;

use iced::widget::{center, column, container, mouse_area, opaque, stack};
use iced::{Color, Element, Length};
use iced::widget::container::Style;

use crate::components::main_toolbar::MainToolbar;
use crate::messages::Message;
use crate::state::UiState;
use crate::design_tokens::constants::spacing;
use abop_core::ViewType;
use crate::styling::material::components::containers;
use crate::components::task_manager::TaskManager;

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
pub fn view(state: &UiState) -> Element<Message> {
    let mut content = column![].spacing(spacing::MD);

    // Add main toolbar
    content = content.push(MainToolbar::view(
        &state.recent_directories,
        &state.library_path,
        &state.material_tokens,
    ));

    // Add task manager if there's an active task or task history is shown
    if state.active_task.is_some() || state.show_task_history {
        content = content.push(TaskManager::view(
            &state.active_task,
            &state.recent_tasks,
            state.show_task_history,
            state.theme_mode,
            &state.material_tokens,
        ));
    }    // Add main content
    content = content.push(match state.core_state.current_view {
        ViewType::Library => library_view(state),
        ViewType::AudioProcessing => audio_processing_view(state),
        ViewType::Settings => settings_view(state),
        ViewType::About => settings_view(state), // Temporary mapping until About view is implemented
    });

    // Add settings modal if open
    if state.settings_open {
        content = modal(
            content,
            settings_view(state),
            Message::CloseSettings,
        );
    }

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme| containers::container_style(theme, state.theme_mode))
        .into()
}
