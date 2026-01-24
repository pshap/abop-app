# ABOP - Audiobook Organizer & Processor

A modern Rust audiobook management system with advanced processing capabilities, featuring a Material Design 3 GUI.

## Project Overview

ABOP is a high-performance audiobook organizer and processor built with the Rust 2024 edition. It leverages the Iced framework for its GUI and Symphonia/Rodio for audio processing.

### Key Technologies
- **Rust 2024**: Latest language features and safety.
- **Iced 0.13.1**: Modern, component-based GUI framework.
- **Symphonia**: Pure Rust audio decoding for various formats (MP3, M4B, FLAC, OGG, etc.).
- **Rodio**: Audio playback and streaming.
- **SQLite (rusqlite)**: Embedded database for metadata and library management.
- **Rayon**: Parallel processing for batch operations.
- **Material Design 3**: Professional design system integration.

## Architecture

The project is structured as a Rust workspace:

- **`abop-core/`**: Core business logic and audio processing engine.
  - `audio/`: Audio pipeline, decoding, and processing (resampling, normalization, silence detection).
  - `db/`: SQLite operations with connection pooling (r2d2) and repository patterns.
  - `scanner/`: Multi-threaded directory scanning and metadata extraction.
  - `models/`: Domain models (Audiobook, Library, Progress, etc.).
  - `config/`: Type-safe configuration with validation.
- **`abop-gui/`**: Graphical User Interface using Iced.
  - `styling/`: Material Design 3 implementation and design tokens.
  - `components/`: Reusable UI elements.
  - `views/`: Application views (Library, Settings, Player).
- **`abop-cli/`**: Command-line interface for headless operations (in progress).

## Development Guidelines

### Coding Standards
- **Modular Design**: Keep modules focused and small (target < 300 lines).
- **Type Safety**: Use Rust's type system to enforce valid configurations and states.
- **Error Handling**: Use `thiserror` for library errors and `anyhow` for application-level errors.
- **Performance**: Use async/await for I/O and Rayon for CPU-bound tasks.
- **Material Design 3**: Adhere to MD3 principles for UI/UX, using the centralized styling system in `abop-gui/src/styling`.

### Testing
- **Centralized Utilities**: Use `abop-core/src/test_utils` and `abop-gui/src/test_utils` for consistent test data and environment setup.
- **Database Tests**: Use `TestDatabase` for in-memory or temporary file-based DB testing.
- **GUI Tests**: Use component-based testing patterns as described in `docs/TESTING_PATTERNS.md`.
- **Command**: Run tests with `cargo test --workspace`.

## Building and Running

### Prerequisites
- **Rust 2024+**: Install via [rustup.rs](https://rustup.rs/).
- **Linux System Dependencies**: `pkg-config`, `gtk3-dev`, `libssl-dev`, `libasound2-dev`.

### Key Commands
- **Build**: `cargo build`
- **Run GUI**: `cargo run -p abop-gui`
- **Run CLI**: `cargo run -p abop-cli`
- **Test**: `cargo test --workspace`
- **Lint**: `cargo clippy --workspace`
- **Format**: `cargo fmt --all`
- **Doc**: `cargo doc --workspace --open`

## Documentation
- `docs/architecture.md`: Detailed system architecture.
- `docs/TESTING_PATTERNS.md`: Guidelines for writing tests.
- `docs/color_strategy_system.md`: MD3 color implementation details.
- `docs/styling_system.md`: Overview of the GUI styling architecture.
