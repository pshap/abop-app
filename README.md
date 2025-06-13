# ABOP - Audiobook Organizer & Processor

*A modern Rust audiobook management system with advanced processing capabilities*

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🚧 Development Status

**Active development with core functionality implemented.** The audio processing pipeline, library scanning system, and GUI foundation are functional. The application can scan directories, extract metadata, and manage audiobook libraries with a modern Material Design 3 interface.

### What's Implemented
- ✅ **Audio Processing Pipeline**: Resampling, channel mixing, normalization, silence detection
- ✅ **Library Scanner**: Multi-threaded directory traversal with metadata extraction
- ✅ **Database Layer**: SQLite with connection pooling and performance monitoring
- ✅ **GUI Foundation**: Iced-based interface with Material Design 3 theming
- ✅ **Configuration System**: Type-safe settings with validation
- ✅ **Progress Tracking**: Real-time scanning progress with cancellation support
- ✅ **Audio Playback**: Basic player with state management
- ✅ **Format Support**: MP3, M4A, M4B, FLAC, OGG, WAV, AAC

### Current Limitations
- 🔲 Cover art display and thumbnail generation
- 🔲 Chapter navigation and bookmarking
- 🔲 Batch processing automation
- 🔲 CLI interface for headless operation

## 🎯 Design Principles

ABOP balances modern Rust best practices with Iced GUI framework capabilities and Material Design 3 aesthetic principles:

### Rust Excellence
- **Memory Safety**: Zero-cost abstractions with compile-time guarantees
- **Modular Architecture**: 99 focused modules averaging 148 lines each (51% better than 300-line target)
- **Type-Safe Configuration**: Comprehensive validation at compile and runtime
- **Error Resilience**: Detailed error handling with proper propagation
- **Performance Focus**: SIMD-ready processing with parallel batch operations

### Iced GUI Best Practices  
- **Component-Based Design**: Reusable, composable UI elements
- **Reactive State Management**: Clean message-passing architecture
- **Theme System**: Consistent styling with design tokens
- **Responsive Layouts**: Adaptive interfaces for different screen sizes
- **Non-Blocking Operations**: Async processing with progress tracking

### Material Design 3 Integration
- **Design Tokens**: Color, typography, and spacing systems
- **Component Translation**: Faithful adaptation of Material Components to Rust/Iced
- **Semantic Color System**: Dynamic theming with seed color generation
- **Accessibility**: High contrast ratios and keyboard navigation support
- **Modern Aesthetics**: Clean, purposeful interface design

## 🛠️ Core Technologies

### Audio Processing
- **[Symphonia 0.5.x](https://github.com/pdeljanov/Symphonia)**: Pure Rust audio decoding for multiple formats
- **[Rodio 0.17.x](https://github.com/RustAudio/rodio)**: Audio playback and streaming
- **[Rayon 1.x](https://github.com/rayon-rs/rayon)**: Parallel processing for batch operations

### User Interface  
- **[Iced 0.13.1](https://github.com/iced-rs/iced)**: Cross-platform GUI with component architecture
- **Material Design 3**: Comprehensive theming system and design tokens

### Data & Configuration
- **[SQLite](https://www.sqlite.org/)** via **[rusqlite 0.31.x](https://github.com/rusqlite/rusqlite)**: Embedded database with connection pooling
- **[Serde](https://serde.rs/)**: Type-safe configuration serialization
- **[UUID v4](https://github.com/uuid-rs/uuid)**: File and record identification

### Development & Quality
- **Rust 2024 Edition**: Latest language features and safety improvements  
- **[Tracing](https://tracing.rs/)**: Structured logging and performance monitoring
- **Comprehensive Testing**: Unit and integration tests across all components

## 📁 Project Architecture

```
abop/
├── abop-core/                    # Core audio processing and library management
│   ├── src/audio/               # Audio pipeline, playback, and metadata
│   │   └── processing/          # Resampling, mixing, normalization, silence detection
│   ├── src/db/                  # SQLite operations with connection pooling
│   ├── src/scanner/             # Multi-threaded directory scanning and orchestration
│   ├── src/models/              # Domain models and business logic
│   └── src/config/              # Configuration management and validation
│
├── abop-gui/                    # Iced GUI application  
│   ├── src/components/          # Reusable UI components
│   ├── src/library/             # Library scanning and management UI
│   ├── src/audio/               # Audio player interface
│   ├── src/commands/            # Async command handlers
│   └── src/theme/               # Material Design 3 implementation
│
├── abop-cli/                    # Command-line interface (in progress)
└── docs/                        # Architecture and best practices documentation
```

## 🚀 Getting Started

### Prerequisites
- **Rust 2024+**: Install from [rustup.rs](https://rustup.rs/)
- **System Dependencies**:
  - **Linux**: `pkg-config`, `gtk3-dev`, `libssl-dev`  
  - **macOS**: Xcode command line tools
  - **Windows**: Windows 10/11 (Visual Studio Build Tools recommended)

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/abop.git
cd abop

# Build the workspace
cargo build

# Run tests to verify setup
cargo test --workspace

# Start the GUI application
cargo run -p abop-gui

# Run CLI (basic functionality)
cargo run -p abop-cli

# Generate documentation
cargo doc --workspace --open
```

### Example Usage

```rust
use abop_core::audio::processing::{AudioProcessingPipeline, ProcessingConfig};
use abop_core::audio::processing::ChannelMixerConfig;
use abop_core::scanner::{LibraryScanner, ScanOptions};

// Audio processing example
let config = ProcessingConfig {
    channel_mixer: Some(ChannelMixerConfig {
        target_channels: Some(1), // Convert to mono
        mix_algorithm: MixingAlgorithm::Average,
    }),
    ..Default::default()
};

let pipeline = AudioProcessingPipeline::new(config)?;
pipeline.process_file_with_output(&input_path, &output_path)?;

// Library scanning example
let scanner = LibraryScanner::new(database, library);
let scan_result = scanner.scan(ScanOptions::default())?;
println!("Found {} audiobooks", scan_result.new_files.len());
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

*Built with modern Rust for performance, safety, and maintainability*