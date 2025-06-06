//! Implementation of the CastingBuilder

use super::config::*;
use crate::utils::casting::error::DomainCastError;

/// Builder for configurable casting operations
#[derive(Debug, Clone)]
pub struct CastingBuilder {
    precision_mode: PrecisionMode,
    overflow_behavior: OverflowBehavior,
    rounding_mode: RoundingMode,
    validation_level: ValidationLevel,
}

impl Default for CastingBuilder {
    fn default() -> Self {
        Self {
            precision_mode: PrecisionMode::Strict,
            overflow_behavior: OverflowBehavior::Fail,
            rounding_mode: RoundingMode::Nearest,
            validation_level: ValidationLevel::Full,
        }
    }
}

impl CastingBuilder {
    /// Create a new casting builder with default settings
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a builder optimized for audio processing
    #[must_use]
    pub const fn for_audio() -> Self {
        Self {
            precision_mode: PrecisionMode::Tolerant { epsilon: 1e-6 },
            overflow_behavior: OverflowBehavior::Clamp,
            rounding_mode: RoundingMode::Nearest,
            validation_level: ValidationLevel::Basic,
        }
    }

    /// Create a builder optimized for UI calculations
    #[must_use]
    pub const fn for_ui() -> Self {
        Self {
            precision_mode: PrecisionMode::Adaptive,
            overflow_behavior: OverflowBehavior::Clamp,
            rounding_mode: RoundingMode::Nearest,
            validation_level: ValidationLevel::Basic,
        }
    }

    /// Create a builder optimized for database operations
    #[must_use]
    pub const fn for_database() -> Self {
        Self {
            precision_mode: PrecisionMode::Strict,
            overflow_behavior: OverflowBehavior::Fail,
            rounding_mode: RoundingMode::Truncate,
            validation_level: ValidationLevel::Full,
        }
    }

    /// Create a builder optimized for audiobook processing
    #[must_use]
    pub const fn for_audiobook_processing() -> Self {
        Self {
            precision_mode: PrecisionMode::Tolerant { epsilon: 1e-3 },
            overflow_behavior: OverflowBehavior::Clamp,
            rounding_mode: RoundingMode::Nearest,
            validation_level: ValidationLevel::Basic,
        }
    }

    /// Create a builder optimized for voice recognition
    #[must_use]
    pub const fn for_voice_recognition() -> Self {
        Self {
            precision_mode: PrecisionMode::Strict,
            overflow_behavior: OverflowBehavior::Fail,
            rounding_mode: RoundingMode::Nearest,
            validation_level: ValidationLevel::Full,
        }
    }

    /// Create a builder optimized for real-time audio processing
    #[must_use]
    pub const fn for_realtime_audio() -> Self {
        Self {
            precision_mode: PrecisionMode::Adaptive,
            overflow_behavior: OverflowBehavior::Clamp,
            rounding_mode: RoundingMode::Truncate,
            validation_level: ValidationLevel::None, // Skip validation for performance
        }
    }

    /// Create a builder optimized for podcast processing
    #[must_use]
    pub const fn for_podcast_processing() -> Self {
        Self {
            precision_mode: PrecisionMode::Tolerant { epsilon: 1e-4 },
            overflow_behavior: OverflowBehavior::Clamp,
            rounding_mode: RoundingMode::Nearest,
            validation_level: ValidationLevel::Basic,
        }
    }

    /// Create a builder optimized for music mastering
    #[must_use]
    pub const fn for_music_mastering() -> Self {
        Self {
            precision_mode: PrecisionMode::Strict,
            overflow_behavior: OverflowBehavior::Fail,
            rounding_mode: RoundingMode::Nearest,
            validation_level: ValidationLevel::Full,
        }
    }

    /// Set precision handling mode
    #[must_use]
    pub const fn with_precision(mut self, mode: PrecisionMode) -> Self {
        self.precision_mode = mode;
        self
    }

    /// Set overflow behavior
    #[must_use]
    pub const fn with_overflow_behavior(mut self, behavior: OverflowBehavior) -> Self {
        self.overflow_behavior = behavior;
        self
    }

    /// Set rounding mode
    #[must_use]
    pub const fn with_rounding(mut self, mode: RoundingMode) -> Self {
        self.rounding_mode = mode;
        self
    }

    /// Set validation level
    #[must_use]
    pub const fn with_validation(mut self, level: ValidationLevel) -> Self {
        self.validation_level = level;
        self
    }

    /// Execute float to integer conversion with configured settings
    pub fn float_to_int<T>(&self, value: f64) -> Result<T, DomainCastError>
    where
        T: TryFrom<i64> + std::fmt::Display,
        T::Error: std::fmt::Debug,
    {
        use crate::utils::casting::error::CastError;

        // Validation phase
        if self.validation_level != ValidationLevel::None {
            if !value.is_finite() {
                return Err(CastError::NotFinite(value).into());
            }

            if self.validation_level == ValidationLevel::Full && value < 0.0 {
                return Err(CastError::NegativeValue(value.to_string()).into());
            }
        }

        // Apply rounding
        let rounded = match self.rounding_mode {
            RoundingMode::Nearest => value.round(),
            RoundingMode::Floor => value.floor(),
            RoundingMode::Ceiling => value.ceil(),
            RoundingMode::Truncate => value.trunc(),
        };

        // Check precision loss based on mode
        match self.precision_mode {
            PrecisionMode::Strict => {
                if (rounded - value).abs() > f64::EPSILON {
                    return Err(CastError::PrecisionLoss(value).into());
                }
            }
            PrecisionMode::Tolerant { epsilon } => {
                if (rounded - value).abs() > epsilon {
                    return Err(CastError::PrecisionLoss(value).into());
                }
            }
            PrecisionMode::Adaptive => {
                // Adaptive mode allows reasonable precision loss for the target type
                // For integer targets, we allow rounding but check for reasonable bounds
            }
        }

        // Handle overflow
        let target_max = i64::MAX as f64;
        let target_min = i64::MIN as f64;

        let final_value = if rounded > target_max || rounded < target_min {
            match self.overflow_behavior {
                OverflowBehavior::Fail => {
                    return Err(CastError::ValueTooLarge(
                        rounded.to_string(),
                        format!("range {target_min} to {target_max}"),
                    )
                    .into());
                }
                OverflowBehavior::Clamp => rounded.clamp(target_min, target_max),
                OverflowBehavior::Saturate => {
                    if rounded > target_max {
                        target_max
                    } else {
                        target_min
                    }
                }
            }
        } else {
            rounded
        };

        // Final conversion
        let as_i64 = final_value as i64;
        T::try_from(as_i64).map_err(|_| {
            CastError::ValueTooLarge(
                final_value.to_string(),
                std::any::type_name::<T>().to_string(),
            )
            .into()
        })
    }

    /// Execute integer to integer conversion with configured settings
    ///
    /// # Arguments
    ///
    /// * `value` - The source integer value to convert
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - The converted value if successful
    /// * `Err(DomainCastError)` - If the conversion fails due to overflow or validation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use abop_core::utils::casting::CastingBuilder;
    ///
    /// let builder = CastingBuilder::new();
    /// let result = builder.int_to_int::<i32, u32>(42);
    /// assert!(result.is_ok());
    /// ```
    pub fn int_to_int<T, U>(&self, value: T) -> Result<U, DomainCastError>
    where
        T: Into<i64> + Copy + std::fmt::Display,
        U: TryFrom<i64> + std::fmt::Display,
        U::Error: std::fmt::Debug,
    {
        use crate::utils::casting::error::CastError;

        let value_i64 = value.into();

        // Validation
        if self.validation_level == ValidationLevel::Full && value_i64 < 0 {
            return Err(CastError::NegativeValue(value_i64.to_string()).into());
        }

        // Direct conversion attempt
        U::try_from(value_i64).map_err(|_| {
            match self.overflow_behavior {
                OverflowBehavior::Fail => CastError::ValueTooLarge(
                    value_i64.to_string(),
                    std::any::type_name::<U>().to_string(),
                )
                .into(),
                OverflowBehavior::Clamp | OverflowBehavior::Saturate => {
                    // For now, just return the error - could implement clamping logic here
                    CastError::ValueTooLarge(
                        value_i64.to_string(),
                        std::any::type_name::<U>().to_string(),
                    )
                    .into()
                }
            }
        })
    }

    /// Convert sample count between different sample rates
    ///
    /// # Arguments
    ///
    /// * `from_rate` - Source sample rate in Hz
    /// * `to_rate` - Target sample rate in Hz
    /// * `samples` - Number of samples to convert
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The converted sample count
    /// * `Err(DomainCastError)` - If the conversion fails due to overflow or validation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use abop_core::utils::casting::CastingBuilder;
    ///
    /// let builder = CastingBuilder::for_audio();
    /// let result = builder.convert_sample_rate(44100, 48000, 1000);
    /// // Should successfully convert sample count between sample rates
    /// match result {
    ///     Ok(_) => println!("Conversion successful"),
    ///     Err(e) => println!("Conversion failed: {}", e),
    /// }
    /// ```
    pub fn convert_sample_rate(
        &self,
        from_rate: u32,
        to_rate: u32,
        samples: usize,
    ) -> Result<usize, DomainCastError> {
        use crate::utils::casting::error::CastError;

        if from_rate == 0 || to_rate == 0 {
            return Err(CastError::InvalidInput("Sample rate cannot be zero".to_string()).into());
        }

        let ratio = f64::from(to_rate) / f64::from(from_rate);
        let new_samples = samples as f64 * ratio;

        // Use configured precision and rounding
        self.float_to_int(new_samples)
    }

    /// Convert time duration to sample count
    ///
    /// # Arguments
    ///
    /// * `time_secs` - Duration in seconds
    /// * `sample_rate` - Sample rate in Hz
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The number of samples
    /// * `Err(DomainCastError)` - If the conversion fails due to overflow or validation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use abop_core::utils::casting::CastingBuilder;
    ///
    /// let builder = CastingBuilder::for_audio();
    /// let result = builder.time_to_samples(1.0, 44100);
    /// assert_eq!(result.unwrap(), 44100);
    /// ```
    pub fn time_to_samples(
        &self,
        time_secs: f32,
        sample_rate: u32,
    ) -> Result<usize, DomainCastError> {
        use crate::utils::casting::error::CastError;

        if sample_rate == 0 {
            return Err(CastError::InvalidInput("Sample rate cannot be zero".to_string()).into());
        }

        let samples = f64::from(time_secs) * f64::from(sample_rate);
        self.float_to_int(samples)
    }

    /// Convert audio sample value between different bit depths
    ///
    /// # Arguments
    ///
    /// * `value` - The audio sample value to convert
    /// * `from_bits` - Source bit depth (e.g., 16, 24, 32)
    /// * `to_bits` - Target bit depth
    ///
    /// # Returns
    ///
    /// * `Ok(i64)` - The converted sample value
    /// * `Err(DomainCastError)` - If the conversion fails due to overflow or validation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use abop_core::utils::casting::CastingBuilder;
    ///
    /// let builder = CastingBuilder::for_audio();
    /// let result = builder.convert_audio_value(32767.0, 16, 24);
    /// assert!(result.is_ok());
    /// ```
    pub fn convert_audio_value(
        &self,
        value: f64,
        from_bits: u8,
        to_bits: u8,
    ) -> Result<i64, DomainCastError> {
        use crate::utils::casting::error::CastError;

        if from_bits == 0 || to_bits == 0 || from_bits > 63 || to_bits > 63 {
            return Err(
                CastError::InvalidInput("Bit depth must be between 1 and 63".to_string()).into(),
            );
        }

        // Calculate scaling factor based on bit depths
        let from_max = (1i64 << (from_bits - 1)) - 1;
        let to_max = (1i64 << (to_bits - 1)) - 1;

        let scaled = value * (to_max as f64) / (from_max as f64);
        self.float_to_int(scaled)
    }

    /// Convert logical UI units to physical pixels
    ///
    /// # Arguments
    ///
    /// * `logical` - Logical UI units (e.g., points)
    /// * `scale_factor` - Display scale factor (e.g., 1.0 for 100% DPI)
    ///
    /// # Returns
    ///
    /// * `Ok(u16)` - The physical pixel value
    /// * `Err(DomainCastError)` - If the conversion fails due to overflow or validation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use abop_core::utils::casting::CastingBuilder;
    ///
    /// let builder = CastingBuilder::for_ui();
    /// let result = builder.logical_to_physical(100.0, 1.5);
    /// assert_eq!(result.unwrap(), 150);
    /// ```
    pub fn logical_to_physical(
        &self,
        logical: f32,
        scale_factor: f32,
    ) -> Result<u16, DomainCastError> {
        use crate::utils::casting::error::CastError;

        if !logical.is_finite() || !scale_factor.is_finite() {
            return Err(CastError::NotFinite(if !logical.is_finite() {
                f64::from(logical)
            } else {
                f64::from(scale_factor)
            })
            .into());
        }

        let physical = f64::from(logical) * f64::from(scale_factor);
        self.float_to_int(physical)
    }

    /// Convert physical pixels to logical UI units
    ///
    /// # Arguments
    ///
    /// * `physical` - Physical pixel value
    /// * `scale_factor` - Display scale factor (e.g., 1.0 for 100% DPI)
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The logical UI units
    /// * `Err(DomainCastError)` - If the conversion fails due to overflow or validation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use abop_core::utils::casting::CastingBuilder;
    ///
    /// let builder = CastingBuilder::for_ui();
    /// let result = builder.physical_to_logical(150, 1.5);
    /// assert_eq!(result.unwrap(), 100.0);
    /// ```
    pub fn physical_to_logical(
        &self,
        physical: u16,
        scale_factor: f32,
    ) -> Result<f32, DomainCastError> {
        use crate::utils::casting::error::CastError;

        if scale_factor == 0.0 {
            return Err(CastError::InvalidInput("Scale factor cannot be zero".to_string()).into());
        }

        let logical = f64::from(physical) / f64::from(scale_factor);

        // For UI, we typically allow some precision loss
        if let PrecisionMode::Strict = self.precision_mode
            && (logical > f64::from(f32::MAX) || logical < f64::from(f32::MIN))
        {
            return Err(CastError::ValueTooLarge(
                logical.to_string(),
                format!("f32 (range: {} to {})", f32::MIN, f32::MAX),
            )
            .into());
        }

        Ok(logical as f32)
    }
}
