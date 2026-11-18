//! Playback message handlers
//!
//! The GUI keeps its own view of the player state (`state.player`) and syncs it
//! with the real audio player on [`Message::PlayerTick`], so the UI notices when
//! a book finishes playing.

use abop_core::PlayerState as CorePlayerState;
use abop_core::models::Audiobook;
use iced::Task;
use std::time::{Duration, Instant};

use crate::audio::player as audio;
use crate::messages::Message;
use crate::state::AppState;

/// Handles playback messages; returns `None` for anything else
pub fn handle_playback_message(state: &mut AppState, message: &Message) -> Option<Task<Message>> {
    let task = match message {
        Message::StartPlayback => start_selected(state),
        Message::RowClicked(id) => row_clicked(state, id, Instant::now()),
        Message::PlayPause => play_pause(state),
        Message::Stop => stop(state),
        Message::SeekBy(seconds) => seek_by(state, *seconds),
        Message::Next => skip(state, 1),
        Message::Previous => skip(state, -1),
        Message::PlaybackStarted(result) => {
            playback_started(state, result);
            Task::none()
        }
        Message::PlayerTick => sync_with_player(state),
        _ => return None,
    };
    Some(task)
}

/// Maximum time between two clicks on the same row to count as a double-click
const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(400);

/// Whether a click on `id` at `now` completes a double-click
fn is_double_click(previous: Option<&(String, Instant)>, id: &str, now: Instant) -> bool {
    previous.is_some_and(|(last_id, at)| {
        last_id == id && now.saturating_duration_since(*at) <= DOUBLE_CLICK_INTERVAL
    })
}

/// A single click toggles the row's selection; a double-click plays the book
fn row_clicked(state: &mut AppState, id: &str, now: Instant) -> Task<Message> {
    if is_double_click(state.ui.last_row_click.as_ref(), id, now) {
        state.ui.last_row_click = None;
        // The first click toggled the selection; keep the book selected
        state.library.selected_audiobooks.insert(id.to_string());
        let book = state
            .library
            .audiobooks
            .iter()
            .find(|b| b.id == id)
            .cloned();
        return book.map_or_else(Task::none, |book| play_book(state, book));
    }

    state.ui.last_row_click = Some((id.to_string(), now));
    let selected = &mut state.library.selected_audiobooks;
    if !selected.remove(id) {
        selected.insert(id.to_string());
    }
    Task::none()
}

/// Selected audiobooks in the order they are shown in the table
fn selected_in_display_order(state: &AppState) -> Vec<&Audiobook> {
    let selected = &state.library.selected_audiobooks;
    state
        .library
        .get_display_audiobooks()
        .iter()
        .filter(|book| selected.contains(&book.id))
        .collect()
}

/// Save progress at least this often while playing
const SAVE_INTERVAL: Duration = Duration::from_secs(15);

/// Persist the current book's position (or completion) in the background
fn save_progress_task(state: &mut AppState, completed: bool) -> Task<Message> {
    let Some(id) = state.player.playing_audiobook_id.clone() else {
        return Task::none();
    };
    let position = if completed {
        state.player.duration.unwrap_or(state.player.position)
    } else {
        state.player.position
    };
    state.player.last_saved_position = position;
    state.library.progress.insert(
        id.clone(),
        crate::state::ListeningProgress {
            position_seconds: position.as_secs(),
            completed,
        },
    );
    Task::perform(
        audio::save_progress(id, position.as_secs(), completed),
        |result| {
            if let Err(error) = result {
                log::warn!("Could not save listening progress: {error}");
            }
            Message::NoOp
        },
    )
}

/// Save progress before the app exits (used by the quit command)
pub fn save_before_exit(state: &mut AppState) -> Task<Message> {
    if state.player.player_state == CorePlayerState::Stopped {
        return Task::none();
    }
    state.player.position = audio::get_position();
    save_progress_task(state, false)
}

/// Clear everything about the current book from the player state
fn clear_current(state: &mut AppState) {
    state.player.player_state = CorePlayerState::Stopped;
    state.player.current_playing_file = None;
    state.player.playing_audiobook_id = None;
    state.player.position = Duration::ZERO;
    state.player.duration = None;
}

/// Start playing a specific audiobook, resuming from its saved position
fn play_book(state: &mut AppState, book: Audiobook) -> Task<Message> {
    log::info!("Playing {}", book.path.display());
    // Remember where we were in the previous book. While a start is still
    // pending the player's position belongs to an older book, so skip it.
    let save_previous = if state.player.player_state == CorePlayerState::Stopped
        || state.player.pending_starts > 0
    {
        Task::none()
    } else {
        state.player.position = audio::get_position();
        save_progress_task(state, false)
    };

    state.player.current_playing_file = Some(book.path.clone());
    state.player.playing_audiobook_id = Some(book.id.clone());
    state.player.player_state = CorePlayerState::Playing;
    state.player.position = Duration::ZERO;
    state.player.last_saved_position = Duration::ZERO;
    state.player.duration = book.duration_seconds.map(Duration::from_secs);
    state.player.pending_starts = state.player.pending_starts.saturating_add(1);
    Task::batch([
        save_previous,
        Task::perform(audio::play_audiobook(book), Message::PlaybackStarted),
    ])
}

/// Play the first selected audiobook (in table order)
fn start_selected(state: &mut AppState) -> Task<Message> {
    match selected_in_display_order(state).first() {
        Some(book) => {
            let book = (*book).clone();
            play_book(state, book)
        }
        None => {
            state.ui.notify_info("Select an audiobook to play");
            Task::none()
        }
    }
}

/// Toggle between playing and paused; starts the selection when stopped
fn play_pause(state: &mut AppState) -> Task<Message> {
    match state.player.player_state {
        CorePlayerState::Playing => {
            audio::pause_audio();
            state.player.player_state = CorePlayerState::Paused;
            state.player.position = audio::get_position();
            save_progress_task(state, false)
        }
        CorePlayerState::Paused => {
            audio::resume_audio();
            state.player.player_state = CorePlayerState::Playing;
            Task::none()
        }
        CorePlayerState::Stopped => start_selected(state),
    }
}

fn stop(state: &mut AppState) -> Task<Message> {
    let save = if state.player.player_state == CorePlayerState::Stopped {
        Task::none()
    } else {
        state.player.position = audio::get_position();
        save_progress_task(state, false)
    };
    audio::stop_audio();
    clear_current(state);
    save
}

/// Jump forward or back within the current book
fn seek_by(state: &mut AppState, seconds: i64) -> Task<Message> {
    if state.player.player_state == CorePlayerState::Stopped {
        return Task::none();
    }
    let current = audio::get_position();
    let delta = Duration::from_secs(seconds.unsigned_abs());
    let mut target = if seconds < 0 {
        current.saturating_sub(delta)
    } else {
        current + delta
    };
    if let Some(duration) = state.player.duration {
        target = target.min(duration.saturating_sub(Duration::from_secs(1)));
    }
    audio::seek_audio(target);
    state.player.position = target;
    Task::none()
}

/// Play the audiobook `offset` rows away from the current one (in table order)
fn skip(state: &mut AppState, offset: isize) -> Task<Message> {
    let Some(current) = state.player.current_playing_file.clone() else {
        // Nothing playing yet: start with the selection
        return start_selected(state);
    };
    let books = state.library.get_display_audiobooks();
    let Some(index) = books.iter().position(|book| book.path == current) else {
        log::warn!("Currently playing file is not in the library view");
        return Task::none();
    };
    match index.checked_add_signed(offset).and_then(|i| books.get(i)) {
        Some(book) => {
            let book = book.clone();
            play_book(state, book)
        }
        None => Task::none(), // already at the first/last audiobook
    }
}

fn playback_started(state: &mut AppState, result: &Result<String, String>) {
    state.player.pending_starts = state.player.pending_starts.saturating_sub(1);
    match result {
        Ok(message) => {
            log::info!("{message}");
            state.ui.notice = None;
        }
        Err(error) => {
            // A newer play request supersedes this one; only reset if none is pending
            if state.player.pending_starts == 0 {
                clear_current(state);
            }
            state
                .ui
                .notify_error(format!("Couldn't play audiobook: {error}"));
        }
    }
}

/// Reflect the real player state: position, duration and the end of a book
fn sync_with_player(state: &mut AppState) -> Task<Message> {
    if state.player.player_state != CorePlayerState::Playing || state.player.pending_starts > 0 {
        return Task::none();
    }
    if audio::get_player_state() == CorePlayerState::Stopped {
        log::info!("Playback finished");
        let save = save_progress_task(state, true);
        clear_current(state);
        return save;
    }

    state.player.position = audio::get_position();
    if let Some(duration) = audio::get_duration() {
        state.player.duration = Some(duration);
    }
    if state
        .player
        .position
        .abs_diff(state.player.last_saved_position)
        >= SAVE_INTERVAL
    {
        return save_progress_task(state, false);
    }
    Task::none()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn book(id: &str, title: &str) -> Audiobook {
        let mut book = Audiobook::new("lib", format!("/nonexistent/{id}.mp3"));
        book.id = id.to_string();
        book.title = Some(title.to_string());
        book
    }

    fn state_with_books() -> AppState {
        let mut state = AppState::default();
        state.library.audiobooks =
            vec![book("a", "Alpha"), book("b", "Bravo"), book("c", "Charlie")];
        state
    }

    #[test]
    fn start_without_selection_shows_hint() {
        let mut state = state_with_books();
        let _ = handle_playback_message(&mut state, &Message::StartPlayback);
        assert_eq!(state.player.player_state, CorePlayerState::Stopped);
        assert!(state.ui.notice.is_some());
    }

    #[test]
    fn start_plays_first_selected_in_table_order() {
        let mut state = state_with_books();
        state.library.selected_audiobooks.insert("c".into());
        state.library.selected_audiobooks.insert("b".into());
        let _ = handle_playback_message(&mut state, &Message::StartPlayback);
        assert_eq!(state.player.player_state, CorePlayerState::Playing);
        assert_eq!(
            state.player.current_playing_file.as_deref(),
            Some(std::path::Path::new("/nonexistent/b.mp3"))
        );
    }

    #[test]
    fn failed_playback_resets_state_and_reports_error() {
        let mut state = state_with_books();
        state.player.player_state = CorePlayerState::Playing;
        state.player.current_playing_file = Some("/nonexistent/a.mp3".into());
        let _ = handle_playback_message(
            &mut state,
            &Message::PlaybackStarted(Err("file not found".into())),
        );
        assert_eq!(state.player.player_state, CorePlayerState::Stopped);
        assert!(state.player.current_playing_file.is_none());
        assert_eq!(
            state.ui.notice.as_ref().map(|n| n.kind),
            Some(crate::state::NoticeKind::Error)
        );
    }

    #[test]
    fn next_and_previous_follow_table_order_and_stop_at_ends() {
        let mut state = state_with_books();
        state.player.current_playing_file = Some("/nonexistent/b.mp3".into());

        let _ = handle_playback_message(&mut state, &Message::Next);
        assert_eq!(
            state.player.current_playing_file.as_deref(),
            Some(std::path::Path::new("/nonexistent/c.mp3"))
        );
        // At the end: stays on the last book
        let _ = handle_playback_message(&mut state, &Message::Next);
        assert_eq!(
            state.player.current_playing_file.as_deref(),
            Some(std::path::Path::new("/nonexistent/c.mp3"))
        );

        state.player.current_playing_file = Some("/nonexistent/a.mp3".into());
        let _ = handle_playback_message(&mut state, &Message::Previous);
        assert_eq!(
            state.player.current_playing_file.as_deref(),
            Some(std::path::Path::new("/nonexistent/a.mp3"))
        );
    }

    #[test]
    fn ticks_are_ignored_until_playback_has_started() {
        let mut state = state_with_books();
        state.library.selected_audiobooks.insert("a".into());
        let _ = handle_playback_message(&mut state, &Message::StartPlayback);
        assert_eq!(state.player.pending_starts, 1);

        // The audio thread hasn't started the book yet: the tick must not
        // treat its stale "stopped" status as the end of the book
        let _ = handle_playback_message(&mut state, &Message::PlayerTick);
        assert_eq!(state.player.player_state, CorePlayerState::Playing);
        assert_eq!(state.player.playing_audiobook_id.as_deref(), Some("a"));
        assert!(!state.library.progress.contains_key("a"));
    }

    #[test]
    fn superseded_start_failure_keeps_the_newer_book() {
        let mut state = state_with_books();
        state.library.selected_audiobooks.insert("a".into());
        let _ = handle_playback_message(&mut state, &Message::StartPlayback);
        let _ = handle_playback_message(&mut state, &Message::Next);
        assert_eq!(state.player.pending_starts, 2);
        assert_eq!(state.player.playing_audiobook_id.as_deref(), Some("b"));

        // The first request fails after the second was sent
        let _ = handle_playback_message(
            &mut state,
            &Message::PlaybackStarted(Err("file not found".into())),
        );
        assert_eq!(state.player.pending_starts, 1);
        assert_eq!(state.player.playing_audiobook_id.as_deref(), Some("b"));
    }

    #[test]
    fn pause_and_resume_toggle_state() {
        let mut state = state_with_books();
        state.player.player_state = CorePlayerState::Playing;
        let _ = handle_playback_message(&mut state, &Message::PlayPause);
        assert_eq!(state.player.player_state, CorePlayerState::Paused);
        let _ = handle_playback_message(&mut state, &Message::PlayPause);
        assert_eq!(state.player.player_state, CorePlayerState::Playing);
    }

    #[test]
    fn single_click_toggles_and_double_click_plays() {
        let mut state = state_with_books();
        let t0 = Instant::now();

        let _ = row_clicked(&mut state, "b", t0);
        assert!(state.library.selected_audiobooks.contains("b"));
        assert_eq!(state.player.player_state, CorePlayerState::Stopped);

        // Second click shortly after on the same row: play, stay selected
        let _ = row_clicked(&mut state, "b", t0 + Duration::from_millis(200));
        assert!(state.library.selected_audiobooks.contains("b"));
        assert_eq!(state.player.player_state, CorePlayerState::Playing);
        assert_eq!(
            state.player.current_playing_file.as_deref(),
            Some(std::path::Path::new("/nonexistent/b.mp3"))
        );
    }

    #[test]
    fn slow_or_different_clicks_are_not_double_clicks() {
        let t0 = Instant::now();
        let previous = ("a".to_string(), t0);
        assert!(!is_double_click(
            Some(&previous),
            "a",
            t0 + Duration::from_secs(1)
        ));
        assert!(!is_double_click(
            Some(&previous),
            "b",
            t0 + Duration::from_millis(100)
        ));
        assert!(!is_double_click(None, "a", t0));
        assert!(is_double_click(
            Some(&previous),
            "a",
            t0 + Duration::from_millis(100)
        ));

        // Two slow clicks just toggle the selection on and off
        let mut state = state_with_books();
        let _ = row_clicked(&mut state, "a", t0);
        let _ = row_clicked(&mut state, "a", t0 + Duration::from_secs(1));
        assert!(!state.library.selected_audiobooks.contains("a"));
        assert_eq!(state.player.player_state, CorePlayerState::Stopped);
    }

    #[test]
    fn playing_a_book_records_id_and_duration() {
        let mut state = state_with_books();
        state.library.audiobooks[1].duration_seconds = Some(3600);
        state.library.selected_audiobooks.insert("b".into());
        let _ = handle_playback_message(&mut state, &Message::StartPlayback);
        assert_eq!(state.player.playing_audiobook_id.as_deref(), Some("b"));
        assert_eq!(state.player.duration, Some(Duration::from_secs(3600)));
        assert_eq!(state.player.position, Duration::ZERO);
    }

    #[test]
    fn seeking_is_ignored_when_stopped_and_clamped_to_duration() {
        let mut state = state_with_books();
        let _ = handle_playback_message(&mut state, &Message::SeekBy(30));
        assert_eq!(state.player.position, Duration::ZERO);

        state.player.player_state = CorePlayerState::Paused;
        state.player.duration = Some(Duration::from_secs(100));
        let _ = handle_playback_message(&mut state, &Message::SeekBy(1_000));
        assert_eq!(state.player.position, Duration::from_secs(99));
        let _ = handle_playback_message(&mut state, &Message::SeekBy(-1_000));
        assert_eq!(state.player.position, Duration::ZERO);
    }

    #[test]
    fn stop_clears_current_file() {
        let mut state = state_with_books();
        state.player.player_state = CorePlayerState::Playing;
        state.player.current_playing_file = Some("/nonexistent/a.mp3".into());
        let _ = handle_playback_message(&mut state, &Message::Stop);
        assert_eq!(state.player.player_state, CorePlayerState::Stopped);
        assert!(state.player.current_playing_file.is_none());
    }
}
