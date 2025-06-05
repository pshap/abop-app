# AI Implementation Prompt for ABOP Performance Optimizations

## Context
You are implementing performance optimizations for the ABOP (Audiobook Organizer & Processor) audio processing pipeline. The project is a Rust-based application with a well-structured modular architecture.

## Project Structure
```
abop-core/src/audio/
├── mod.rs                     # AudioBuffer<T> definition
├── decoder.rs                 # Audio format decoding
└── processing/
    ├── pipeline.rs            # AudioProcessingPipeline coordinator
    ├── resampler.rs           # LinearResampler (main target)
    ├── channel_mixer.rs       # ChannelMixer (mix_to_mono, mix_to_stereo)
    ├── normalizer.rs          # AudioNormalizer (normalize method)
    ├── batch_processor.rs     # BatchProcessor (process_files_parallel)
    └── file_io.rs             # AudioFileProcessor
```

## Current Architecture
- **AudioBuffer<T>**: `{data: Vec<T>, format: SampleFormat, sample_rate: u32, channels: u16}`
- **BatchProcessor**: Uses Rayon for parallel processing with `process_files_parallel` method
- **LinearResampler**: Core component with `process` method for sample rate conversion
- **Error Handling**: Uses `AudioProcessingError` and `BatchProcessingError` types

## Implementation Task
Implement the performance optimizations outlined in `suggestions/performance_optimizations.md` in the following priority order:

### Phase 1: SIMD Processing (Highest Priority)
**Target**: `abop-core/src/audio/processing/resampler.rs`

1. Add SIMD support to `LinearResampler::process` method
2. Implement `process_simd` method using `std::simd::f32x8`
3. Process 8 samples at a time with fallback to scalar processing
4. Maintain compatibility with existing `process` method signature

**Requirements**:
- Use `std::simd::{f32x8, Simd}` for portable SIMD
- Process chunks of 8 samples, handle remainder with scalar code
- Return `Result<(), AudioProcessingError>` to match existing API
- Add feature flag `simd` for conditional compilation

### Phase 2: Memory Pool Implementation
**Target**: `abop-core/src/audio/mod.rs` and `batch_processor.rs`

1. Create `AudioBufferPool<T>` with thread-safe acquire/release methods
2. Integrate into `BatchProcessor::process_files_parallel`
3. Reduce allocation overhead in batch operations

**Requirements**:
- Thread-safe pool using `Arc<Mutex<Vec<AudioBuffer<T>>>>`
- Pool size of 16 buffers with 1MB capacity each
- `acquire()` and `release()` methods
- Integration with existing `BatchProcessor` structure

### Implementation Guidelines

#### Code Quality
- Maintain existing error handling patterns
- Follow Rust naming conventions and module structure
- Add comprehensive documentation for new public APIs
- Include unit tests for new functionality

#### Performance Considerations
- Profile before and after implementation using `criterion` benchmarks
- Ensure SIMD fallback works on all target architectures
- Minimize locking contention in memory pool implementation
- Preserve existing parallel processing benefits

#### Compatibility
- All optimizations must maintain existing public APIs
- No breaking changes to current method signatures
- Feature flags for optional optimizations (`simd`, `optimize-branches`)
- Graceful degradation when optimizations unavailable

## Testing Requirements

### Unit Tests
Create tests for:
- SIMD vs scalar processing output equivalence
- Memory pool acquire/release behavior under concurrent access
- Performance regression tests for critical paths

### Integration Tests
- Verify batch processing still works with pooled buffers
- Test with real audiobook files (MP3, M4A, WAV)
- Ensure error handling paths remain functional

### Benchmarks
Use `criterion` to measure:
- Sample processing throughput (samples/second)
- Memory allocation rates during batch operations
- End-to-end file processing performance

## Current Dependencies
The project already includes:
- `rayon` for parallel processing
- Standard error handling types
- Modular audio processing pipeline

## Expected Outcomes
- 2-4x improvement in sample processing throughput (SIMD)
- 30-50% reduction in allocation overhead (memory pooling)
- Maintained code quality and API compatibility
- Clear performance metrics and benchmarking results

## Implementation Notes
- Start with the LinearResampler SIMD optimization as it will provide the most immediate performance gains
- The memory pool should integrate seamlessly with existing BatchProcessor parallel operations
- All changes should be additive - enhance existing functionality rather than replacing it
- Consider real-world audiobook processing workloads when optimizing

## Success Criteria
1. SIMD implementation processes audio samples 2-4x faster than scalar version
2. Memory pool reduces allocation pressure during batch processing by >30%
3. All existing tests pass without modification
4. New benchmarks demonstrate clear performance improvements
5. Code maintains existing style and architecture patterns

Begin with Phase 1 (SIMD) implementation in the LinearResampler, ensuring you understand the current `process` method before adding optimizations.
