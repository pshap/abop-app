# ABOP GUI

Audiobook Organizer & Processor (ABOP) - Modern GUI Application

A sophisticated graphical interface built with Iced 0.13.1, providing an intuitive and responsive experience for audiobook organization and processing.

## 🚀 Features

- **Modern UI**: Built with Iced 0.13.1 featuring component-based architecture
- **Responsive Design**: Adaptive layouts that work across different screen sizes
- **Professional Theming**: Consolidated design token system with consistent spacing, typography, and colors
- **Real-time Updates**: Live progress tracking and status notifications
- **Table Management**: Advanced data visualization with sorting and filtering
- **Component Architecture**: Modular, reusable UI components following DRY principles
- **Audio Integration**: Seamless integration with `abop-core` audio processing engine
- **Consistent Styling**: Unified spacing system using design tokens across all components

## 📋 Prerequisites

- **Rust 2024+**: Install from [rustup.rs](https://rustup.rs/)
- **System Dependencies**:
  - **Windows**: Windows 10/11 (recommended)
  - **Linux**: `pkg-config`, `gtk3-dev`, `libssl-dev`
  - **macOS**: Xcode command line tools

## 🚀 Getting Started

### Running from Workspace Root
From the main `abop-iced` directory:

```pwsh
# Build and run the GUI application
cargo run --release -p abop-gui

# For development (debug build)
cargo run -p abop-gui
```

### Running from GUI Directory
```pwsh
# Navigate to GUI crate
cd abop-gui

# Build and run
cargo run --release
```

## 📁 Project Structure

The GUI crate is organized with a clear component-based architecture:

```
abop-gui/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Main application logic and state management
│   ├── lib.rs               # Library exports and re-exports
│   ├── state.rs             # Application state management
│   ├── theme.rs             # Professional sunset theme with dark/light modes
│   ├── design_tokens.rs     # Centralized design system (spacing, typography, radius, elevation)
│   ├── messages.rs          # Application message types
│   ├── update.rs            # Message handling and updates
│   ├── utils.rs             # Utility functions
│   │
│   ├── components/          # Reusable UI components (spacing consolidated)
│   │   ├── mod.rs
│   │   ├── audio_controls.rs    # ✅ Uses design tokens
│   │   ├── toolbar.rs           # ✅ Uses design tokens
│   │   ├── table.rs             # ✅ Uses design tokens
│   │   ├── status.rs            # ✅ Uses design tokens
│   │   ├── dialogs.rs           # ✅ Uses design tokens
│   │   ├── common.rs            # ✅ Uses design tokens
│   │   └── about.rs             # ✅ Uses design tokens
│   │
│   ├── views/               # Application views (spacing consolidated)
│   │   ├── mod.rs               # ✅ Uses design tokens
│   │   ├── library.rs           # ✅ Uses design tokens
│   │   ├── settings.rs          # ✅ Uses design tokens
│   │   ├── about.rs             # ✅ Uses design tokens
│   │   └── audio_processing.rs  # ✅ Uses design tokens
│   │
│   ├── styling/             # Modular styling system
│   │   ├── mod.rs
│   │   ├── button.rs
│   │   ├── container.rs     # Professional container styles
│   │   ├── input.rs
│   │   ├── scrollable.rs
│   │   └── utils.rs
│   │
│   ├── handlers/            # Event and message handlers
│   ├── audio/               # Audio-related UI components
│   ├── library/             # Library management UI
│   └── commands/            # Async command handling
│
├── assets/                  # Static assets
│   └── fonts/               # Font files
├── examples/                # Usage examples
├── build.rs                 # Build script
└── Cargo.toml              # Dependencies and metadata
```

## 🎨 Architecture

### Component System
The GUI is built using a modular component architecture with consolidated styling:
- **Components**: Reusable UI elements with consistent design token usage
- **Views**: Complete application screens composed of components
- **Handlers**: Centralized message processing and state updates
- **Styling**: Professional theme-based styling with centralized design tokens

### Design Token System
Comprehensive design system providing consistency across the application:
- **Spacing Tokens**: XS(4px), SM(8px), MD(16px), LG(24px), XL(32px), XXL(48px)
- **Typography**: Consistent font sizes and hierarchies
- **Colors**: Professional sunset theme with dark/light mode support
- **Elevation**: Consistent shadow and depth system
- **Border Radius**: Unified corner rounding standards

### Integration with Core
The GUI seamlessly integrates with `abop-core`:
- **Audio Processing**: Real-time processing with progress updates
- **Database Operations**: Efficient data management and display
- **Configuration**: Type-safe configuration management
- **Error Handling**: Comprehensive error display and recovery

## 🔧 Development

### Code Quality
```pwsh
# Format code
cargo fmt

# Check for issues
cargo clippy -- -D warnings

# Run tests
cargo test
```

### Building for Release
```pwsh
# Optimized release build
cargo build --release

# Generate documentation
cargo doc --open
```

## 📚 Key Dependencies

- **[Iced 0.13.1](https://github.com/iced-rs/iced)**: Cross-platform GUI framework
- **[abop-core](../abop-core/)**: Core audio processing and business logic
- **[iced_aw](https://github.com/iced-rs/iced_aw)**: Additional widgets for enhanced functionality
- **[tokio](https://tokio.rs/)**: Async runtime for non-blocking operations

## 🤝 Contributing

When contributing to the GUI:

1. **Follow Component Patterns**: Use the established component architecture
2. **Use Design Tokens**: Always use `crate::design_tokens::spacing` instead of hardcoded values
3. **Maintain Theme Consistency**: Use the professional sunset theme system
4. **Test UI Components**: Ensure components work across different themes and screen sizes
5. **Follow DRY Principles**: Reuse existing styling patterns and components
6. **Document New Features**: Update this README for significant changes

### Style Guidelines
- **Spacing**: Use `spacing::MD` (16px) for standard spacing, `spacing::LG` (24px) for larger gaps
- **Typography**: Follow the established size hierarchy from design tokens
- **Container Styles**: Use appropriate container styles (base, dialog, feedback, layout, table)
- **Import Patterns**: Always import design tokens, never use abop-core constants for UI spacing

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

---

**Part of the ABOP ecosystem** • **Built with ❤️ using Iced and Rust** • **Modern, responsive, and performant**

## Styling Guidelines

1. **Use Material Design Tokens**: Always use `crate::styling::material::*` tokens instead of hardcoded values
2. **Follow Material Design 3**: Adhere to Material Design 3 specifications for components and layouts
3. **Theme Support**: Use the theme system for consistent styling across the application
