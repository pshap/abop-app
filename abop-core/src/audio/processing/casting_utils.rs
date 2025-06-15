//! Audio processing casting utilities (consolidated)
//!
//! This module re-exports safe casting utilities from the unified casting module
//! and provides audio-specific convenience functions.

// Re-export from unified casting system
pub use crate::utils::casting::domain::audio::*;
pub use crate::utils::casting::{CastError, CastResult, CastingBuilder, DomainCastError};

use crate::audio::processing::error::{AudioProcessingError, Result};

/// Audio-specific error conversion utilities
pub mod error_conversion {
    use super::{AudioProcessingError, DomainCastError, Result};

    /// Convert a DomainCastError to an AudioProcessingError
    #[must_use]
    pub fn cast_to_audio_error(err: DomainCastError) -> AudioProcessingError {
        AudioProcessingError::buffer(format!("Casting error: {err}"))
    }

    /// Convert a casting result to an audio processing result
    pub fn cast_result_to_audio<T>(result: super::CastResult<T>) -> Result<T> {
        result.map_err(|e| cast_to_audio_error(e.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::casting::domain::{audio as unified_audio, db};

    #[test]
    fn test_error_conversion() {
        let cast_error = DomainCastError::Generic(CastError::Overflow);
        let audio_error = error_conversion::cast_to_audio_error(cast_error);
        assert!(audio_error.to_string().contains("Casting error"));
    }

    #[test]
    fn test_direct_unified_calls() {
        // Test direct calls to the unified system
        assert_eq!(db::safe_db_count_to_usize(-1), 0);
        assert_eq!(db::safe_db_count_to_usize(42), 42);

        let result = unified_audio::safe_usize_to_f64_audio(100);
        assert_eq!(result, 100.0);
    }
}
