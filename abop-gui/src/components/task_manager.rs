//! Task manager component for handling task state and UI

use iced::widget::{button, column, container, progress_bar, row, text};
use iced::{Alignment, Element, Length, Theme};
use iced::widget::container::Style;
use uuid::Uuid;

use crate::design_tokens::spacing;
use crate::messages::Message;
use crate::state::{TaskInfo, TaskType, UiState};
use crate::styling::material::MaterialTokens;
use crate::theme::ThemeMode;
use crate::styling::material::components::{buttons, containers};
use crate::styling::material::components::widgets::material_button;

/// Task manager component for handling task state and UI
pub struct TaskManager;

impl TaskManager {
    /// Creates a new task
    pub fn create_task(task_type: TaskType) -> TaskInfo {
        TaskInfo {
            id: Uuid::new_v4().to_string(),
            task_type,
            progress: 0.0,
            status: "Starting...".to_string(),
            is_running: true,
            is_completed: false,
            error: None,
            start_time: chrono::Local::now(),
            end_time: None,
        }
    }

    /// Renders the task progress UI
    pub fn view(state: &UiState) -> Element<Message> {
        let content = if let Some(task) = &state.active_task {
            column![
                text("Active Task").size(20),
                text(&task.task_type.to_string()).size(16),
                progress_bar(0.0..=1.0, task.progress)
                    .width(Length::Fill),
                row![
                    button("Cancel")
                        .on_press(Message::CancelTask)
                        .style(buttons::button_style(&state.theme_mode)),
                ]
                .spacing(10)
                .padding(10),
            ]
            .padding(20)
            .style(|theme| containers::container_style(theme, state.theme_mode))
        } else {
            column![]
        };

        let history = if state.recent_tasks.is_empty() {
            column![]
        } else {
            column![
                text("Recent Tasks").size(20),
                column(
                    state.recent_tasks
                        .iter()
                        .map(|task| {
                            row![
                                text(&task.task_type.to_string()).size(16),
                                text(format!("{:.0}%", task.progress * 100.0)).size(14),
                            ]
                            .spacing(10)
                            .padding(5)
                        })
                        .collect::<Vec<_>>(),
                )
                .spacing(5),
            ]
            .padding(20)
            .style(|theme| containers::container_style(theme, state.theme_mode))
        };

        column![content, history].spacing(20).into()
    }
} 