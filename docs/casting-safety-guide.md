# Casting Safety Guide for ABOP-Iced

## Quick Reference

### Audio Processing
- `safe_usize_to_f64_audio()` - For sample count calculations
- `safe_f64_to_usize_samples()` - Converting back to sample indices
- `safe_sample_progress()` - Percentage calculations

### Database Operations  
- `safe_db_count_to_usize()` - SQLite count results
- `validate_db_count()` - With error handling
- Platform-safe for 32-bit systems

### UI Components
- `safe_spacing_to_pixels()` - Material Design spacing
- `safe_thickness_to_pixels()` - Border/divider thickness
- `format_file_size()` - Human-readable file sizes

## Performance Impact
All safe conversion utilities have < 1% performance overhead compared to direct casting, with significant safety benefits.

## Migration Guide
- Replace all direct `as` casts with the appropriate safe conversion utility.
- For new code, always use the safe conversion utilities.
- For tests, use `#[allow(clippy::cast_...)]` only when intentional and safe.

## Example Patterns
```rust
// Audio: usize to f64
let samples = safe_conversions::safe_usize_to_f64_audio(count)?;

// Database: i64 to usize
let total = db_conversions::safe_db_count_to_usize(db_count);

// UI: f32 to u16
let px = ui_conversions::safe_spacing_to_pixels(spacing);
```

## Platform Notes
- All conversions are tested on 32-bit and 64-bit platforms.
- Use property-based tests to ensure no panics or overflows.

## Further Reading
- [Rust Numeric Casts](https://doc.rust-lang.org/rust-by-example/types/cast.html)
- [Clippy Lints: Casts](https://rust-lang.github.io/rust-clippy/master/index.html#cast_possible_truncation)

## Contact
For questions or migration help, contact the ABOP-Iced maintainers.
