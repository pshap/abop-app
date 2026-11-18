// Table row component with Material Design 3 styling and selection support

use iced::widget::{button, checkbox, container, row, text};
use iced::{Background, Color, Element, Length, Padding};
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

use abop_core::models::Audiobook;

use crate::components::cover_art::CoverArt;
use crate::messages::Message;
use crate::styling::material::MaterialTokens;
use crate::styling::material::components::data;
use crate::utils::image_cache::ImageCache;

/// Height of each audiobook row
pub const ROW_HEIGHT: f32 = 56.0;

/// Inputs shared by every row of one table render
///
/// `'c` borrows per-render values; `'a` is the lifetime of the produced elements.
pub struct RowContext<'c, 'a> {
    /// Columns to render, in order
    pub columns: &'c [data::TableColumn],
    /// Ids of the selected audiobooks
    pub selected: &'c HashSet<String>,
    /// Table behavior (selection, striping)
    pub config: &'c data::DataTableConfig,
    /// File currently playing, highlighted in the table
    pub now_playing: Option<&'c Path>,
    /// Saved listening progress by audiobook id
    pub progress: &'c std::collections::HashMap<String, crate::state::ListeningProgress>,
    /// Design tokens
    pub tokens: &'a MaterialTokens,
    /// Cover thumbnail cache
    pub image_cache: &'a Arc<ImageCache>,
}

/// Component for creating table rows with Material Design styling
pub struct TableRow;

impl TableRow {
    /// Create a collection of row elements from audiobooks
    #[must_use]
    pub fn create_rows<'a>(
        audiobooks: &'a [Audiobook],
        context: &RowContext<'_, 'a>,
    ) -> Vec<Element<'a, Message>> {
        audiobooks
            .iter()
            .enumerate()
            .map(|(index, audiobook)| Self::create_single_row(audiobook, context, index))
            .collect()
    }
    /// Create a single row element with selection checkbox
    fn create_single_row<'a>(
        audiobook: &'a Audiobook,
        context: &RowContext<'_, 'a>,
        row_index: usize,
    ) -> Element<'a, Message> {
        let tokens = context.tokens;
        let config = context.config;
        let is_selected = context.selected.contains(&audiobook.id);
        let is_playing = context.now_playing == Some(audiobook.path.as_path());
        let is_striped = config.striped && row_index % 2 == 1;

        // Create enhanced selection checkbox with improved Material Design 3 styling
        let mut all_cells = vec![];

        let selection_checkbox = checkbox("", is_selected)
            .on_toggle(|_| Message::ToggleAudiobookSelection(audiobook.id.clone()))
            .size(20)
            .style(move |_theme, _status| {
                // Enhanced Material Design styling with better visual feedback
                iced::widget::checkbox::Style {
                    background: Background::Color(if is_selected {
                        tokens.colors.primary.base
                    } else {
                        Color::TRANSPARENT
                    }),
                    icon_color: if is_selected {
                        tokens.colors.primary.on_base
                    } else {
                        tokens.colors.primary.base
                    },
                    border: iced::Border {
                        color: if is_selected {
                            tokens.colors.primary.base
                        } else {
                            tokens.colors.outline
                        },
                        width: 2.0,
                        radius: 4.0.into(),
                    },
                    text_color: Some(tokens.colors.on_surface),
                }
            });

        let checkbox_cell = container(selection_checkbox)
            .width(Length::Fixed(48.0))
            .height(Length::Fill)
            .padding(Padding::from([8.0, 8.0]))
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center);

        all_cells.push(checkbox_cell.into());

        // Create data cells
        let data_cells: Vec<Element<'a, Message>> = context
            .columns
            .iter()
            .map(|column| {
                Self::create_cell(
                    audiobook,
                    column,
                    tokens,
                    context.image_cache,
                    is_playing,
                    context.progress.get(&audiobook.id),
                )
            })
            .collect();

        all_cells.extend(data_cells);

        // Add action cell (Rename button)
        let action_cell = Self::create_action_cell(audiobook, tokens);
        all_cells.push(action_cell);

        let row_content = row(all_cells);

        // Make the entire row clickable for selection (excluding checkbox area)
        let styled_row: Element<'a, Message> = if config.selectable {
            button(row_content)
                .style(move |theme, status| {
                    Self::row_button_style(
                        theme,
                        status,
                        tokens,
                        is_selected,
                        is_striped,
                        row_index,
                    )
                })
                .on_press(Message::RowClicked(audiobook.id.clone()))
                .width(Length::Fill)
                .padding(0)
                .into()
        } else {
            container(row_content)
                .style(data::MaterialDataTable::table_row(
                    tokens,
                    row_index,
                    is_selected,
                    is_striped,
                ))
                .width(Length::Fill)
                .into()
        };
        container(styled_row)
            .width(Length::Fill)
            .height(Length::Fixed(ROW_HEIGHT))
            .into()
    }

    /// Create action cell with buttons for operations like renaming
    fn create_action_cell<'a>(
        audiobook: &'a Audiobook,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        let rename_button = crate::components::ui::icon_button(
            tokens,
            crate::components::ui::ButtonKind::Ghost,
            "pen",
            "Rename",
            Some(Message::BeginRename(audiobook.id.clone())),
        );

        container(rename_button)
            .width(Length::Fixed(64.0))
            .height(Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .into()
    }

    /// Create individual cell content
    fn create_cell<'a>(
        audiobook: &'a Audiobook,
        column: &data::TableColumn,
        tokens: &'a MaterialTokens,
        image_cache: &'a Arc<ImageCache>,
        is_playing: bool,
        listened: Option<&crate::state::ListeningProgress>,
    ) -> Element<'a, Message> {
        // Handle cover art column specially
        if column.id == "cover_art" {
            let cover_art = CoverArt::from_data(
                audiobook.cover_art.as_deref(),
                crate::components::cover_art::TABLE_THUMBNAIL_SIZE,
                image_cache,
                &audiobook.id,
            )
            .with_title(audiobook.title.as_deref().unwrap_or("Unknown Title"));

            let width = match column.width {
                data::ColumnWidth::Fixed(w) => Length::Fixed(w),
                data::ColumnWidth::FillPortion(factor) => Length::FillPortion(factor),
                data::ColumnWidth::Auto => Length::Fixed(64.0), // Default width for cover art
                data::ColumnWidth::Ratio(num, _den) => {
                    Length::FillPortion(u16::try_from(num).unwrap_or(u16::MAX))
                }
                data::ColumnWidth::Shrink => Length::Fixed(64.0),
            };

            return container(cover_art.view(tokens))
                .width(width)
                .height(Length::Fill)
                .align_x(iced::Alignment::Center)
                .align_y(iced::alignment::Vertical::Center)
                .into();
        }

        let cell_text = match column.id.as_str() {
            "title" => audiobook
                .title
                .as_deref()
                .unwrap_or(&audiobook.id)
                .to_string(),
            "author" => audiobook.author.as_deref().unwrap_or("Unknown").to_string(),
            "duration" => Self::format_duration(audiobook.duration_seconds.unwrap_or(0)),
            "size" => Self::format_file_size(audiobook.size_bytes.unwrap_or(0)),
            "progress" => Self::format_progress(listened, audiobook.duration_seconds),
            "actions" => {
                return container(iced::widget::Space::with_width(Length::Fixed(0.0)))
                    .width(Length::Fixed(0.0))
                    .into();
            } // Already handled in create_single_row
            _ => String::new(),
        };
        let width = match column.width {
            data::ColumnWidth::Fixed(w) => Length::Fixed(w),
            data::ColumnWidth::FillPortion(factor) => Length::FillPortion(factor),
            data::ColumnWidth::Auto => Length::Shrink,
            data::ColumnWidth::Ratio(num, _den) => {
                Length::FillPortion(u16::try_from(num).unwrap_or(u16::MAX))
            }
            data::ColumnWidth::Shrink => Length::Shrink,
        };

        // Primary text for title/author, muted for numeric metadata
        let (text_color, size, font) = match column.id.as_str() {
            "title" => (
                if is_playing {
                    tokens.colors.primary.base
                } else {
                    tokens.colors.on_surface
                },
                tokens.typography().body_large.size,
                iced::Font {
                    weight: iced::font::Weight::Medium,
                    ..iced::Font::DEFAULT
                },
            ),
            "author" => (
                tokens.colors.on_surface,
                tokens.typography().body_medium.size,
                iced::Font::DEFAULT,
            ),
            _ => (
                tokens.colors.on_surface_variant,
                tokens.typography().body_medium.size,
                iced::Font::DEFAULT,
            ),
        };
        let align = match column.alignment {
            data::TextAlignment::End => iced::Alignment::End,
            data::TextAlignment::Center => iced::Alignment::Center,
            _ => iced::Alignment::Start,
        };

        let label = text(cell_text)
            .size(size)
            .font(font)
            .color(text_color)
            .wrapping(iced::widget::text::Wrapping::None);
        let content: Element<'a, Message> = if is_playing && column.id == "title" {
            row![
                crate::components::ui::icon("volume-high", 14.0, tokens.colors.primary.base),
                label
            ]
            .spacing(8)
            .align_y(iced::Alignment::Center)
            .into()
        } else {
            label.into()
        };

        container(content)
            .width(width)
            .height(Length::Fill)
            .padding(Padding::from([0, 12]))
            .clip(true)
            .align_x(align)
            .align_y(iced::alignment::Vertical::Center)
            .into()
    }

    /// Style for selectable rows: flat, with subtle hover and a tinted selection
    fn row_button_style(
        _theme: &iced::Theme,
        status: button::Status,
        tokens: &MaterialTokens,
        is_selected: bool,
        is_striped: bool,
        _row_index: usize,
    ) -> button::Style {
        use crate::components::ui::blend;
        let colors = &tokens.colors;
        let surface = if is_striped {
            colors.surface_container
        } else {
            colors.surface_container_low
        };
        let base = if is_selected {
            blend(surface, colors.primary.base, 0.14)
        } else {
            surface
        };
        let background = match status {
            button::Status::Hovered => blend(base, colors.on_surface, 0.05),
            button::Status::Pressed => blend(base, colors.on_surface, 0.10),
            _ => base,
        };

        button::Style {
            background: Some(Background::Color(background)),
            text_color: colors.on_surface,
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        }
    }

    /// "Finished", a percentage, or a dash when the book hasn't been started
    fn format_progress(
        listened: Option<&crate::state::ListeningProgress>,
        duration_seconds: Option<u64>,
    ) -> String {
        match listened {
            Some(progress) if progress.completed => "Finished".to_string(),
            Some(progress) => match progress.fraction(duration_seconds) {
                Some(fraction) if fraction > 0.0 => format!("{:.0}%", (fraction * 100.0).max(1.0)),
                _ => "—".to_string(),
            },
            None => "—".to_string(),
        }
    }

    /// Format duration in seconds to human-readable string (delegates to core utils)
    fn format_duration(seconds: u64) -> String {
        abop_core::utils::format_seconds(seconds, abop_core::utils::TimeFormat::HoursWhenNonZero)
    }

    /// Format file size in bytes to human-readable string
    fn format_file_size(bytes: u64) -> String {
        // Use the unified file size formatting from the casting module
        abop_core::utils::casting::format_file_size_standard(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ListeningProgress;

    #[test]
    fn formats_progress_cells() {
        let progress = |position_seconds, completed| ListeningProgress {
            position_seconds,
            completed,
        };
        assert_eq!(TableRow::format_progress(None, Some(100)), "—");
        assert_eq!(
            TableRow::format_progress(Some(&progress(0, false)), Some(100)),
            "—"
        );
        assert_eq!(
            TableRow::format_progress(Some(&progress(42, false)), Some(100)),
            "42%"
        );
        // A few seconds in still shows as started
        assert_eq!(
            TableRow::format_progress(Some(&progress(1, false)), Some(36_000)),
            "1%"
        );
        assert_eq!(
            TableRow::format_progress(Some(&progress(5, true)), None),
            "Finished"
        );
    }
}
