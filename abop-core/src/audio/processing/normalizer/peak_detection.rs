//! Peak detection utilities for audio normalization

/// SIMD-optimized peak finding for audio data
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
pub fn find_peak_simd(data: &[f32]) -> f32 {
    use std::simd::prelude::*;

    if data.is_empty() {
        return 0.0;
    }

    const SIMD_LANES: usize = 8;
    let chunks = data.len() / SIMD_LANES;
    let remainder_start = chunks * SIMD_LANES;

    // SIMD processing for bulk data
    let mut max_vec = f32x8::splat(0.0);

    for chunk_idx in 0..chunks {
        let start = chunk_idx * SIMD_LANES;
        let chunk_data = &data[start..start + SIMD_LANES];

        // Load 8 samples into SIMD vector
        let sample_vec = f32x8::from_slice(chunk_data);

        // Take absolute value and find max
        let abs_vec = sample_vec.abs();
        max_vec = max_vec.simd_max(abs_vec);
    }

    // Horizontal reduction to find maximum across lanes
    let max_array = max_vec.to_array();
    let mut simd_max = max_array[0];
    for &val in &max_array[1..] {
        simd_max = simd_max.max(val);
    }

    // Handle remainder samples with scalar processing
    let scalar_max = data[remainder_start..]
        .iter()
        .fold(0.0f32, |max, &s| max.max(s.abs()));

    simd_max.max(scalar_max)
}

/// Fallback peak finding for non-SIMD targets
#[cfg(not(all(feature = "simd", target_arch = "x86_64")))]
pub fn find_peak_simd(data: &[f32]) -> f32 {
    data.iter().fold(0.0f32, |max, &s| max.max(s.abs()))
}
