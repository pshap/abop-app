//! Sample rate validation utilities

use crate::audio::processing::error::{AudioProcessingError, Result};

/// Common sample rate constants
pub const MIN_SAMPLE_RATE: u32 = 8_000;
/// Maximum supported sample rate
pub const MAX_SAMPLE_RATE: u32 = 192_000;
/// Standard audio sample rates
pub const STANDARD_RATES: &[u32] = &[
    8_000, 11_025, 16_000, 22_050, 44_100, 48_000, 88_200, 96_000, 176_400, 192_000,
];

/// Validates sample rate is within acceptable range
///
/// # Errors
///
/// Returns [`AudioProcessingError::SampleRateValidation`] if sample rate is outside of valid range.
pub fn validate_sample_rate(sample_rate: u32) -> Result<()> {
    if !(MIN_SAMPLE_RATE..=MAX_SAMPLE_RATE).contains(&sample_rate) {
        return Err(AudioProcessingError::SampleRateValidation(format!(
            "Sample rate {sample_rate} outside valid range [{MIN_SAMPLE_RATE}, {MAX_SAMPLE_RATE}]"
        )));
    }
    Ok(())
}

/// Checks if sample rate is a standard/common rate
#[must_use]
pub fn is_standard_sample_rate(sample_rate: u32) -> bool {
    STANDARD_RATES.contains(&sample_rate)
}

/// Finds the closest standard sample rate
#[must_use]
pub fn closest_standard_rate(sample_rate: u32) -> u32 {
    STANDARD_RATES
        .iter()
        .min_by_key(|&&rate| {
            let rate_i64 = i64::from(rate);
            let sample_rate_i64 = i64::from(sample_rate);
            (rate_i64 - sample_rate_i64).abs()
        })
        .copied()
        .unwrap_or(44100)
}

/// Validates sample rate conversion ratio
///
/// # Errors
///
/// Returns [`AudioProcessingError::SampleRateValidation`] if either sample rate is invalid.
pub fn validate_conversion_ratio(input_rate: u32, output_rate: u32) -> Result<()> {
    validate_sample_rate(input_rate)?;
    validate_sample_rate(output_rate)?;

    let ratio = if input_rate > output_rate {
        f64::from(input_rate) / f64::from(output_rate)
    } else {
        f64::from(output_rate) / f64::from(input_rate)
    };

    // Warn for extreme ratios that might cause quality issues
    if ratio > 8.0 {
        log::warn!(
            "Large sample rate conversion ratio: {ratio:.2}x ({input_rate}Hz -> {output_rate}Hz)"
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_sample_rate_valid() {
        assert!(validate_sample_rate(44100).is_ok());
        assert!(validate_sample_rate(48000).is_ok());
    }

    #[test]
    fn test_validate_sample_rate_invalid() {
        assert!(validate_sample_rate(0).is_err());
        assert!(validate_sample_rate(7_999).is_err());
        assert!(validate_sample_rate(200_000).is_err());
    }

    #[test]
    fn test_is_standard_sample_rate() {
        assert!(is_standard_sample_rate(44100));
        assert!(is_standard_sample_rate(48000));
        assert!(!is_standard_sample_rate(44056));
    }

    #[test]
    fn test_closest_standard_rate() {
        assert_eq!(closest_standard_rate(44056), 44100);
        assert_eq!(closest_standard_rate(49000), 48000);
        assert_eq!(closest_standard_rate(96000), 96000);
    }

    #[test]
    fn test_validate_conversion_ratio() {
        assert!(validate_conversion_ratio(44100, 48000).is_ok());
        assert!(validate_conversion_ratio(48000, 44100).is_ok());
        assert!(validate_conversion_ratio(0, 44100).is_err());
    }
}
