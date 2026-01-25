//! Range validation for audio parameters

use crate::error::{AppError, Result as AppResult};

/// Validate sample rate is within supported range
///
/// # Errors
///
/// Returns [`AppError::Audio`] if sample rate is outside supported range (8000-192000 Hz).
pub fn validate_sample_rate(sample_rate: u32) -> AppResult<()> {
    const MIN_SAMPLE_RATE: u32 = 8000;
    const MAX_SAMPLE_RATE: u32 = 192000;

    if !(MIN_SAMPLE_RATE..=MAX_SAMPLE_RATE).contains(&sample_rate) {
        return Err(AppError::Audio(format!(
            "Sample rate {} Hz is outside supported range ({}-{} Hz)",
            sample_rate, MIN_SAMPLE_RATE, MAX_SAMPLE_RATE
        )));
    }

    // Check for common sample rates and warn if unusual
    const COMMON_RATES: &[u32] = &[
        8000, 11025, 16000, 22050, 44100, 48000, 88200, 96000, 176400, 192000,
    ];

    if !COMMON_RATES.contains(&sample_rate) {
        log::warn!("Unusual sample rate specified: {} Hz", sample_rate);
    }

    Ok(())
}

/// Validate bit depth is within supported range
///
/// # Errors
///
/// Returns [`AppError::Audio`] if bit depth is not supported (16, 24, or 32 bits).
pub fn validate_bit_depth(bit_depth: u16) -> AppResult<()> {
    const SUPPORTED_DEPTHS: &[u16] = &[16, 24, 32];

    if !SUPPORTED_DEPTHS.contains(&bit_depth) {
        return Err(AppError::Audio(format!(
            "Bit depth {} bits is not supported. Supported depths: {:?}",
            bit_depth, SUPPORTED_DEPTHS
        )));
    }

    Ok(())
}

/// Validate channel count is within reasonable range
///
/// # Errors
///
/// Returns [`AppError::Audio`] if channel count is outside supported range (1-8 channels).
pub fn validate_channels(channels: u16) -> AppResult<()> {
    const MIN_CHANNELS: u16 = 1;
    const MAX_CHANNELS: u16 = 8;

    if !(MIN_CHANNELS..=MAX_CHANNELS).contains(&channels) {
        return Err(AppError::Audio(format!(
            "Channel count {} is outside supported range ({}-{} channels)",
            channels, MIN_CHANNELS, MAX_CHANNELS
        )));
    }

    if channels > 2 {
        log::info!("Multi-channel audio detected: {} channels", channels);
    }

    Ok(())
}

/// Validate audio buffer size is reasonable for processing
///
/// # Errors
///
/// Returns [`AppError::Audio`] if buffer size is too small or too large.
pub fn validate_buffer_size(buffer_size: usize) -> AppResult<()> {
    const MIN_BUFFER_SIZE: usize = 64;
    const MAX_BUFFER_SIZE: usize = 65536;

    if buffer_size < MIN_BUFFER_SIZE {
        return Err(AppError::Audio(format!(
            "Buffer size {} is too small (minimum: {} samples)",
            buffer_size, MIN_BUFFER_SIZE
        )));
    }

    if buffer_size > MAX_BUFFER_SIZE {
        return Err(AppError::Audio(format!(
            "Buffer size {} is too large (maximum: {} samples)",
            buffer_size, MAX_BUFFER_SIZE
        )));
    }

    Ok(())
}

/// Validate duration is within reasonable bounds
///
/// # Errors
///
/// Returns [`AppError::Audio`] if duration is negative or exceeds maximum reasonable duration.
pub fn validate_duration(duration: std::time::Duration) -> AppResult<()> {
    const MAX_REASONABLE_DURATION: std::time::Duration = std::time::Duration::from_secs(24 * 3600); // 24 hours

    if duration.is_zero() {
        return Err(AppError::Audio("Duration cannot be zero".to_string()));
    }

    if duration > MAX_REASONABLE_DURATION {
        return Err(AppError::Audio(format!(
            "Duration {:?} exceeds reasonable maximum (24 hours)",
            duration
        )));
    }

    Ok(())
}
