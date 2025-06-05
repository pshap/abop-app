# Error Context Improvements

## Current State
The error types provide good domain separation but could benefit from enhanced context.

## Suggested Improvements

### 1. Enhanced Error Context
```rust
// In error.rs - Add more context to errors
#[derive(Debug, Error)]
pub enum CastError {
    #[error("Value is not finite: {value} (context: {context})")]
    NotFinite { value: f64, context: String },
    
    #[error("Negative value not allowed: {value} in {operation}")]
    NegativeValue { value: String, operation: String },
    
    #[error("Value {value} exceeds maximum {max} for {target_type}")]
    ValueTooLarge { value: String, max: String, target_type: String },
}
```

### 2. Error Recovery Patterns
```rust
// Add error recovery utilities
impl DomainCastError {
    /// Attempt to recover from overflow by clamping
    pub fn try_recover_with_clamp<T>(&self, clamp_value: T) -> Option<T> {
        match self {
            DomainCastError::Audio(AudioCastError::SampleCountOutOfRange(_)) => Some(clamp_value),
            _ => None,
        }
    }
    
    /// Get suggested alternative approach for failed conversions
    pub fn suggest_alternative(&self) -> Option<String> {
        match self {
            DomainCastError::Audio(AudioCastError::InvalidSampleRate(rate)) => {
                Some(format!("Consider using a standard sample rate like 44100 Hz instead of {} Hz", rate))
            }
            _ => None,
        }
    }
}
```

### 3. Error Aggregation for Batch Operations
```rust
#[derive(Debug, Error)]
pub enum BatchCastError {
    #[error("Multiple conversion errors occurred")]
    Multiple(Vec<DomainCastError>),
    
    #[error("Fatal error stopped batch processing: {0}")]
    Fatal(DomainCastError),
}
```
