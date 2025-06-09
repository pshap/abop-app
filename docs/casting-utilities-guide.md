# ABOP Casting Utilities Guide

## Overview

The ABOP casting utilities provide safe, configurable numeric type conversions with comprehensive error handling. The system is designed to prevent common casting errors like overflow, precision loss, and invalid conversions.

## Quick Start

### Basic Usage

```rust
use abop_core::utils::casting::CastingBuilder;

// Create a builder for audio processing
let builder = CastingBuilder::for_audio();

// Convert sample count to time
let duration = builder.time_to_samples(1.0, 44100).unwrap();
assert_eq!(duration, 44100);

// Convert with error handling
match builder.float_to_int::<i32>(42.7) {
    Ok(value) => println!("Converted: {}", value),
    Err(e) => eprintln!("Conversion failed: {}", e),
}
```

### Domain-Specific Utilities

```rust
use abop_core::utils::casting::domain::{audio, db, ui};

// Audio conversions
let samples = audio::duration_to_samples(1.0, 44100)?;
let duration = audio::samples_to_duration(44100, 44100)?;

// Database conversions
let count = db::safe_db_count_to_usize(42);
let validated = db::validate_db_count(-1)?; // Returns error

// UI conversions
let pixels = ui::logical_to_physical(100.0, 2.0)?;
```

## Configuration Modes

### Precision Modes

- **Strict**: No precision loss allowed (best for financial/exact calculations)
- **Tolerant**: Allow minor precision loss within epsilon
- **Adaptive**: Context-aware precision handling

```rust
let strict_builder = CastingBuilder::new()
    .with_precision(PrecisionMode::Strict);

let tolerant_builder = CastingBuilder::new()
    .with_precision(PrecisionMode::Tolerant { epsilon: 1e-6 });
```

### Overflow Behavior

- **Fail**: Return error on overflow
- **Clamp**: Clamp to target type bounds
- **Saturate**: Use type's MIN/MAX values

```rust
let builder = CastingBuilder::new()
    .with_overflow_behavior(OverflowBehavior::Clamp);
```

### Rounding Modes

- **Nearest**: Round to nearest integer
- **Floor**: Always round down
- **Ceiling**: Always round up
- **Truncate**: Remove fractional part

```rust
let builder = CastingBuilder::new()
    .with_rounding(RoundingMode::Nearest);
```

## Presets

The system includes optimized presets for common use cases:

```rust
// Audio processing (tolerant precision, clamp overflow)
let audio_builder = CastingBuilder::for_audio();

// Database operations (strict precision, fail on overflow)
let db_builder = CastingBuilder::for_database();

// UI calculations (adaptive precision, clamp overflow)
let ui_builder = CastingBuilder::for_ui();

// Real-time audio (fast, minimal validation)
let realtime_builder = CastingBuilder::for_realtime_audio();
```

## Error Handling

All conversions return `Result<T, DomainCastError>` with detailed error information:

```rust
use abop_core::utils::casting::error::DomainCastError;

match builder.float_to_int::<i32>(value) {
    Ok(result) => println!("Success: {}", result),
    Err(DomainCastError::Audio(audio_err)) => {
        eprintln!("Audio conversion error: {}", audio_err);
    }
    Err(DomainCastError::Generic(cast_err)) => {
        eprintln!("Generic casting error: {}", cast_err);
    }
    // ... handle other error types
}
```

## Common Patterns

### Audio Sample Calculations

```rust
let builder = CastingBuilder::for_audio();

// Convert time to samples
let samples = builder.time_to_samples(2.5, 48000)?;

// Convert between sample rates
let new_samples = builder.convert_sample_rate(44100, 48000, 1000)?;

// Convert audio bit depths
let converted = builder.convert_audio_value(32767.0, 16, 24)?;
```

### Database Operations

```rust
// Safe count conversion
let count = db::safe_db_count_to_usize(db_result);

// With validation
let validated_count = db::validate_db_count(db_result)?;

// Check if value fits in usize
if db::can_fit_in_usize(large_value) {
    let safe_value = large_value as usize;
}
```

### UI Calculations

```rust
let builder = CastingBuilder::for_ui();

// Convert logical to physical pixels
let physical = builder.logical_to_physical(100.0, scale_factor)?;

// Convert physical to logical pixels
let logical = builder.physical_to_logical(200, scale_factor)?;
```

## Best Practices

1. **Choose the Right Preset**: Use domain-specific presets when available
2. **Handle Errors Appropriately**: Don't unwrap conversion results in production code
3. **Validate Input**: Use validation functions for external data
4. **Consider Performance**: Use relaxed settings for performance-critical code
5. **Test Edge Cases**: Test with boundary values and invalid inputs

## Migration from Direct Casting

Replace direct casts with safe conversions:

```rust
// Old (unsafe)
let value = some_float as i32;

// New (safe)
let value = CastingBuilder::for_audio()
    .float_to_int::<i32>(some_float)?;

// Or use convenience functions
let value = audio::f64_to_samples(some_float)?;
```

## Performance Considerations

- Validation overhead is typically < 1% of conversion time
- Use `ValidationLevel::None` for performance-critical sections
- Presets are optimized for their use cases
- Batch conversions when possible

## Testing

The casting utilities include comprehensive tests. When writing your own tests:

```rust
#[test]
fn test_my_conversion() {
    let builder = CastingBuilder::for_audio();
    
    // Test success case
    assert_eq!(builder.float_to_int::<i32>(42.0).unwrap(), 42);
    
    // Test error case
    assert!(builder.float_to_int::<i32>(f64::NAN).is_err());
}
```

## Troubleshooting

### Common Errors

1. **PrecisionLoss**: Occurs with strict precision mode and fractional values
   - Solution: Use tolerant mode or round the input

2. **ValueTooLarge**: Value exceeds target type bounds
   - Solution: Use clamp overflow behavior or validate input range

3. **NotFinite**: Input is NaN or infinity
   - Solution: Validate input before conversion

4. **InvalidInput**: Domain-specific validation failed
   - Solution: Check input against domain constraints

### Debug Tips

1. Enable debug logging to see conversion details
2. Use `ValidationLevel::Full` during development
3. Check error messages for specific failure reasons
4. Test with boundary values and edge cases
