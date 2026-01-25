//! Channel count validation utilities

use crate::audio::processing::error::{AudioProcessingError, Result};

/// Maximum supported channel count
pub const MAX_CHANNELS: u16 = 32;

/// Common channel configurations
pub const MONO: u16 = 1;
/// Stereo channel count
pub const STEREO: u16 = 2;
/// 5.1 surround sound channel count
pub const SURROUND_5_1: u16 = 6;
/// 7.1 surround sound channel count
pub const SURROUND_7_1: u16 = 8;

/// Validates channel count
///
/// # Errors
///
/// Returns [`AudioProcessingError::ChannelValidation`] if channel count is zero or exceeds maximum supported channels.
pub fn validate_channels(channels: u16) -> Result<()> {
    if channels == 0 {
        return Err(AudioProcessingError::ChannelValidation(
            "Channel count cannot be zero".to_string(),
        ));
    }
    if channels > MAX_CHANNELS {
        return Err(AudioProcessingError::ChannelValidation(format!(
            "Channel count {channels} exceeds maximum {MAX_CHANNELS}"
        )));
    }
    Ok(())
}

/// Validates channel mixing configuration
///
/// # Errors
///
/// Returns [`AudioProcessingError::ChannelValidation`] if either channel count is invalid.
pub fn validate_channel_mixing(input_channels: u16, output_channels: u16) -> Result<()> {
    validate_channels(input_channels)?;
    validate_channels(output_channels)?;

    // Log warnings for unusual conversions
    if input_channels > output_channels && output_channels == 1 {
        log::debug!("Downmixing {input_channels} channels to mono");
    }

    Ok(())
}

/// Checks if channel configuration is standard
#[must_use]
pub const fn is_standard_configuration(channels: u16) -> bool {
    matches!(channels, MONO | STEREO | SURROUND_5_1 | SURROUND_7_1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_channels_valid() {
        assert!(validate_channels(1).is_ok());
        assert!(validate_channels(2).is_ok());
        assert!(validate_channels(8).is_ok());
    }

    #[test]
    fn test_validate_channels_invalid() {
        assert!(validate_channels(0).is_err());
        assert!(validate_channels(33).is_err());
    }

    #[test]
    fn test_validate_channel_mixing() {
        assert!(validate_channel_mixing(2, 1).is_ok());
        assert!(validate_channel_mixing(1, 2).is_err()); // Can't upmix to mono
    }

    #[test]
    fn test_is_standard_configuration() {
        assert!(is_standard_configuration(1)); // Mono
        assert!(is_standard_configuration(2)); // Stereo
        assert!(is_standard_configuration(6)); // 5.1
        assert!(is_standard_configuration(8)); // 7.1
        assert!(!is_standard_configuration(3)); // Non-standard
        assert!(!is_standard_configuration(10)); // Too many
    }
}
