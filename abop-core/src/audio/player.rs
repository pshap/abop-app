//! Audio playback functionality using Rodio
//!
//! This module provides audio playback capabilities for the ABOP application.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError, mpsc};
use std::time::Duration;

use rodio::{OutputStream, Sink};

use crate::error::{AppError, Result};

/// Audio player state
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PlayerState {
    /// Player is stopped
    #[default]
    Stopped,
    /// Player is playing
    Playing,
    /// Player is paused
    Paused,
}

/// Thread-safe audio player for playing audio files
///
/// This type is not `Send` or `Sync` because rodio's output stream is bound to the
/// thread that created it. Use [`ThreadSafeAudioPlayer`] to control playback from
/// other threads.
pub struct AudioPlayer {
    /// Current audio sink
    sink: Option<Sink>,
    /// Audio output stream handle (must be kept alive)
    _stream: Option<OutputStream>,
    /// Current player state
    state: PlayerState,
    /// Current volume (0.0 to 1.0)
    volume: f32,
    /// Current playing file path
    current_file: Option<PathBuf>,
    /// Position the current source started from (sink positions are relative to it)
    position_offset: Duration,
    /// Total duration of the current file, if known
    duration: Option<Duration>,
}

/// Commands sent to the dedicated audio thread
enum AudioCommand {
    Play(PathBuf, Duration, mpsc::Sender<Result<()>>),
    Seek(Duration),
    Stop,
    Pause,
    Resume,
    SetVolume(f32),
}

/// Snapshot of the player status shared with other threads
#[derive(Debug, Clone, Default)]
struct PlayerStatus {
    state: PlayerState,
    current_file: Option<PathBuf>,
    volume: f32,
    position: Duration,
    duration: Option<Duration>,
}

/// How often the audio thread checks whether playback has finished
const STATUS_POLL_INTERVAL: Duration = Duration::from_millis(250);

/// Thread-safe handle to an [`AudioPlayer`]
///
/// Rodio's output stream is not `Send`, so the actual [`AudioPlayer`] lives on a
/// dedicated audio thread. This handle forwards commands to that thread over a
/// channel and exposes a status snapshot, making it `Send + Sync` and cheap to clone.
#[derive(Clone)]
pub struct ThreadSafeAudioPlayer {
    commands: mpsc::Sender<AudioCommand>,
    status: Arc<Mutex<PlayerStatus>>,
}

impl ThreadSafeAudioPlayer {
    /// Creates a new thread-safe audio player, spawning its audio thread
    ///
    /// # Errors
    ///
    /// Returns `AppError::Audio` if the audio thread cannot be spawned.
    pub fn new() -> Result<Self> {
        let status = Arc::new(Mutex::new(PlayerStatus {
            volume: AudioPlayer::default().get_volume(),
            ..PlayerStatus::default()
        }));
        let (tx, rx) = mpsc::channel();

        let thread_status = Arc::clone(&status);
        // The player must be created on the audio thread: rodio streams are not `Send`
        std::thread::Builder::new()
            .name("abop-audio".to_string())
            .spawn(move || Self::run(AudioPlayer::default(), &rx, &thread_status))
            .map_err(|e| AppError::Audio(format!("Failed to spawn audio thread: {e}")))?;

        Ok(Self {
            commands: tx,
            status,
        })
    }

    /// Audio thread main loop; exits when every handle has been dropped
    fn run(
        mut player: AudioPlayer,
        rx: &mpsc::Receiver<AudioCommand>,
        status: &Mutex<PlayerStatus>,
    ) {
        loop {
            match rx.recv_timeout(STATUS_POLL_INTERVAL) {
                Ok(AudioCommand::Play(path, start, reply)) => {
                    let result = player.play_from(&path, start);
                    // The caller may have stopped waiting; that's fine
                    let _ = reply.send(result);
                }
                Ok(AudioCommand::Seek(position)) => {
                    if let Err(e) = player.seek(position) {
                        log::warn!("{e}");
                    }
                }
                Ok(AudioCommand::Stop) => player.stop(),
                Ok(AudioCommand::Pause) => player.pause(),
                Ok(AudioCommand::Resume) => player.resume(),
                Ok(AudioCommand::SetVolume(v)) => player.set_volume(v),
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }
            player.update_state();
            let mut snapshot = status.lock().unwrap_or_else(PoisonError::into_inner);
            snapshot.state = player.get_state();
            snapshot.current_file = player.get_current_file();
            snapshot.volume = player.get_volume();
            snapshot.position = player.position().unwrap_or_default();
            snapshot.duration = player.duration();
        }
        player.stop();
    }

    fn send(&self, command: AudioCommand) -> Result<()> {
        self.commands
            .send(command)
            .map_err(|_| AppError::Audio("Audio thread is not running".to_string()))
    }

    fn snapshot(&self) -> PlayerStatus {
        self.status
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    /// Plays an audio file, blocking until playback has started or failed
    ///
    /// # Errors
    ///
    /// See `AudioPlayer::play` for details.
    pub fn play<P: AsRef<Path>>(&self, file_path: P) -> Result<()> {
        self.play_from(file_path, Duration::ZERO)
    }

    /// Plays an audio file starting at `start`, blocking until playback has started
    ///
    /// # Errors
    ///
    /// See `AudioPlayer::play_from` for details.
    pub fn play_from<P: AsRef<Path>>(&self, file_path: P, start: Duration) -> Result<()> {
        let (reply_tx, reply_rx) = mpsc::channel();
        self.send(AudioCommand::Play(
            file_path.as_ref().to_path_buf(),
            start,
            reply_tx,
        ))?;
        let result = reply_rx
            .recv()
            .map_err(|_| AppError::Audio("Audio thread stopped unexpectedly".to_string()))?;
        if result.is_ok() {
            // Reflect the new state immediately instead of waiting for the next poll
            let mut status = self.status.lock().unwrap_or_else(PoisonError::into_inner);
            status.state = PlayerState::Playing;
            status.current_file = Some(file_path.as_ref().to_path_buf());
            status.position = start;
        }
        result
    }

    /// Jumps to `position` in the current file
    pub fn seek(&self, position: Duration) {
        if self.send(AudioCommand::Seek(position)).is_ok() {
            self.status
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .position = position;
        }
    }

    /// Current playback position (updated a few times per second)
    #[must_use]
    pub fn get_position(&self) -> Duration {
        self.snapshot().position
    }

    /// Total duration of the current file, if known
    #[must_use]
    pub fn get_duration(&self) -> Option<Duration> {
        self.snapshot().duration
    }

    /// Stops audio playback
    pub fn stop(&self) {
        if self.send(AudioCommand::Stop).is_ok() {
            let mut status = self.status.lock().unwrap_or_else(PoisonError::into_inner);
            status.state = PlayerState::Stopped;
            status.current_file = None;
            status.position = Duration::ZERO;
            status.duration = None;
        }
    }

    /// Pauses audio playback
    pub fn pause(&self) {
        let _ = self.send(AudioCommand::Pause);
    }

    /// Resumes audio playback
    pub fn resume(&self) {
        let _ = self.send(AudioCommand::Resume);
    }

    /// Sets the volume (0.0 to 1.0)
    pub fn set_volume(&self, volume: f32) {
        let _ = self.send(AudioCommand::SetVolume(volume));
    }

    /// Gets the current volume (0.0 to 1.0)
    #[must_use]
    pub fn get_volume(&self) -> f32 {
        self.snapshot().volume
    }

    /// Gets the current player state
    #[must_use]
    pub fn get_state(&self) -> PlayerState {
        self.snapshot().state
    }

    /// Gets the currently playing file path
    #[must_use]
    pub fn get_current_file(&self) -> Option<PathBuf> {
        self.snapshot().current_file
    }
}

impl AudioPlayer {
    /// Creates a new audio player
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Audio`] if audio system initialization fails.
    pub const fn new() -> Result<Self> {
        Ok(Self {
            sink: None,
            _stream: None,
            state: PlayerState::Stopped,
            volume: 0.7, // Default volume 70%
            current_file: None,
            position_offset: Duration::ZERO,
            duration: None,
        })
    }

    /// Creates a new thread-safe audio player
    ///
    /// This is a convenience method that wraps the player in a `ThreadSafeAudioPlayer`.
    ///
    /// # Errors
    ///
    /// Returns `AppError::Audio` if audio system initialization fails.
    pub fn new_thread_safe() -> Result<ThreadSafeAudioPlayer> {
        ThreadSafeAudioPlayer::new()
    }

    /// Plays an audio file
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Audio`] if the audio output stream cannot be created,
    /// the file format is unsupported, or the audio sink creation fails.
    /// Returns [`AppError::Io`] if the file cannot be read.
    pub fn play<P: AsRef<Path>>(&mut self, file_path: P) -> Result<()> {
        self.play_from(file_path, Duration::ZERO)
    }

    /// Plays an audio file starting at `start`
    ///
    /// # Errors
    ///
    /// Same as [`AudioPlayer::play`]; a failed seek to `start` is logged and
    /// playback begins at the start of the file instead.
    pub fn play_from<P: AsRef<Path>>(&mut self, file_path: P, start: Duration) -> Result<()> {
        use rodio::Source;
        let file_path = file_path.as_ref();

        // Stop any currently playing audio
        self.stop();

        // Create output stream for this playback session
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| AppError::Audio(format!("Failed to create audio output stream: {e}")))?;

        // Decode with our Symphonia decoder (rodio's own fails on many M4B files)
        let mut source = super::playback_source::DecoderSource::open(file_path).map_err(|e| {
            AppError::Audio(format!(
                "Failed to decode audio file '{}': {e}",
                file_path.display()
            ))
        })?;
        let duration = source.total_duration();
        // Seek before handing the source to rodio, so starting never blocks
        let offset = if start.is_zero() {
            Duration::ZERO
        } else if let Err(e) = source.try_seek(start) {
            log::warn!("Could not resume at {start:?}, starting from the beginning: {e}");
            source = super::playback_source::DecoderSource::open(file_path)?;
            Duration::ZERO
        } else {
            start
        };

        // Create new sink and set volume
        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| AppError::Audio(format!("Failed to create audio sink: {e}")))?;
        sink.set_volume(self.volume);

        // Append the source and play
        sink.append(source);
        sink.play();

        // Update state
        self.sink = Some(sink);
        self._stream = Some(stream);
        self.state = PlayerState::Playing;
        self.current_file = Some(file_path.to_path_buf());
        self.position_offset = offset;
        self.duration = duration;

        log::info!("Started playing audio file: {}", file_path.display());
        Ok(())
    }

    /// Stops audio playback
    pub fn stop(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
            log::info!("Stopped audio playback");
        }
        self._stream = None;

        self.state = PlayerState::Stopped;
        self.current_file = None;
        self.position_offset = Duration::ZERO;
        self.duration = None;
    }

    /// Jumps to `position` in the current file
    ///
    /// # Errors
    ///
    /// Returns an error if nothing is playing or the source cannot seek.
    pub fn seek(&mut self, position: Duration) -> Result<()> {
        let sink = self
            .sink
            .as_ref()
            .ok_or_else(|| AppError::Audio("Nothing is playing".to_string()))?;
        let position = self.duration.map_or(position, |d| position.min(d));
        sink.try_seek(position)
            .map_err(|e| AppError::Audio(format!("Seek failed: {e}")))?;
        // rodio now reports positions from the start of the file
        self.position_offset = Duration::ZERO;
        Ok(())
    }

    /// Current playback position, if something is loaded
    #[must_use]
    pub fn position(&self) -> Option<Duration> {
        self.sink
            .as_ref()
            .map(|sink| self.position_offset + sink.get_pos())
    }

    /// Total duration of the current file, if known
    #[must_use]
    pub const fn duration(&self) -> Option<Duration> {
        self.duration
    }
    /// Pauses audio playback
    pub fn pause(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.pause();
            self.state = PlayerState::Paused;
            log::info!("Paused audio playback");
        }
    }

    /// Resumes audio playback
    pub fn resume(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.play();
            self.state = PlayerState::Playing;
            log::info!("Resumed audio playback");
        }
    }

    /// Gets the current player state
    #[must_use]
    pub fn get_state(&self) -> PlayerState {
        self.state.clone()
    }

    /// Sets the volume (0.0 to 1.0)
    pub fn set_volume(&mut self, volume: f32) {
        let volume = volume.clamp(0.0, 1.0);
        self.volume = volume;

        if let Some(ref sink) = self.sink {
            sink.set_volume(volume);
        }
        log::debug!("Set volume to {:.1}%", volume * 100.0);
    }

    /// Gets the current volume (0.0 to 1.0)
    #[must_use]
    pub const fn get_volume(&self) -> f32 {
        self.volume
    }

    /// Checks if the player is currently playing
    #[must_use]
    pub fn is_playing(&self) -> bool {
        matches!(self.get_state(), PlayerState::Playing)
    }

    /// Checks if the player is paused
    #[must_use]
    pub fn is_paused(&self) -> bool {
        matches!(self.get_state(), PlayerState::Paused)
    }

    /// Checks if the player is stopped
    #[must_use]
    pub fn is_stopped(&self) -> bool {
        matches!(self.get_state(), PlayerState::Stopped)
    }

    /// Gets the currently playing file path
    #[must_use]
    pub fn get_current_file(&self) -> Option<PathBuf> {
        self.current_file.clone()
    }

    /// Checks if the sink is empty (finished playing)
    pub fn is_finished(&self) -> bool {
        self.sink.as_ref().is_none_or(|s| s.empty())
    }

    /// Updates the player state based on sink status
    pub fn update_state(&mut self) {
        if let Some(sink) = self.sink.as_ref()
            && sink.empty()
        {
            self.state = PlayerState::Stopped;
        }
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self {
            sink: None,
            _stream: None,
            state: PlayerState::Stopped,
            volume: 0.7,
            current_file: None,
            position_offset: Duration::ZERO,
            duration: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_player_creation() {
        let player = AudioPlayer::new();
        assert!(player.is_ok());
    }

    #[test]
    fn test_initial_state() {
        let player = AudioPlayer::new().unwrap();
        assert_eq!(player.get_state(), PlayerState::Stopped);
        assert!(!player.is_playing());
        assert!(!player.is_paused());
        assert!(player.is_stopped());
        assert_eq!(player.get_volume(), 0.7);
        assert!(player.get_current_file().is_none());
    }

    #[test]
    fn test_volume_control() {
        let mut player = AudioPlayer::new().unwrap();

        player.set_volume(0.5);
        assert_eq!(player.get_volume(), 0.5);

        // Test clamping
        player.set_volume(1.5);
        assert_eq!(player.get_volume(), 1.0);

        player.set_volume(-0.5);
        assert_eq!(player.get_volume(), 0.0);
    }

    #[test]
    fn test_thread_safe_player_is_send_sync() {
        fn assert_send_sync<T: Send + Sync + 'static>() {}
        assert_send_sync::<ThreadSafeAudioPlayer>();
    }

    #[test]
    fn test_thread_safe_player_initial_state() {
        let player = ThreadSafeAudioPlayer::new().unwrap();
        assert_eq!(player.get_state(), PlayerState::Stopped);
        assert!(player.get_current_file().is_none());
        assert_eq!(player.get_volume(), 0.7);
    }

    /// Needs a real audio output device: `cargo test -p abop-core -- --ignored`
    #[test]
    #[ignore = "requires an audio output device"]
    fn test_thread_safe_player_plays_pauses_and_finishes() {
        let fixture = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/tagged.mp3");
        let player = ThreadSafeAudioPlayer::new().unwrap();
        player.play(fixture).expect("fixture should play");
        assert_eq!(player.get_state(), PlayerState::Playing);
        assert!(player.get_current_file().is_some());

        player.pause();
        std::thread::sleep(Duration::from_millis(400));
        assert_eq!(player.get_state(), PlayerState::Paused);

        player.resume();
        std::thread::sleep(Duration::from_millis(400));
        assert_eq!(player.get_state(), PlayerState::Playing);

        // The fixture is one second long; the audio thread should notice the end
        std::thread::sleep(Duration::from_millis(2000));
        assert_eq!(player.get_state(), PlayerState::Stopped);
    }

    #[test]
    #[ignore = "requires an audio output device"]
    fn test_thread_safe_player_resumes_and_seeks() {
        let fixture = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/tagged.m4b");
        let player = ThreadSafeAudioPlayer::new().unwrap();
        player
            .play_from(fixture, Duration::from_millis(500))
            .unwrap();
        std::thread::sleep(Duration::from_millis(300));
        let position = player.get_position();
        assert!(
            position >= Duration::from_millis(500),
            "resumed at {position:?}"
        );
        assert!(player.get_duration().is_some());

        player.seek(Duration::from_millis(100));
        std::thread::sleep(Duration::from_millis(300));
        let position = player.get_position();
        assert!(
            position < Duration::from_millis(500),
            "after seek {position:?}"
        );
        player.stop();
    }

    /// M4B/M4A need symphonia's aac + isomp4 features (enabled in the workspace)
    #[test]
    #[ignore = "requires an audio output device"]
    fn test_thread_safe_player_plays_m4b() {
        let fixture = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/tagged.m4b");
        let player = ThreadSafeAudioPlayer::new().unwrap();
        player.play(fixture).expect("m4b fixture should play");
        assert_eq!(player.get_state(), PlayerState::Playing);
        player.stop();
    }

    #[test]
    fn test_thread_safe_player_play_missing_file_errors() {
        let player = ThreadSafeAudioPlayer::new().unwrap();
        assert!(player.play("/nonexistent/abop/test-file.mp3").is_err());
        assert_eq!(player.get_state(), PlayerState::Stopped);
        player.stop();
        assert_eq!(player.get_state(), PlayerState::Stopped);
    }
}
