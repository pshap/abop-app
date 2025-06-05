# Testing Strategy Improvements

## Current State
Good basic test coverage exists, but could be enhanced with property-based testing and edge case coverage.

## Suggested Improvements

### 1. Property-Based Testing
```rust
// Add to Cargo.toml:
// proptest = "1.4"

use proptest::prelude::*;

proptest! {
    #[test]
    fn test_audio_conversions_never_panic(
        duration in 0.0f32..1000.0,
        sample_rate in 1u32..192000
    ) {
        // This should never panic, only return errors
        let _ = audio::safe_duration_to_samples(duration, sample_rate);
    }
    
    #[test]
    fn test_db_count_roundtrip(count in 0i64..=isize::MAX as i64) {
        let usize_val = safe_db_count_to_usize(count);
        if usize_val > 0 {
            // Converting back should give us the same or clamped value
            let back_to_i64 = safe_usize_to_i64(usize_val).unwrap();
            prop_assert!(back_to_i64 <= count);
        }
    }
    
    #[test]
    fn test_builder_consistency(
        value in any::<f64>().prop_filter("Must be finite", |v| v.is_finite()),
        precision in prop_oneof![
            Just(PrecisionMode::Strict),
            Just(PrecisionMode::Adaptive),
            (0.0..1.0).prop_map(|e| PrecisionMode::Tolerant { epsilon: e })
        ]
    ) {
        let builder = CastingBuilder::new().with_precision(precision);
        // Test that the builder produces consistent results
        let result1 = builder.float_to_int::<i32>(value);
        let result2 = builder.float_to_int::<i32>(value);
        prop_assert_eq!(result1.is_ok(), result2.is_ok());
    }
}
```

### 2. Fuzzing Integration
```rust
// Add fuzz tests for critical paths
#[cfg(test)]
mod fuzz_tests {
    use super::*;
    
    #[test]
    fn fuzz_audio_sample_calculations() {
        // Use cargo-fuzz for this in practice
        let test_cases = [
            (f32::MIN, u32::MIN),
            (f32::MAX, u32::MAX),
            (0.0, 0),
            (-0.0, 44100),
            (f32::INFINITY, 44100),
            (f32::NEG_INFINITY, 44100),
            (f32::NAN, 44100),
        ];
        
        for (duration, sample_rate) in test_cases {
            // Should never panic or cause undefined behavior
            let _ = audio::safe_duration_to_samples(duration, sample_rate);
        }
    }
}
```

### 3. Performance Regression Tests
```rust
// Add to benches/ directory
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_casting_operations(c: &mut Criterion) {
    c.bench_function("safe_duration_to_samples", |b| {
        b.iter(|| {
            audio::safe_duration_to_samples(
                black_box(1.0), 
                black_box(44100)
            )
        });
    });
    
    c.bench_function("casting_builder_audio", |b| {
        let builder = CastingBuilder::for_audio();
        b.iter(|| {
            builder.float_to_int::<usize>(black_box(44100.0))
        });
    });
}

criterion_group!(benches, benchmark_casting_operations);
criterion_main!(benches);
```

### 4. Edge Case Matrix Testing
```rust
#[test]
fn test_edge_case_matrix() {
    let edge_values = [
        (i64::MIN, "minimum i64"),
        (i64::MAX, "maximum i64"),
        (-1, "negative one"),
        (0, "zero"),
        (1, "one"),
        (isize::MAX as i64, "maximum safe"),
        (isize::MAX as i64 + 1, "just over safe"),
    ];
    
    for (value, description) in edge_values {
        // Test each conversion path with edge values
        let usize_result = safe_db_count_to_usize(value);
        let validation_result = validate_db_count(value);
        let fits_result = can_fit_in_usize(value);
        
        // Log results for analysis
        println!("Testing {}: {} -> usize={}, valid={:?}, fits={}", 
                description, value, usize_result, validation_result, fits_result);
        
        // Verify consistency between functions
        if fits_result {
            assert!(validation_result.is_ok(), 
                   "Value {} should validate if it fits", value);
        }
    }
}
```
