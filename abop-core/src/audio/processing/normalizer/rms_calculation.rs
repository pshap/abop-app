//! RMS calculation utilities for audio normalization

/// SIMD-optimized RMS calculation for audio data
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
pub fn calculate_rms_simd(data: &[f32]) -> f32 {
    use std::simd::prelude::*;

    if data.is_empty() {
        return 0.0;
    }

    const SIMD_LANES: usize = 8;
    let chunks = data.len() / SIMD_LANES;
    let remainder_start = chunks * SIMD_LANES;

    // SIMD processing for bulk squares calculation
    let mut sum_vec = f32x8::splat(0.0);

    for chunk_idx in 0..chunks {
        let start = chunk_idx * SIMD_LANES;
        let chunk_data = &data[start..start + SIMD_LANES];

        // Load 8 samples into SIMD vector
        let sample_vec = f32x8::from_slice(chunk_data);

        // Square samples: sample_vec * sample_vec
        let squared_vec = sample_vec * sample_vec;
        sum_vec += squared_vec;
    }

    // Horizontal sum reduction across lanes
    let sum_array = sum_vec.to_array();
    let mut simd_sum = 0.0f32;
    for &val in &sum_array {
        simd_sum += val;
    }

    // Handle remainder samples with scalar processing
    let scalar_sum: f32 = data[remainder_start..].iter().map(|&s| s * s).sum();
    let total_sum = simd_sum + scalar_sum;

    // Calculate RMS with safe conversion
    #[allow(clippy::cast_precision_loss)]
    let data_len_f32 = if data.len() <= (1_usize << 24) {
        data.len() as f32
    } else {
        // For extremely large arrays, we accept potential precision loss
        data.len() as f32
    };

    (total_sum / data_len_f32).sqrt()
}

/// Fallback RMS calculation for non-SIMD targets
#[cfg(not(all(feature = "simd", target_arch = "x86_64")))]
pub fn calculate_rms_simd(data: &[f32]) -> f32 {
    calculate_rms(data)
}

/// Calculates the RMS (Root Mean Square) value of audio data
pub fn calculate_rms(data: &[f32]) -> f32 {
    if data.is_empty() {
        return 0.0;
    }

    // TODO: Implement SIMD optimization for RMS calculation
    // - Process 8 samples at a time using f32x8 vectors
    // - Use SIMD multiplication for squaring: sample_vec * sample_vec
    // - Use horizontal sum reduction to accumulate squares efficiently
    // - Should provide 4-8x speedup for RMS calculation on large buffers
    // - Handle remainder samples with scalar code
    // - Maintain exact numerical compatibility with current RMS calculation

    #[allow(clippy::cast_precision_loss)]
    let sum_squares: f32 = data.iter().map(|&s| s * s).sum();

    // Use safe conversion for sample count, with fallback for very large arrays
    #[allow(clippy::cast_possible_truncation)]
    let data_len_f32 = if data.len() <= (1_usize << 24) {
        data.len() as f32
    } else {
        // For extremely large arrays, we accept potential precision loss
        data.len() as f32
    };

    (sum_squares / data_len_f32).sqrt()
}
