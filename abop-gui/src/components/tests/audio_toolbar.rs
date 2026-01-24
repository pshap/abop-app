//! Tests for `audio_toolbar` component

use crate::components::audio_toolbar::AudioToolbar;
use crate::styling::material::MaterialTokens;

#[test]
fn test_audio_toolbar_creation() {
    let tokens = MaterialTokens::default();
    let toolbar = AudioToolbar::new();
    let element = toolbar.view(&tokens);
    let _ = element; // Just verify it compiles and runs
}

#[test]
fn test_audio_toolbar_with_playing_state() {
    let tokens = MaterialTokens::default();
    let mut toolbar = AudioToolbar::new();

    toolbar.set_playing(true);
    let playing_element = toolbar.view(&tokens);
    let _ = playing_element; // Just verify it compiles and runs

    toolbar.set_playing(false);
    let stopped_element = toolbar.view(&tokens);
    let _ = stopped_element; // Just verify it compiles and runs
}
