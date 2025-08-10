//! Tests for the AudioToolbar component
//!
//! Tests covering toolbar state management and rendering.

use super::*;
use crate::components::audio_toolbar::AudioToolbar;

#[test]
fn audio_toolbar_creates_successfully() {
    let tokens = MaterialTokens::default();
    let toolbar = AudioToolbar::new();
    let element = toolbar.view(&tokens);
    let _ = element;
}

#[test]
fn audio_toolbar_handles_state_transitions() {
    let tokens = MaterialTokens::default();
    let mut toolbar = AudioToolbar::new();

    // Test state transitions
    let states = [
        (false, "stopped state"),
        (true, "playing state"),
        (false, "back to stopped"),
    ];

    for (playing, description) in states {
        toolbar.set_playing(playing);
        let element = toolbar.view(&tokens);
        let _ = element;
        println!("✓ Audio toolbar rendered in {description}");
    }
}

#[test]
fn audio_toolbar_state_consistency() {
    let mut toolbar = AudioToolbar::new();
    let tokens = MaterialTokens::default();
    
    // Test multiple state transitions to ensure consistency
    for i in 0..10 {
        let expected_state = i % 2 == 1;
        toolbar.set_playing(expected_state);
        
        // Verify the toolbar still renders successfully after state changes
        let element = toolbar.view(&tokens);
        let _ = element;
    }
}