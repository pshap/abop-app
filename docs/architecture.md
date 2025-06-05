# ABOP-Iced Architecture

## System Overview
ABOP-Iced is a modern, high-performance GUI application for organizing and processing audiobooks, built using Rust and the Iced framework. The system is designed with modular architecture, efficient resource management, and comprehensive audio processing capabilities for handling large audiobook collections.

## Key Design Principles

### Architectural Philosophy
- **Modular Design**: Clear separation of concerns across workspace crates
- **Performance First**: Optimized for large-scale audio processing with async operations
- **Type Safety**: Leveraging Rust's type system for robust configuration and error handling
- **Domain-Driven Design**: Modules organized by business functionality
- **Resource Efficiency**: Memory-conscious processing with streaming capabilities

### Development Standards
- **Single Responsibility**: Each module has a clear, focused purpose
- **Dependency Management**: Clean separation between core logic and UI layers
- **Configuration Management**: Centralized, validated settings using type-safe configurations
- **Design System Enforcement**: Consistent use of design tokens across all UI components
- **Error Handling**: Comprehensive error types with proper propagation

## Workspace Organization

The project follows a clean workspace structure with three main crates:

```
abop-iced/
├── abop-core/        # Core business logic and audio processing engine
├── abop-gui/         # Iced-based graphical user interface
└── abop-cli/         # Command-line interface (minimal implementation)
```

### Crate Dependencies
- `abop-gui` depends on `abop-core` for business logic
- `abop-cli` uses minimal dependencies (planned for future expansion)
- `abop-core` is self-contained with no internal workspace dependencies

## Core Components

### 1. Audio Processing Engine (`abop-core`)

The core crate provides comprehensive audio processing capabilities with a focus on performance and reliability.

#### Core Architecture
- **Symphonia Integration**: Multi-format audio decoding (MP3, FLAC, AAC, OGG, WAV, M4B, M4A)
- **Advanced Processing**: Normalization, channel mixing, silence detection, and audio manipulation
- **Configuration System**: Type-safe, validated processing configurations with TOML support
- **Parallel Processing**: Rayon-based batch operations for large libraries
- **Database Layer**: SQLite-based storage with rusqlite for metadata and state persistence

#### Module Structure
```
abop-core/src/
├── app/              # Application lifecycle and state management
├── audio/            # Audio processing pipeline and codec support
├── component.rs      # Reusable component traits (Component, Renderable, Updatable)
├── config.rs         # Configuration management and validation
├── constants.rs      # Application constants and shared values
├── db/               # Database operations and schema management
├── error.rs          # Centralized error handling (AppError, Result)
├── message.rs        # Application message types (AppMessage)
├── models/           # Domain models organized by business area
├── scanner.rs        # File system scanning with audio format detection
├── services.rs       # Service container and dependency injection
└── utils.rs          # Shared utility functions
```

#### Key Features
- **Type-Safe Configuration**: Validated settings with proper error handling
- **Streaming Processing**: Memory-efficient handling of large audio files
- **Async-First Design**: Non-blocking operations throughout the pipeline
- **Comprehensive Testing**: Extensive test coverage for audio operations

### 2. Graphical User Interface (`abop-gui`)

Modern Iced-based interface with professional design system and Material Design integration.

#### Architecture Overview
- **Iced 0.13.1**: Latest framework features with canvas, image, and SVG support
- **Component Architecture**: Modular, reusable UI components
- **Professional Theme System**: Multi-theme support with sunset and Material Design themes
- **Real-time Updates**: Live progress tracking and responsive state updates
- **Advanced Styling**: Comprehensive design token system with plugin architecture

#### Module Structure
```
abop-gui/src/
├── design_tokens/    # Centralized design system (spacing, typography, colors)
├── theme.rs          # Theme management (Dark/Light Sunset, Material Design)
├── styling/          # Professional styling components
│   ├── container/    # Specialized container styles (base, dialog, feedback, layout)
│   ├── material/     # Material Design 3 component implementations
│   ├── builders.rs   # Fluent API for style construction
│   ├── button.rs     # Semantic button styling with variants
│   ├── input.rs      # Form input styling with validation states
│   └── plugins.rs    # Extensible styling plugin system
├── components/       # Reusable UI components
├── views/            # Application views (library, settings, processing)
├── handlers/         # Message handling and state updates
├── audio/            # GUI-specific audio controls and visualization
└── library/          # Library management interface components
```

#### Design System Features
- **Design Tokens**: Centralized spacing (XS: 4px → XXL: 48px), typography, and color systems
- **Theme Integration**: Seamless switching between sunset and Material Design themes
- **Component Builders**: Fluent API for consistent component styling
- **Validation Styling**: Built-in error, success, warning, and info states
- **Accessibility**: WCAG-compliant color contrast and keyboard navigation

### 3. Command-Line Interface (`abop-cli`)

Minimal CLI implementation with clap for argument parsing and planned for future expansion.

#### Current Implementation
- **Basic Structure**: Command-line argument parsing with clap
- **Logging**: Configurable logging levels with env_logger
- **Future-Ready**: Extensible design for adding audio processing commands

#### Planned Features
- Batch audio processing commands
- Library scanning and validation
- Configuration management via CLI
- Integration with core processing pipeline
- `handlers/`: Message handling for different UI sections

**Design Token Architecture:**
- **Spacing System**: XS(4px), SM(8px), MD(16px), LG(24px), XL(32px), XXL(48px)
- **Typography Hierarchy**: Consistent font sizes across all components
- **Professional Colors**: Sunset theme with proper contrast ratios
- **Elevation System**: SHADOW_SOFT, SHADOW_MEDIUM, SHADOW_STRONG constants
- **Border Radius**: Unified corner rounding standards

### 3. Data Management Layer
**Robust Storage and Persistence**
- **SQLite Database**: Efficient audiobook metadata storage
- **Connection Management**: Optimized database connection handling
- **Health Monitoring**: Database performance and status tracking
- **Configuration Persistence**: Type-safe settings storage
- **Export Capabilities**: CSV and other format support

**Database Architecture:**
- `db/connection.rs`: Streamlined connection management (340 lines)
- `db/health.rs`: Database monitoring and diagnostics
- `db/retry.rs`: Resilient operation handling
- `db/statistics.rs`: Performance metrics tracking

### 4. File System Operations
**Efficient File Handling**
- **Scanner Engine**: Multi-threaded directory traversal
- **Progress Tracking**: Real-time scan progress reporting
- **Metadata Extraction**: Comprehensive audio file analysis
- **UUID Tracking**: Persistent file identification
- **Async I/O**: Non-blocking file operations

## Technology Stack

### Core Dependencies
- **Rust 2024**: Latest edition with advanced language features
- **Iced 0.13.1**: Modern GUI framework with canvas, image, and SVG support
- **Symphonia**: Comprehensive audio decoding (all major formats)
- **Rodio**: Audio playback and manipulation
- **SQLite/Rusqlite**: Embedded database with bundled SQLite
- **Rayon**: Data parallelism for batch processing
- **Serde/TOML**: Configuration serialization and deserialization

### Development Tools
- **Clap**: Command-line argument parsing
- **Thiserror/Anyhow**: Error handling and propagation
- **Tracing**: Structured logging and diagnostics
- **RFD**: Native file dialogs

## Data Models and Domain Design

### Core Business Models (`abop-core/models/`)
The application uses a domain-driven approach with models organized by business area:

```
models/
├── audiobook.rs      # Audiobook entity with metadata
├── bookmark.rs       # User bookmarks and playback positions
├── core.rs          # Core domain types (Chapter, etc.)
├── library.rs       # Library collections and organization
├── progress.rs      # Playback progress and statistics
├── search.rs        # Search queries and results
└── ui.rs            # UI state and configuration models
```

### Key Domain Types
- **Audiobook**: Complete audiobook with metadata, chapters, and file information
- **Library**: Collection of audiobooks with organization and filtering
- **Progress**: Playback progress, bookmarks, and listening statistics
- **Chapter**: Individual audio segments with timing and metadata
- **UserPreferences**: Application settings and personalization
- **ThemeConfig**: Theme configuration and styling preferences

## Technical Stack

### Core Technologies
- **Rust 2024 Edition**: Modern language features and performance
- **Iced 0.13.1**: Advanced GUI framework with component architecture
- **Symphonia 0.5.4**: Pure Rust audio decoding library
- **Tokio 1.32**: Async runtime for concurrent operations
- **Rayon 1.10.0**: Data parallelism for batch processing

### Audio Processing
- **Multi-format Support**: MP3, M4B, M4A, AAC, FLAC, OGG, WAV
- **Advanced Algorithms**: Normalization, channel mixing, silence detection
- **Quality Control**: Sample rate validation, bit depth management
- **Performance Optimization**: Parallel processing, streaming I/O

### Data & Storage
- **SQLite (rusqlite 0.29)**: Embedded database for metadata
- **Serde 1.0**: Type-safe serialization/deserialization
- **UUID 1.4**: Unique file identification
- **Configuration Management**: Validated, type-safe settings

### UI & User Experience
- **iced_aw 0.12.2**: Advanced widgets and components
- **rfd 0.14**: Cross-platform file dialogs
- **Design Tokens**: Consistent theming system
- **Responsive Layout**: Adaptive UI components

## Architecture Patterns

### 1. Modular Configuration System
**Type-Safe Audio Processing Configuration**
```rust
let config = ProcessingConfig::builder()
    .with_target_sample_rate(44100)
    .with_normalization_algorithm(NormalizationAlgorithm::LUFS)
    .with_silence_threshold(-40.0)
    .build_validated()?;
```

**Benefits:**
- Compile-time validation of audio parameters
- Fluent API for intuitive configuration
- Comprehensive error messages for invalid settings
- Serializable configurations for persistence

### 2. Component-Based UI Architecture
**Modular Interface Design with Consolidated Styling**
- **Container Styling**: Specialized styling modules (base, dialog, feedback, layout, table)
- **View Components**: Independent, reusable UI sections using design tokens
- **Message Handling**: Clean separation of UI logic
- **State Management**: Predictable state updates
- **Design Token Integration**: All components migrated from mixed spacing constants to unified design system

**Professional Design System Benefits:**
- **Consistency**: Unified spacing across all 15+ components and views
- **Maintainability**: Single source of truth for design values
- **Scalability**: Easy to update design system without touching individual components
- **Performance**: Reduced code duplication and improved build times
- **Developer Experience**: Clear, semantic naming (spacing::MD vs hardcoded values)

### 3. Event-Driven Processing
**Async Audio Pipeline**
- Non-blocking audio operations with tokio
- Progress reporting through message channels
- Background task management
- Graceful error handling and recovery

### 4. Database Layer Architecture
**Optimized Data Management**
- Connection pooling and health monitoring
- Retry logic for resilient operations
- Performance statistics and monitoring
- Transaction safety and consistency

## Performance Characteristics

### Memory Management
- **Streaming Processing**: Large files processed in chunks
- **Resource Pooling**: Efficient connection and thread management
- **Memory Safety**: Rust's ownership system prevents leaks
- **Batch Optimization**: Parallel processing with optimal resource usage

### Scalability Features
- **Parallel Scanning**: Multi-threaded directory traversal
- **Concurrent Processing**: Multiple audio files processed simultaneously
- **Progress Tracking**: Real-time updates without blocking
- **Resource Limits**: Configurable thread pools and memory usage

### Cross-Platform Optimization
- **Native Performance**: Platform-specific optimizations
- **Consistent UI**: Uniform experience across Windows, macOS, Linux
- **Audio Codec Support**: Format compatibility across platforms
- **File System Integration**: Platform-aware file operations

## Quality Assurance & Reliability

### Code Quality Metrics
- **Average File Size**: 148 lines (well below 300-line target)
- **Modular Organization**: 99 focused modules vs. previous monolithic structure
- **Design System Consolidation**: 100% migration from mixed spacing constants to unified design tokens
- **Styling Consistency**: All 15+ UI components now use centralized design system
- **Import Cleanup**: Eliminated all abop-core UI constant imports from GUI layer
- **Professional Theming**: Consistent sunset theme across all components
- **Test Coverage**: Comprehensive test suite across all components
- **Documentation**: Complete API documentation and usage examples

### Error Handling Strategy
- **Type-Safe Errors**: Custom error types with detailed context
- **Graceful Degradation**: Fallback mechanisms for failed operations
- **User Feedback**: Clear error messages and recovery suggestions
- **Logging Integration**: Comprehensive debugging and monitoring

### Security Considerations
- **Memory Safety**: Rust's ownership system prevents common vulnerabilities
- **Input Validation**: Comprehensive validation of audio parameters
- **File System Security**: Safe file operations with proper permissions
- **Database Security**: Parameterized queries and transaction safety

## Development Workflow

### Code Organization Best Practices
- **Domain-Driven Design**: Clear separation between business logic and presentation
- **Module Boundaries**: Well-defined interfaces between core and GUI layers
- **Error Handling**: Comprehensive error types with proper propagation
- **Configuration Management**: Type-safe configuration with validation
- **Testing Strategy**: Unit tests for core logic, integration tests for components

### Quality Assurance
- **Type Safety**: Leveraging Rust's type system for compile-time guarantees
- **Memory Safety**: Automatic memory management with zero-cost abstractions
- **Concurrency Safety**: Fearless concurrency with Rust's ownership model
- **Performance Monitoring**: Built-in profiling and performance metrics

### Development Tools Integration
- **Cargo Workspace**: Unified dependency management across crates
- **Documentation**: Comprehensive rustdoc documentation with examples
- **Testing**: Extensive test coverage with cargo test integration
- **Linting**: Clippy integration for code quality enforcement

## Extension Points and Customization

### Plugin Architecture
The styling system supports extensible plugins for custom components:

#### Style Plugins
- **Custom Components**: Define styling for application-specific widgets
- **Theme Extensions**: Add new color schemes and styling variants
- **Validation Rules**: Custom accessibility and design validation
- **Runtime Loading**: Dynamic plugin loading for third-party extensions

#### Configuration Extensions
- **Custom Audio Formats**: Support for additional audio codecs
- **Processing Pipelines**: Custom audio processing workflows
- **Metadata Extractors**: Custom metadata parsing for specialized formats
- **Export Formats**: Additional export and conversion options

### Theming Extensibility
- **Dynamic Theme Loading**: Load themes from external files (TOML/JSON)
- **Color Scheme Generation**: Automatic theme generation from seed colors
- **Component Overrides**: Selective styling overrides for specific components
- **Accessibility Customization**: User-specific accessibility adaptations

## Future Architecture Considerations

### Planned Enhancements
- **Multi-Platform Support**: Enhanced platform-specific optimizations
- **Cloud Integration**: Cloud library synchronization and backup
- **Plugin Ecosystem**: Official plugin registry and management
- **Advanced Audio Processing**: AI-powered audio enhancement and analysis

### Scalability Roadmap
- **Library Size Optimization**: Support for extremely large audiobook collections
- **Network Streaming**: Support for networked audio sources
- **Distributed Processing**: Multi-machine audio processing capabilities
- **Database Scaling**: Support for alternative database backends

### Technology Evolution
- **Framework Updates**: Continuous integration with latest Iced developments
- **Rust Evolution**: Adoption of new Rust language features
- **Audio Technology**: Integration with emerging audio processing technologies
- **Platform Integration**: Enhanced OS-specific feature integration
