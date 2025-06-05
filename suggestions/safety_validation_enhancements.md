# Safety Checks and Validation Enhancements

## Current State
Good basic safety checks exist, but could be enhanced with more comprehensive validation.

## Suggested Improvements

### 1. Compile-Time Safety Guarantees
```rust
// Use phantom types to encode safety at the type level
use std::marker::PhantomData;

pub struct Validated<T, V> {
    value: T,
    _validation: PhantomData<V>,
}

pub struct PositiveValidation;
pub struct FiniteValidation;
pub struct RangeValidation<const MIN: i64, const MAX: i64>;

impl<T> Validated<T, PositiveValidation> {
    pub fn new_positive(value: T) -> Result<Self, CastError> 
    where 
        T: PartialOrd + Default + Copy + std::fmt::Display
    {
        if value < T::default() {
            return Err(CastError::NegativeValue(value.to_string()));
        }
        Ok(Self { value, _validation: PhantomData })
    }
    
    pub fn get(&self) -> T {
        self.value  // Safe to unwrap since we validated
    }
}

impl Validated<f64, FiniteValidation> {
    pub fn new_finite(value: f64) -> Result<Self, CastError> {
        if !value.is_finite() {
            return Err(CastError::NotFinite(value));
        }
        Ok(Self { value, _validation: PhantomData })
    }
}

// Usage ensures validation at compile time
fn process_audio_samples(count: Validated<usize, PositiveValidation>) -> Result<(), AudioError> {
    let sample_count = count.get();  // No runtime check needed
    // ... processing logic
    Ok(())
}
```

### 2. Runtime Validation Framework
```rust
// Comprehensive validation framework
pub trait Validator<T> {
    type Error;
    
    fn validate(&self, value: &T) -> Result<(), Self::Error>;
    fn can_validate(&self, value: &T) -> bool;
    fn suggest_fix(&self, value: &T) -> Option<T>;
}

pub struct RangeValidator<T> {
    min: T,
    max: T,
    name: &'static str,
}

impl<T> Validator<T> for RangeValidator<T>
where 
    T: PartialOrd + Copy + std::fmt::Display
{
    type Error = CastError;
    
    fn validate(&self, value: &T) -> Result<(), Self::Error> {
        if *value < self.min || *value > self.max {
            return Err(CastError::ValueTooLarge(
                value.to_string(),
                format!("{} to {} for {}", self.min, self.max, self.name)
            ));
        }
        Ok(())
    }
    
    fn can_validate(&self, value: &T) -> bool {
        *value >= self.min && *value <= self.max
    }
    
    fn suggest_fix(&self, value: &T) -> Option<T> {
        Some(if *value < self.min {
            self.min
        } else if *value > self.max {
            self.max
        } else {
            *value
        })
    }
}

// Predefined validators
pub const SAMPLE_RATE_VALIDATOR: RangeValidator<u32> = RangeValidator {
    min: 1,
    max: 192000,
    name: "sample rate",
};

pub const UI_PIXEL_VALIDATOR: RangeValidator<f32> = RangeValidator {
    min: 0.0,
    max: 65535.0,
    name: "UI pixels",
};
```

### 3. Defensive Programming Patterns
```rust
// Add defensive checks with debug assertions
pub fn safe_duration_to_samples_defensive(
    duration_secs: f32, 
    sample_rate: u32
) -> Result<usize, DomainCastError> {
    // Pre-conditions
    debug_assert!(duration_secs >= 0.0, "Duration must be non-negative");
    debug_assert!(sample_rate > 0, "Sample rate must be positive");
    debug_assert!(sample_rate <= 192000, "Sample rate exceeds reasonable maximum");
    
    // Input validation
    if !duration_secs.is_finite() {
        return Err(AudioCastError::InvalidDuration(duration_secs).into());
    }
    
    if duration_secs < 0.0 {
        return Err(CastError::NegativeValue(duration_secs.to_string()).into());
    }
    
    if sample_rate == 0 {
        return Err(AudioCastError::InvalidSampleRate(sample_rate).into());
    }
    
    // Overflow protection
    let max_safe_duration = (usize::MAX as f64 / sample_rate as f64) as f32;
    if duration_secs > max_safe_duration {
        return Err(AudioCastError::SampleRateOverflow {
            duration: duration_secs as f64,
            sample_rate,
        }.into());
    }
    
    // Calculation with overflow detection
    let samples_f64 = duration_secs as f64 * sample_rate as f64;
    
    // Post-condition checks
    debug_assert!(samples_f64.is_finite(), "Sample calculation must be finite");
    debug_assert!(samples_f64 >= 0.0, "Sample count must be non-negative");
    
    let samples = samples_f64.round() as usize;
    
    // Final validation
    debug_assert!(samples <= usize::MAX, "Sample count exceeds usize maximum");
    
    Ok(samples)
}
```

### 4. Sanitization and Input Cleaning
```rust
// Input sanitization utilities
pub struct InputSanitizer;

impl InputSanitizer {
    /// Clean floating point input by handling edge cases
    pub fn sanitize_float(value: f64) -> Result<f64, CastError> {
        match value.classify() {
            std::num::FpCategory::Nan => Err(CastError::NotFinite(value)),
            std::num::FpCategory::Infinite => Err(CastError::NotFinite(value)),
            std::num::FpCategory::Subnormal => {
                warn!("Subnormal float value {} detected, converting to zero", value);
                Ok(0.0)
            }
            _ => Ok(value)
        }
    }
    
    /// Clean string input for numeric conversion
    pub fn sanitize_numeric_string(input: &str) -> Result<String, CastError> {
        let cleaned = input
            .trim()
            .replace(',', "")  // Remove thousands separators
            .replace('\u{00A0}', "")  // Remove non-breaking spaces
            .to_lowercase();
        
        // Check for obviously invalid patterns
        if cleaned.is_empty() {
            return Err(CastError::NegativeValue("empty string".to_string()));
        }
        
        if cleaned.matches('.').count() > 1 {
            return Err(CastError::PrecisionLoss(0.0));
        }
        
        Ok(cleaned)
    }
    
    /// Sanitize database count with context
    pub fn sanitize_db_count(count: i64, context: &str) -> Result<usize, DomainCastError> {
        // Log suspicious values for audit
        if count < 0 {
            warn!("Negative database count {} in context: {}", count, context);
        }
        
        if count > 1_000_000_000 {  // 1 billion seems large for most use cases
            warn!("Very large database count {} in context: {}", count, context);
        }
        
        validate_db_count(count).map_err(|e| {
            error!("Database count validation failed for {} in {}: {:?}", count, context, e);
            e
        })
    }
}
```

### 5. Audit Trail and Monitoring
```rust
// Add monitoring for conversion operations
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

pub struct ConversionMetrics {
    pub total_conversions: AtomicU64,
    pub failed_conversions: AtomicU64,
    pub overflow_events: AtomicU64,
    pub precision_loss_events: AtomicU64,
    pub last_reset: std::sync::Mutex<Instant>,
}

impl ConversionMetrics {
    pub const fn new() -> Self {
        Self {
            total_conversions: AtomicU64::new(0),
            failed_conversions: AtomicU64::new(0),
            overflow_events: AtomicU64::new(0),
            precision_loss_events: AtomicU64::new(0),
            last_reset: std::sync::Mutex::new(Instant::now()),
        }
    }
    
    pub fn record_conversion(&self, result: &Result<(), DomainCastError>) {
        self.total_conversions.fetch_add(1, Ordering::Relaxed);
        
        if let Err(error) = result {
            self.failed_conversions.fetch_add(1, Ordering::Relaxed);
            
            match error {
                DomainCastError::Generic(CastError::ValueTooLarge(_, _)) => {
                    self.overflow_events.fetch_add(1, Ordering::Relaxed);
                }
                DomainCastError::Generic(CastError::PrecisionLoss(_)) => {
                    self.precision_loss_events.fetch_add(1, Ordering::Relaxed);
                }
                _ => {}
            }
        }
    }
    
    pub fn get_stats(&self) -> ConversionStats {
        ConversionStats {
            total: self.total_conversions.load(Ordering::Relaxed),
            failed: self.failed_conversions.load(Ordering::Relaxed),
            overflows: self.overflow_events.load(Ordering::Relaxed),
            precision_losses: self.precision_loss_events.load(Ordering::Relaxed),
        }
    }
}

pub struct ConversionStats {
    pub total: u64,
    pub failed: u64,
    pub overflows: u64,
    pub precision_losses: u64,
}

// Global metrics instance
static CONVERSION_METRICS: ConversionMetrics = ConversionMetrics::new();

pub fn get_conversion_metrics() -> &'static ConversionMetrics {
    &CONVERSION_METRICS
}
```
