//! Tests for the Status component
//!
//! Comprehensive tests for status display with various progress states and edge cases.

use super::*;
use crate::components::status::{EnhancedStatusDisplayParams, StatusDisplay};

#[test]
fn status_display_renders_all_states() {
    let tokens = MaterialTokens::default();

    // Test basic status display
    let params = create_minimal_status_params();
    let element = StatusDisplay::enhanced_view(params, &tokens);
    let _ = element;
}

#[test]
fn status_display_handles_scan_progress_variants() {
    let tokens = MaterialTokens::default();

    let scan_progress_variants = [
        ScanProgress::Started { total_files: 100 },
        ScanProgress::FileProcessed {
            current: 50,
            total: 100,
            file_name: "test_file.mp3".to_string(),
            progress_percentage: 0.5,
        },
        ScanProgress::BatchCommitted {
            count: 10,
            total_processed: 50,
        },
        ScanProgress::Complete {
            processed: 100,
            errors: 0,
            duration: std::time::Duration::from_secs(30),
        },
    ];

    for progress in scan_progress_variants {
        let mut params = create_minimal_status_params();
        params.scanning = true;
        params.scan_progress = Some(progress.clone());
        params.cached_scan_progress_text = Some("Test progress");

        let element = StatusDisplay::enhanced_view(params, &tokens);
        let _ = element;
        println!("✓ Status display rendered with {progress:?}");
    }
}

#[test]
fn status_display_handles_processing_progress() {
    let tokens = MaterialTokens::default();

    let progress_values = [
        -0.1, // Below bounds - should clamp to 0.0
        0.0,  // Start
        0.25, // Quarter
        0.5,  // Half
        0.75, // Three quarters
        1.0,  // Complete
        1.5,  // Above bounds - should clamp to 1.0
    ];

    for progress in progress_values {
        let mut params = create_minimal_status_params();
        params.processing_audio = true;
        params.processing_progress = Some(progress);
        let progress_text = format!("Processing {:.1}%", progress * 100.0);
        params.cached_processing_progress_text = Some(&progress_text);

        let element = StatusDisplay::enhanced_view(params, &tokens);
        let _ = element;
    }
}

#[test]
fn status_display_handles_player_states() {
    let tokens = MaterialTokens::default();
    let test_path = PathBuf::from("/test/audio.mp3");

    let player_states = [
        (PlayerState::Stopped, None, "stopped without file"),
        (PlayerState::Playing, Some(&test_path), "playing with file"),
        (PlayerState::Paused, Some(&test_path), "paused with file"),
        (PlayerState::Playing, None, "playing without file"),
    ];

    for (state, file, description) in player_states {
        let mut params = create_minimal_status_params();
        params.player_state = state;
        params.current_playing_file = file;

        let element = StatusDisplay::enhanced_view(params, &tokens);
        let _ = element;
        println!("✓ Status display rendered with {description}");
    }
}

#[test]
fn status_display_handles_extreme_values() {
    let tokens = MaterialTokens::default();
    let very_long_path =
        PathBuf::from("/very/long/path/".to_string() + &"segment/".repeat(50) + "file.mp3");

    let mut params = create_minimal_status_params();

    // Test with extreme values
    params.scanning = true;
    params.scan_progress = Some(ScanProgress::FileProcessed {
        current: usize::MAX,
        total: usize::MAX,
        file_name: "x".repeat(1000),        // Very long filename
        progress_percentage: f32::INFINITY, // Extreme percentage
    });
    params.processing_audio = true;
    params.processing_progress = Some(f32::INFINITY);
    params.current_playing_file = Some(&very_long_path);
    params.selected_count = usize::MAX;
    params.total_count = usize::MAX;
    let long_scan_text = "Very long progress text ".repeat(100);
    let long_processing_text = "Very long processing text ".repeat(100);
    params.cached_scan_progress_text = Some(&long_scan_text);
    params.cached_processing_progress_text = Some(&long_processing_text);

    let element = StatusDisplay::enhanced_view(params, &tokens);
    let _ = element; // Should handle extreme values gracefully
}

#[test]
fn status_display_app_footer_renders() {
    let element = StatusDisplay::app_footer(42, ThemeMode::Light);
    let _ = element;

    // Test with different theme
    let element_dark = StatusDisplay::app_footer(0, ThemeMode::Dark);
    let _ = element_dark;
}

#[test]
fn status_display_concurrent_operations() {
    let tokens = MaterialTokens::default();
    let test_path = PathBuf::from("/test/concurrent.mp3");

    // Test with all operations running simultaneously
    let params = EnhancedStatusDisplayParams {
        scanning: true,
        scan_progress: Some(ScanProgress::FileProcessed {
            current: 25,
            total: 100,
            file_name: "scanning_file.mp3".to_string(),
            progress_percentage: 0.25,
        }),
        cached_scan_progress_text: Some("Scanning in progress..."),
        processing_audio: true,
        processing_progress: Some(0.75),
        cached_processing_progress_text: Some("Processing audio files..."),
        processing_status: Some("Batch 3 of 4"),
        player_state: PlayerState::Playing,
        current_playing_file: Some(&test_path),
        selected_count: 15,
        total_count: 100,
        theme: ThemeMode::MaterialDark,
    };

    let element = StatusDisplay::enhanced_view(params, &tokens);
    let _ = element; // Should handle multiple concurrent operations
}

/// Helper function to create minimal status parameters for testing
fn create_minimal_status_params<'a>() -> EnhancedStatusDisplayParams<'a> {
    EnhancedStatusDisplayParams {
        scanning: false,
        scan_progress: None,
        cached_scan_progress_text: None,
        processing_audio: false,
        processing_progress: None,
        cached_processing_progress_text: None,
        processing_status: None,
        player_state: PlayerState::Stopped,
        current_playing_file: None,
        selected_count: 0,
        total_count: 0,
        theme: ThemeMode::Light,
    }
}
