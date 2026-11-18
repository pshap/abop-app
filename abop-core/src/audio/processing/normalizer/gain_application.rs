//! Gain application utilities for audio normalization

/// SIMD-optimized gain application with limiting
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
pub fn apply_gain_with_limiting_simd(data: &mut [f32], gain: f32) {
    use std::simd::prelude::*;

    if data.is_empty() {
        return;
    }

    const SIMD_LANES: usize = 8;
    let chunks = data.len() / SIMD_LANES;
    let remainder_start = chunks * SIMD_LANES;

    let gain_vec = f32x8::splat(gain);
    let min_vec = f32x8::splat(-1.0);
    let max_vec = f32x8::splat(1.0);

    // SIMD processing for bulk gain application
    for chunk_idx in 0..chunks {
        let start = chunk_idx * SIMD_LANES;
        let chunk_data = &mut data[start..start + SIMD_LANES];

        // Load 8 samples into SIMD vector
        let sample_vec = f32x8::from_slice(chunk_data);

        // Apply gain and clamp to [-1.0, 1.0]
        let amplified_vec = sample_vec * gain_vec;
        let limited_vec = amplified_vec.simd_clamp(min_vec, max_vec);

        // Store results back
        limited_vec.copy_to_slice(chunk_data);
    }

    // Handle remainder samples with scalar processing
    for sample in &mut data[remainder_start..] {
        let amplified = *sample * gain;
        *sample = amplified.clamp(-1.0, 1.0);
    }
}

/// Fallback gain application for non-SIMD targets
#[cfg(not(all(feature = "simd", target_arch = "x86_64")))]
pub fn apply_gain_with_limiting_simd(data: &mut [f32], gain: f32) {
    for sample in data {
        let amplified = *sample * gain;
        *sample = amplified.clamp(-1.0, 1.0);
    }
}
