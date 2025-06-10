# ABOP - Audiobook Organizer & Processor

*A personal project for organizing and processing audiobooks*

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🚧 Development Status

**This application is currently in active development with significant functionality already implemented.** The core audio processing engine, library scanning, basic audio playback, and Material Design 3 GUI foundation are all working. While still evolving toward a complete release, ABOP provides functional audiobook library management and basic playback capabilities.

### What's Implemented
- ✅ **Core Audio Processing Pipeline**: Complete framework with resampling, channel mixing, normalization, and silence detection
- ✅ **Material Design 3 Translation**: Comprehensive design system adapted from TypeScript to Rust
- ✅ **Database Layer**: SQLite integration with connection pooling and health monitoring  
- ✅ **GUI Foundation**: Iced-based interface with component architecture and theming
- ✅ **Configuration System**: Type-safe, validated processing parameters
- ✅ **Test Suite**: Extensive testing across audio processing components
- ✅ **Library Scanner**: File system traversal and audiobook discovery with metadata extraction
- ✅ **Audio Playback**: Basic audio player with playback controls and state management
- ✅ **File Management UI**: Directory scanning and audiobook library management interface
- ✅ **Progress Tracking**: State persistence and progress monitoring for long-running operations

### What's Coming Next
- 🔲 **Complete Application Integration**: Connecting all pieces into a polished, cohesive experience
- 🔲 **Advanced Audio Processing**: Enhanced batch processing with customizable quality presets
- 🔲 **Enhanced Playback Features**: Chapter navigation, bookmarks, and advanced playback controls
- 🔲 **CLI Interface**: Command-line tools for headless operation and automation
- 🔲 **Export/Import**: Backup and restore functionality for library databases

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
- **[Symphonia 0.5.4](https://github.com/pdeljanov/Symphonia)**: Pure Rust audio decoding for MP3, FLAC, AAC, OGG, WAV, M4B, M4A
- **[Rayon 1.10.0](https://github.com/rayon-rs/rayon)**: Data parallelism for batch processing operations
- **Custom Pipeline**: Modular processing with configurable quality settings

### User Interface  
- **[Iced 0.13.1](https://github.com/iced-rs/iced)**: Cross-platform GUI framework with component architecture
- **[Iced Font Awesome 0.2.1](https://github.com/iced-rs/iced_aw)**: Icon system integration
- **[Palette 0.7.6](https://github.com/Ogeon/palette)**: Color science for Material Design 3 theming

### Data & Configuration
- **[SQLite](https://www.sqlite.org/)** via **[rusqlite](https://github.com/rusqlite/rusqlite)**: Embedded database with full-text search
- **[Serde](https://serde.rs/)**: Type-safe serialization for configuration persistence
- **[UUID v4/v7](https://github.com/uuid-rs/uuid)**: Unique identification for files and database records

### Development & Quality
- **Rust 2024 Edition**: Latest language features and safety improvements  
- **[Tracing](https://tracing.rs/)**: Structured logging and diagnostics
- **Comprehensive Testing**: Unit tests across all processing components

## 📁 Project Architecture

```
abop-iced/
├── abop-core/                 # Audio processing engine
│   ├── src/audio/            # Audio pipeline and processing
│   │   └── processing/       # Resampling, mixing, normalization
│   ├── src/db/               # Database operations and health
│   ├── src/models/           # Domain models and business logic
│   └── src/scanner.rs        # File system traversal
│
├── abop-gui/                 # Iced GUI application  
│   ├── src/components/       # Reusable UI components
│   ├── src/styling/          # Material Design 3 implementation
│   │   └── material/         # Design tokens and theme system
│   ├── src/views/            # Application screens and layouts
│   └── src/handlers/         # Message processing and state updates
│
├── abop-cli/                 # Command-line interface (planned)
└── material-web-clean/       # Material Components reference
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
git clone https://github.com/yourusername/abop-iced.git
cd abop-iced

# Build the workspace
cargo build

# Run tests to verify setup
cargo test --workspace

# Start the GUI (functional library management and basic playback)
cargo run -p abop-gui

# Generate documentation
cargo doc --workspace --open
```

### Example Usage

```rust
use abop_core::audio::processing::{AudioProcessingPipeline, ProcessingConfig};
use abop_core::audio::ChannelMixerConfig;

let config = ProcessingConfig {
    channel_mixer: Some(ChannelMixerConfig {
        target_channels: Some(1), // Convert to mono
        mix_algorithm: MixingAlgorithm::Average,
    }),
    ..Default::default()
};

let pipeline = AudioProcessingPipeline::new(config)?;
// Process audiobooks - core functionality is implemented
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

*Built with modern Rust for performance, safety, and maintainability*

## Feature Limitations

The following features are intentionally not supported at this time:
- Cover art extraction or display
- Additional audio formats beyond MP3, M4B, and FLAC
- Online metadata fetching or enrichment

These limitations are by design to maintain focus on core functionality and performance.