# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ABOP (Audiobook Organizer & Processor) is a modern Rust audiobook management system built with Iced GUI framework and comprehensive audio processing capabilities. The project follows a workspace structure with three main crates:

- `abop-core`: Core business logic, audio processing, and database operations
- `abop-gui`: Iced-based graphical user interface with Material Design 3 theming
- `abop-cli`: Command-line interface (basic functionality)

## Build System & Development Commands

### Building the Project
```bash
# Build entire workspace
cargo build

# Build specific crate
cargo build -p abop-core
cargo build -p abop-gui
cargo build -p abop-cli

# Release build with optimizations
cargo build --release
```

### Running Applications
```bash
# Start GUI application
cargo run -p abop-gui

# Run CLI (basic functionality)
cargo run -p abop-cli

# Run with specific log level
RUST_LOG=debug cargo run -p abop-gui
```

### Testing
```bash
# Run all tests
cargo test --workspace

# Test specific crate
cargo test -p abop-core

# Run tests with output
cargo test --workspace -- --nocapture

# Run benchmarks (abop-core has criterion benchmarks)
cargo bench -p abop-core
```

### Code Quality
```bash
# Lint with Clippy
cargo clippy --workspace --all-targets

# Format code
cargo fmt --all

# Generate documentation
cargo doc --workspace --open
```

## Architecture Overview

### Core Dependencies & Technologies
- **Rust 2024 Edition**: Latest language features and performance improvements
- **Iced 0.13.1**: GUI framework with advanced features (canvas, image, SVG)
- **Symphonia**: Pure Rust audio decoding (MP3, FLAC, AAC, OGG, WAV, M4A, M4B)
- **Rodio**: Audio playback and streaming
- **SQLite/rusqlite**: Embedded database with connection pooling
- **Rayon**: Parallel processing for batch operations
- **Tokio**: Async runtime for concurrent operations

### Key Design Principles
1. **Modular Architecture**: Clear separation between core logic and UI
2. **Type Safety**: Extensive use of Rust's type system for configuration validation
3. **Performance**: Streaming audio processing, parallel operations, memory efficiency
4. **Material Design 3**: Professional theming system with design tokens
5. **Safe Casting**: All numeric conversions use safe conversion utilities (no direct `as` casts)

### Project Structure
```
abop/
├── abop-core/                 # Business logic and audio processing
│   ├── src/audio/            # Audio pipeline, codecs, processing algorithms
│   ├── src/db/               # SQLite operations, connection pooling, migrations
│   ├── src/scanner/          # Multi-threaded directory scanning
│   ├── src/models/           # Domain models (audiobook, library, progress)
│   └── src/config/           # Configuration management and validation
├── abop-gui/                 # Iced GUI application
│   ├── src/styling/          # Professional styling system with Material Design
│   ├── src/components/       # Reusable UI components
│   ├── src/views/            # Application views (library, settings, audio processing)
│   └── src/theme/            # Theme management and design tokens
├── abop-cli/                 # Command-line interface
└── docs/                     # Architecture and development documentation
```

## Development Guidelines

### Audio Processing
- Audio files are processed using Symphonia for decoding and Rodio for playback
- Processing pipeline supports normalization, channel mixing, silence detection, resampling
- Configuration is type-safe and validated using `ProcessingConfig`
- All audio operations are designed for streaming to handle large files efficiently

### Database Operations
- SQLite database with connection pooling via r2d2
- Migrations managed in `abop-core/src/db/migrations/`
- Health monitoring and retry logic for resilient operations
- All queries use prepared statements for security

### GUI Development
- Uses Iced 0.13.1 with component-based architecture
- Material Design 3 theming system with comprehensive design tokens
- Professional styling system in `abop-gui/src/styling/`
- All components use centralized design tokens for spacing, typography, colors
- Theme switching between sunset and Material Design themes

### Error Handling
- Custom error types in `abop-core/src/error.rs` using `thiserror`
- Comprehensive error propagation with `Result<T, AppError>`
- Graceful error handling in GUI with user-friendly messages
- Structured logging with `tracing` for debugging

### Safe Casting Practices
**CRITICAL**: This codebase enforces safe numeric conversions
- **Never use direct `as` casts** (e.g., `u64 as usize`, `f64 as u32`)
- Use safe conversion utilities in:
  - `abop-core/src/audio/processing/casting_utils.rs`
  - `abop-core/src/db/helpers.rs` (safe conversions)
  - `abop-gui/src/utils/safe_conversions.rs`
- All conversions must handle potential overflow/truncation
- Use `TryFrom`/`TryInto` traits for fallible conversions
- Property-based tests are required for new conversion utilities

### Testing Strategy
- Unit tests for core logic in each module
- Integration tests in `tests/` directories
- Property-based testing with `proptest` for audio processing
- Benchmark tests using `criterion` for performance validation
- GUI components have test utilities in `test_utils.rs`

### Code Style
- Follow Rust 2024 edition conventions
- Use `rustfmt` for consistent formatting
- Enable Clippy lints defined in workspace `Cargo.toml`
- Average file size kept under 300 lines for maintainability
- Comprehensive documentation with `///` comments for public APIs

### Workspace Management
- Shared dependencies managed in root `Cargo.toml` workspace
- Each crate has specific feature flags when needed
- Internal dependencies: `abop-gui` and `abop-cli` depend on `abop-core`
- Development dependencies include `criterion`, `approx`, `tempfile`

## Common Development Tasks

### Adding New Audio Format Support
1. Extend `AudioFormat` enum in `abop-core/src/audio/mod.rs`
2. Add format detection in `from_extension()` method
3. Update `SUPPORTED_AUDIO_EXTENSIONS` in scanner module
4. Test with sample files of the new format

### Adding New UI Components
1. Create component in `abop-gui/src/components/`
2. Use design tokens from `styling/` modules for consistent styling
3. Follow Iced component patterns with proper message handling
4. Add to the component module exports

### Database Schema Changes
1. Create migration SQL files in `abop-core/src/db/migrations/`
2. Update models in `abop-core/src/models/`
3. Modify queries in `abop-core/src/db/queries/`
4. Test migration with existing data

### Configuration Updates
1. Modify config types in `abop-core/src/config/`
2. Update validation logic
3. Handle backwards compatibility for existing config files
4. Update default configurations

This is a mature codebase with established patterns. When making changes, follow existing architectural decisions and coding standards. The project emphasizes safety, performance, and maintainability through Rust's type system and modern development practices.