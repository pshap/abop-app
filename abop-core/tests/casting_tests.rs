//! Tests for the casting module
//!
//! These tests verify the behavior of the safe casting utilities, including:
//! - Database count conversions
//! - Audio sample rate and duration calculations
//! - Domain-specific error conditions
//! Note: File size formatting tests are in domain/file_size.rs

use abop_core::utils::casting::{
    builder::{CastingBuilder, PrecisionMode, RoundingMode},
    domain::audio,
    domain::db::{can_fit_in_usize, safe_db_count_to_usize, validate_db_count},
    domain::ui,
};

// Re-export for easier access in tests
use abop_core::utils::casting::domain::db as db_utils;

#[test]
fn test_db_count_conversion() {
    assert_eq!(safe_db_count_to_usize(100), 100);
    assert_eq!(safe_db_count_to_usize(-1), 0);
    assert_eq!(safe_db_count_to_usize(0), 0);
    assert_eq!(
        safe_db_count_to_usize(isize::MAX as i64),
        isize::MAX as usize
    );
}

#[test]
fn test_validate_db_count() {
    // Test valid cases
    assert_eq!(validate_db_count(0).unwrap(), 0);
    assert_eq!(validate_db_count(100).unwrap(), 100);

    // Test error cases
    assert!(validate_db_count(-1).is_err());

    // Test platform-specific behavior
    let max_safe = isize::MAX as i64;
    if cfg!(target_pointer_width = "64") {
        // On 64-bit platforms, isize::MAX should work
        assert_eq!(validate_db_count(max_safe).unwrap(), max_safe as usize);

        // On 64-bit platforms, i64::MAX should actually work too since usize is also 64-bit
        // But let's test with a value that definitely won't fit
        // We can't actually create a value larger than i64::MAX, so this test is moot
    } else {
        // On 32-bit platforms, isize::MAX is much smaller than i64::MAX
        let too_large = i64::MAX;
        assert!(validate_db_count(too_large).is_err());
    }
}

#[test]
fn test_can_fit_in_usize() {
    assert!(can_fit_in_usize(0));
    assert!(can_fit_in_usize(100));
    assert!(can_fit_in_usize(isize::MAX as i64));
    assert!(!can_fit_in_usize(-1));

    // On 64-bit systems, i64::MAX is larger than isize::MAX
    let max_i64 = i64::MAX;
    let max_isize = isize::MAX as i64;
    if max_i64 > max_isize {
        assert!(!can_fit_in_usize(max_i64));
    }
}

#[test]
fn test_audio_conversions() {
    // Test safe conversions
    let result = audio::safe_usize_to_f64_audio(1000);
    assert_eq!(result, 1000.0);

    let result = audio::safe_f64_to_usize_samples(1000.0).unwrap();
    assert_eq!(result, 1000);

    // Test error cases
    assert!(audio::safe_f64_to_usize_samples(-1.0).is_err());
    assert!(audio::safe_f64_to_usize_samples(f64::NAN).is_err());
}

#[test]
fn test_duration_sample_conversions() {
    // Test safe_duration_to_samples
    let samples = audio::safe_duration_to_samples(1.0, 44100).unwrap();
    assert_eq!(samples, 44100);

    let samples = audio::safe_duration_to_samples(0.5, 48000).unwrap();
    assert_eq!(samples, 24000);

    // Test error cases
    assert!(audio::safe_duration_to_samples(-1.0, 44100).is_err());
    assert!(audio::safe_duration_to_samples(1.0, 0).is_err());
    assert!(audio::safe_duration_to_samples(f32::NAN, 44100).is_err());

    // Test safe_samples_to_duration
    let duration = audio::safe_samples_to_duration(44100, 44100).unwrap();
    assert!((duration - 1.0).abs() < 0.001);

    let duration = audio::safe_samples_to_duration(22050, 44100).unwrap();
    assert!((duration - 0.5).abs() < 0.001);

    // Test error cases
    assert!(audio::safe_samples_to_duration(1000, 0).is_err());
}

#[test]
fn test_safe_progress() {
    // Test normal cases
    let progress = audio::safe_progress(50, 100).unwrap();
    assert!((progress - 0.5).abs() < 0.001);

    let progress = audio::safe_progress(100, 100).unwrap();
    assert!((progress - 1.0).abs() < 0.001);

    let progress = audio::safe_progress(0, 100).unwrap();
    assert!((progress - 0.0).abs() < 0.001);

    // Test error case for values over 100%
    assert!(audio::safe_progress(150, 100).is_err());

    // Test error cases
    assert!(audio::safe_progress(50, 0).is_err());
}

#[test]
fn test_domain_specific_errors() {
    // Test audio errors
    assert!(audio::safe_duration_to_samples(-1.0, 44100).is_err());
    assert!(audio::safe_duration_to_samples(1.0, 0).is_err());

    // Test UI errors
    assert!(ui::safe_spacing_to_pixels(-1.0).is_err());
    assert!(ui::safe_spacing_to_pixels(f32::NAN).is_err());
    // Note: 100_000.0 gets clamped to u16::MAX (65535), so it's actually ok
    assert!(ui::safe_spacing_to_pixels(100_000.0).is_ok());
    assert_eq!(ui::safe_spacing_to_pixels(100_000.0).unwrap(), u16::MAX);

    // Test database errors
    let large_value = i64::MAX;
    if large_value > isize::MAX as i64 {
        assert!(validate_db_count(large_value).is_err());
    }
}

#[test]
fn test_audio_conversion_methods() {
    let builder = CastingBuilder::for_realtime_audio(); // Use realtime preset with adaptive precision

    // Test convert_sample_rate
    let result = builder.convert_sample_rate(44100, 48000, 44100).unwrap();
    assert!(result > 44100); // Should be scaled up

    // Test time_to_samples
    let samples = builder.time_to_samples(1.0, 44100).unwrap();
    assert_eq!(samples, 44100);

    // Test convert_audio_value (16-bit to 24-bit)
    let value = 1000.0;
    let converted = builder.convert_audio_value(value, 16, 24).unwrap();
    assert!(converted > value as i64);

    // Test error cases
    assert!(builder.convert_sample_rate(0, 48000, 1000).is_err());
    assert!(builder.time_to_samples(1.0, 0).is_err());
    assert!(builder.convert_audio_value(1000.0, 0, 24).is_err());
}

#[test]
fn test_ui_conversion_methods() {
    let builder = CastingBuilder::for_ui();

    // Test logical_to_physical
    let physical = builder.logical_to_physical(100.0, 2.0).unwrap();
    assert_eq!(physical, 200);

    // Test physical_to_logical
    let logical = builder.physical_to_logical(200, 2.0).unwrap();
    assert!((logical - 100.0).abs() < f32::EPSILON);

    // Test error cases
    assert!(builder.logical_to_physical(f32::NAN, 1.0).is_err());
    assert!(builder.physical_to_logical(100, 0.0).is_err());
}

#[test]
fn test_validate_channel_count() {
    // Test valid cases
    assert_eq!(
        audio::validate_channel_count(1).unwrap(),
        1,
        "Single channel should be valid"
    );
    assert_eq!(
        audio::validate_channel_count(2).unwrap(),
        2,
        "Stereo should be valid"
    );
    assert_eq!(
        audio::validate_channel_count(6).unwrap(),
        6,
        "5.1 surround should be valid"
    );

    // Test invalid cases
    assert!(
        audio::validate_channel_count(0).is_err(),
        "Zero channels should be invalid"
    );
    assert!(
        audio::validate_channel_count(16).is_err(),
        "More than 8 channels should be invalid"
    );
}

#[test]
fn test_validate_bit_depth() {
    // Test valid cases
    assert_eq!(
        audio::validate_bit_depth(16).unwrap(),
        16,
        "16-bit should be valid"
    );
    assert_eq!(
        audio::validate_bit_depth(24).unwrap(),
        24,
        "24-bit should be valid"
    );
    assert_eq!(
        audio::validate_bit_depth(32).unwrap(),
        32,
        "32-bit should be valid"
    );

    // Test invalid cases
    assert!(
        audio::validate_bit_depth(7).is_err(),
        "7-bit should be invalid (not a power of 2)"
    );
    assert!(
        audio::validate_bit_depth(33).is_err(),
        "33-bit should be invalid (too high)"
    );
}

#[test]
fn test_validate_audiobook_duration() {
    // Test valid cases
    assert!(
        audio::validate_audiobook_duration(60.0).is_ok(),
        "1 minute should be valid"
    );
    assert!(
        audio::validate_audiobook_duration(3600.0).is_ok(),
        "1 hour should be valid"
    );
    assert!(
        audio::validate_audiobook_duration(7200.0).is_ok(),
        "2 hours should be valid"
    );

    // Test invalid cases
    assert!(
        audio::validate_audiobook_duration(0.5).is_err(),
        "30 seconds should be too short"
    );
    assert!(
        audio::validate_audiobook_duration(400000.0).is_err(),
        "Too long duration should be invalid"
    );
}

#[test]
fn test_validate_sample_rate_audiobook() {
    // Test valid cases
    assert_eq!(
        audio::validate_sample_rate_audiobook(44100).unwrap(),
        44100,
        "44.1kHz should be valid"
    );
    assert_eq!(
        audio::validate_sample_rate_audiobook(48000).unwrap(),
        48000,
        "48kHz should be valid"
    );

    // Test invalid cases
    assert!(
        audio::validate_sample_rate_audiobook(0).is_err(),
        "Zero sample rate should be invalid"
    );
    assert!(
        audio::validate_sample_rate_audiobook(200000).is_err(),
        "200kHz should be invalid (too high)"
    );
}

#[test]
fn test_convenience_wrappers() {
    // Test audio convenience functions
    assert_eq!(audio::duration_to_samples(1.0, 44100).unwrap(), 44100);
    assert_eq!(audio::samples_to_duration(44100, 44100).unwrap(), 1.0);
    assert_eq!(audio::samples_to_f64(1000), 1000.0);
    assert_eq!(audio::f64_to_samples(1000.0).unwrap(), 1000);

    // Test db convenience functions
    assert_eq!(db_utils::count_to_usize(100).unwrap(), 100);
    assert!(db_utils::count_to_usize(-1).is_err());
    assert_eq!(db_utils::size_to_i64(100).unwrap(), 100);
    assert_eq!(db_utils::count_to_size(100), 100);
}

#[test]
fn test_rounding_modes() {
    let value = 42.5;

    // Test with nearest rounding (rounds half away from zero)
    let nearest = CastingBuilder::new()
        .with_rounding(RoundingMode::Nearest)
        .with_precision(PrecisionMode::Adaptive) // Allow rounding
        .float_to_int::<i32>(value)
        .unwrap();
    assert_eq!(nearest, 43); // 42.5 rounds to 43 (away from zero)

    // Test with floor rounding
    let floor = CastingBuilder::new()
        .with_rounding(RoundingMode::Floor)
        .with_precision(PrecisionMode::Adaptive)
        .float_to_int::<i32>(value)
        .unwrap();
    assert_eq!(floor, 42);

    // Test with ceiling rounding
    let ceiling = CastingBuilder::new()
        .with_rounding(RoundingMode::Ceiling)
        .with_precision(PrecisionMode::Adaptive)
        .float_to_int::<i32>(value)
        .unwrap();
    assert_eq!(ceiling, 43);

    // Test with truncate rounding
    let truncate = CastingBuilder::new()
        .with_rounding(RoundingMode::Truncate)
        .with_precision(PrecisionMode::Adaptive)
        .float_to_int::<i32>(value)
        .unwrap();
    assert_eq!(truncate, 42);

    // Test with different precision modes
    let value = 1.23456789;

    // Test with strict precision (should fail on any precision loss)
    let strict = CastingBuilder::new()
        .with_precision(PrecisionMode::Strict)
        .float_to_int::<i32>(value);
    assert!(strict.is_err());

    // Test with tolerant precision (should allow small precision loss)
    let tolerant = CastingBuilder::new()
        .with_precision(PrecisionMode::Tolerant { epsilon: 0.5 }) // Allow up to 0.5 precision loss
        .float_to_int::<i32>(value)
        .unwrap();
    assert_eq!(tolerant, 1);

    // Test with adaptive precision
    let adaptive = CastingBuilder::new()
        .with_precision(PrecisionMode::Adaptive)
        .float_to_int::<i32>(value)
        .unwrap();
    assert_eq!(adaptive, 1);
}
