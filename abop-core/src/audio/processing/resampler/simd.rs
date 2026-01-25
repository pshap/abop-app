//! SIMD-accelerated implementation of linear resampling algorithm

use super::super::error::Result;
use crate::audio::AudioBuffer;

/// SIMD-accelerated resampling implementation using std::simd::f32x8
///
/// Processes 8 samples at a time using SIMD instructions for better performance.
/// Falls back to scalar processing for remaining samples.
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[doc(hidden)]
#[cfg_attr(feature = "bench", allow(unused))]
pub fn resample_buffer_simd(buffer: &mut AudioBuffer<f32>, target_rate: u32) -> Result<()> {
    use std::simd::prelude::*;

    let ratio = f64::from(target_rate) / f64::from(buffer.sample_rate);
    log::debug!(
        "SIMD resampling from {} Hz to {} Hz (ratio: {:.3})",
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

    // Process samples in SIMD chunks for better performance
    const SIMD_LANES: usize = 8;
    let samples_per_chunk = SIMD_LANES / channels_usize.max(1);
    let simd_chunks = output_samples / samples_per_chunk;
    let remainder_start = simd_chunks * samples_per_chunk;

    // SIMD processing for bulk samples
    for chunk_idx in 0..simd_chunks {
        let base_sample = chunk_idx * samples_per_chunk;

        // Pre-calculate positions for this chunk
        let mut positions = [0.0f64; SIMD_LANES];
        let mut pos_fracs = [0.0f32; SIMD_LANES];
        let mut pos0s = [0usize; SIMD_LANES];
        let mut pos1s = [0usize; SIMD_LANES];

        for i in 0..samples_per_chunk {
            let sample_idx = base_sample + i;
            let i_f64 = safe_usize_to_f64_audio(sample_idx);
            let pos_f64 = i_f64 * f64::from(buffer.sample_rate) / f64::from(target_rate);

            let pos_floor = pos_f64.floor();
            let pos_frac = pos_f64 - pos_floor;

            for c in 0..channels_usize {
                let vec_idx = i * channels_usize + c;
                if vec_idx < SIMD_LANES {
                    positions[vec_idx] = pos_f64;
                    pos_fracs[vec_idx] = pos_frac as f32;
                    pos0s[vec_idx] = pos_floor as usize;
                    pos1s[vec_idx] =
                        (pos_floor as usize + 1).min((input_samples_f64 - 1.0) as usize);
                }
            }
        }

        // Gather samples for SIMD interpolation
        let mut sample0_data = [0.0f32; SIMD_LANES];
        let mut sample1_data = [0.0f32; SIMD_LANES];

        for i in 0..samples_per_chunk {
            for c in 0..channels_usize {
                let vec_idx = i * channels_usize + c;
                if vec_idx < SIMD_LANES {
                    let pos0 = pos0s[vec_idx];
                    let pos1 = pos1s[vec_idx];

                    let idx0 = pos0 * channels_usize + c;
                    let idx1 = pos1 * channels_usize + c;

                    // Get sample values with bounds checking
                    sample0_data[vec_idx] = if idx0 < buffer.data.len() {
                        buffer.data[idx0]
                    } else if !buffer.data.is_empty() {
                        *buffer.data.last().unwrap()
                    } else {
                        0.0
                    };

                    sample1_data[vec_idx] = if idx1 < buffer.data.len() {
                        buffer.data[idx1]
                    } else if !buffer.data.is_empty() {
                        *buffer.data.last().unwrap()
                    } else {
                        0.0
                    };
                }
            }
        }

        // SIMD linear interpolation: sample0 + (sample1 - sample0) * pos_frac
        let sample0_vec = f32x8::from_array(sample0_data);
        let sample1_vec = f32x8::from_array(sample1_data);
        let pos_frac_vec = f32x8::from_array(pos_fracs);

        let diff_vec = sample1_vec - sample0_vec;
        let interpolated_vec = sample0_vec + diff_vec * pos_frac_vec;

        // Store results
        let results = interpolated_vec.to_array();
        for i in 0..samples_per_chunk {
            for c in 0..channels_usize {
                let vec_idx = i * channels_usize + c;
                if vec_idx < SIMD_LANES {
                    resampled_data.push(results[vec_idx]);
                }
            }
        }
    }

    // Handle remaining samples with scalar processing
    for i in remainder_start..output_samples {
        let i_f64 = safe_usize_to_f64_audio(i);
        let pos_f64 = i_f64 * f64::from(buffer.sample_rate) / f64::from(target_rate);

        let pos_floor = pos_f64.floor();
        let pos_frac = pos_f64 - pos_floor;

        let pos0 = pos_floor as usize;
        let pos1 = (pos_floor as usize + 1).min((input_samples_f64 - 1.0) as usize);

        for c in 0..channels_usize {
            let idx0 = pos0 * channels_usize + c;
            let idx1 = pos1 * channels_usize + c;

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

            let interpolated = sample0 + (sample1 - sample0) * pos_frac as f32;
            resampled_data.push(interpolated);
        }
    }

    // Update buffer with resampled data
    buffer.data = resampled_data;
    buffer.sample_rate = target_rate;

    Ok(())
}

/// Fallback SIMD implementation when SIMD is not available
#[cfg(not(all(feature = "simd", target_arch = "x86_64")))]
#[doc(hidden)]
#[cfg_attr(feature = "bench", allow(unused))]
pub fn resample_buffer_simd(buffer: &mut AudioBuffer<f32>, target_rate: u32) -> Result<()> {
    // Fallback to scalar implementation when SIMD is not available
    super::scalar::resample_buffer_scalar(buffer, target_rate)
}
