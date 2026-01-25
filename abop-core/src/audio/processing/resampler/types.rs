//! Type definitions for resampler function selection

/// Function type selection for resampling implementations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResampleBufferFn {
    /// Scalar implementation (fallback)
    Scalar,
    /// SIMD-accelerated implementation
    Simd,
}

impl Default for ResampleBufferFn {
    fn default() -> Self {
        if cfg!(feature = "simd") {
            ResampleBufferFn::Simd
        } else {
            ResampleBufferFn::Scalar
        }
    }
}
