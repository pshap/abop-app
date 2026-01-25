//! Scalar implementation of linear resampling algorithm

use super::super::{casting_utils::error_conversion::cast_to_audio_error, error::Result};
use crate::audio::AudioBuffer;
use crate::utils::casting::domain::audio::{safe_f64_to_usize_samples, safe_usize_to_f64_audio};

/// Scalar implementation of resampling algorithm.
/// This is used as a reference implementation and fallback when SIMD is not available.
#[doc(hidden)]
#[cfg_attr(feature = "bench", allow(unused))]
pub fn resample_buffer_scalar(buffer: &mut AudioBuffer<f32>, target_rate: u32) -> Result<()> {
    let ratio = f64::from(target_rate) / f64::from(buffer.sample_rate);
    log::debug!(
        "Resampling from {} Hz to {} Hz (ratio: {:.3})",
        buffer.sample_rate,
        target_rate,
        ratio
    );

    // Calculate output length with safe conversion
    let input_samples_f64 = safe_usize_to_f64_audio(buffer.data.len()) / f64::from(buffer.channels);
    let output_samples_f64 = input_samples_f64 * ratio;

    // Safe conversion to usize with bounds checking
    let output_samples =
        safe_f64_to_usize_samples(output_samples_f64).map_err(|e| cast_to_audio_error(e.into()))?;

    // Create new buffer for resampled data
    let channels_usize = usize::from(buffer.channels);
    let mut resampled_data = Vec::with_capacity(output_samples * channels_usize);

    // Linear interpolation resampling
    for i in 0..output_samples {
        // Calculate source position with safe bounds checking using f64 for precision
        let i_f64 = safe_usize_to_f64_audio(i);
        let pos_f64 = i_f64 * f64::from(buffer.sample_rate) / f64::from(target_rate);

        // Get integer and fractional parts of position
        let pos_floor = pos_f64.floor();
        let pos_frac = pos_f64 - pos_floor;

        // Convert to usize for array indexing
        let pos0 = pos_floor as usize;
        let pos1 = (pos_floor as usize + 1).min((input_samples_f64 - 1.0) as usize);

        // For each channel
        for c in 0..channels_usize {
            // Get two samples to interpolate between
            let idx0 = pos0 * channels_usize + c;
            let idx1 = pos1 * channels_usize + c;

            // Get sample values with bounds checking
            let sample0 = if idx0 < buffer.data.len() {
                buffer.data[idx0]
            } else if !buffer.data.is_empty() {
                *buffer.data.last().unwrap()
            } else {
                0.0
            };

            let sample1 = if idx1 < buffer.data.len() {
                buffer.data[idx1]
            } else if !buffer.data.is_empty() {
                *buffer.data.last().unwrap()
            } else {
                0.0
            };

            // Linear interpolation: sample0 + (sample1 - sample0) * pos_frac
            let interpolated = sample0 + (sample1 - sample0) * pos_frac as f32;
            resampled_data.push(interpolated);
        }
    }

    // Update buffer with resampled data
    buffer.data = resampled_data;
    buffer.sample_rate = target_rate;

    Ok(())
}
