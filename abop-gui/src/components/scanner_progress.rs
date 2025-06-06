use iced::widget::{button, column, container, progress_bar, row, text};
use iced::{Alignment, Element, Length, Theme};
use crate::core::scanner::{ScannerState, ScanProgress};

#[derive(Debug, Clone)]
pub struct ScannerProgress {
    state: ScannerState,
    progress: Option<ScanProgress>,
}

impl ScannerProgress {
    pub fn new(state: ScannerState, progress: Option<ScanProgress>) -> Self {
        Self { state, progress }
    }

    pub fn view(&self) -> Element<Message> {
        let progress_bar = if let Some(progress) = &self.progress {
            let progress_value = if progress.total_files > 0 {
                progress.files_processed as f32 / progress.total_files as f32
            } else {
                0.0
            };

            progress_bar(0.0..=1.0, progress_value)
                .width(Length::Fill)
                .height(Length::Fixed(20.0))
        } else {
            progress_bar(0.0..=1.0, 0.0)
                .width(Length::Fill)
                .height(Length::Fixed(20.0))
        };

        let status_text = match self.state {
            ScannerState::Idle => "Ready to scan",
            ScannerState::Scanning => "Scanning...",
            ScannerState::Paused => "Scan paused",
            ScannerState::Completed => "Scan completed",
            ScannerState::Error(ref msg) => msg,
        };

        let status = text(status_text).size(16);

        let progress_text = if let Some(progress) = &self.progress {
            format!(
                "Processed {} of {} files",
                progress.files_processed, progress.total_files
            )
        } else {
            "No progress information".to_string()
        };

        let progress_info = text(progress_text).size(14);

        let controls = row![
            button("Pause")
                .on_press(Message::PauseScan)
                .style(if self.state == ScannerState::Scanning {
                    Theme::Primary
                } else {
                    Theme::Secondary
                }),
            button("Resume")
                .on_press(Message::ResumeScan)
                .style(if self.state == ScannerState::Paused {
                    Theme::Primary
                } else {
                    Theme::Secondary
                }),
            button("Cancel")
                .on_press(Message::CancelScan)
                .style(Theme::Destructive),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        column![
            status,
            progress_bar,
            progress_info,
            controls,
        ]
        .spacing(10)
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    PauseScan,
    ResumeScan,
    CancelScan,
} 