CLEAN-PROMPT

# Comprehensive Code Review Prompt for Rust 2024/Iced 0.13.x Audiobook App

## Environment Context
- **Rust Edition**: 2024
- **Iced Version**: 0.13.x
- **Platform**: Windows with PowerShell
- **App Type**: Audiobook organizing application with Material Design 3

## Review Scope
**Default**: Analyze the entire codebase unless I specify a particular module, file, or directory.

## Review Categories

### 1. Rust 2024 Edition Best Practices
Examine the code for modern Rust 2024 idioms and best practices:

- **Error Handling**: Check for proper use of `Result<T, E>` and `Option<T>`, avoiding `unwrap()` and `expect()` in production code. Look for opportunities to use `?` operator and `anyhow`/`thiserror` crates
- **Async/Await**: Ensure proper async patterns, especially for file I/O operations (audiobook scanning, metadata reading)
- **Ownership & Borrowing**: Look for unnecessary clones, inefficient borrowing patterns, or lifetime issues. Check for proper use of `Cow<'_, T>` where appropriate
- **Pattern Matching**: Ensure exhaustive pattern matching, proper use of `if let` chains, and `let-else` statements (Rust 2021+)
- **Generics & Traits**: Check for proper trait bounds, associated types, and generic parameter usage
- **Memory Safety**: Identify potential memory leaks, unsafe code blocks that could be avoided
- **Performance**: Spot inefficient algorithms, unnecessary allocations, or suboptimal data structures for large audiobook collections
- **Naming Conventions**: Verify snake_case for functions/variables, PascalCase for types, SCREAMING_SNAKE_CASE for constants
- **Documentation**: Check for missing or inadequate doc comments, especially on public APIs
- **Cargo.toml**: Review dependencies for unused crates, outdated versions, or missing features. Check for proper workspace configuration
- **Windows-specific**: Ensure proper path handling with `std::path::PathBuf`, Windows path separators, and case-insensitive file operations

### 2. Iced 0.13.x Framework Best Practices
Analyze adherence to Iced 0.13.x-specific patterns and performance considerations:

- **Application Architecture**: Check proper use of `iced::Application` trait, `new()`, `title()`, `update()`, `view()`, and `subscription()` methods
- **Message Handling**: Ensure clean separation between UI events and business logic in `update()` methods. Check for proper `Command` return types
- **State Management**: Verify appropriate state structure, avoiding unnecessary complexity in the main application state
- **Widget Usage**: 
  - Check for proper use of Iced 0.13.x widgets (`button`, `text_input`, `scrollable`, `container`, `column`, `row`)
  - Verify efficient widget composition and layout strategies
  - Look for proper use of `Element<Message>` type
- **Styling & Theming**:
  - Check implementation of custom styles using the new styling system
  - Verify proper use of `Theme` and custom theme implementations
  - Look for consistent color scheme application
- **Subscriptions**: Examine subscription usage for file watching, timer events, or async operations. Check for proper subscription lifecycle management
- **Commands**: Check for proper async operation handling using `Command::perform()` and command chaining
- **Custom Widgets**: If any, verify they follow Iced 0.13.x widget patterns and proper `Widget` trait implementation
- **Performance**: 
  - Identify unnecessary redraws or heavy computations in `view()` method
  - Check for efficient widget tree construction
  - Look for proper use of `Cache` for expensive operations
- **Window Management**: Check proper window configuration and event handling

### 3. Material Design 3 Principles Implementation
Assess how well the app incorporates MD3 design principles:

- **Color System**: Check implementation of dynamic color theming, color roles, and accessibility
- **Typography**: Verify proper type scale usage, font weights, and text hierarchy
- **Motion & Animation**: Look for opportunities to add meaningful transitions and micro-interactions
- **Component Design**: Ensure buttons, cards, navigation follow MD3 specifications
- **Layout & Spacing**: Check adherence to MD3 grid system and spacing guidelines
- **Accessibility**: Verify color contrast ratios, touch targets, and screen reader compatibility
- **Elevation & Shadows**: Assess proper use of surface elevation and shadow system
- **Adaptive Design**: Check responsive behavior and layout adaptation

### 4. Hardcoded Values Analysis
Identify values that should be externalized or made configurable:

- **Magic Numbers**: Dimensions, delays, thresholds that should be named constants or configuration values
- **File Paths**: Hardcoded directories that should use Windows environment variables (`%APPDATA%`, `%USERPROFILE%`) or config files
- **Windows-specific Paths**: Ensure proper handling of Windows path conventions (backslashes, drive letters, UNC paths)
- **URLs & Endpoints**: API endpoints for audiobook metadata that should be configurable
- **UI Dimensions**: Widget sizes, margins, paddings that should be theme-based or responsive
- **String Literals**: Text that should be externalized for internationalization (consider `fluent` crate)
- **Configuration Values**: Audio settings, library paths, playback preferences that users should customize
- **Resource Limits**: Buffer sizes for audio processing, file scanning limits, cache sizes
- **Audio Format Constants**: Supported file extensions, codec parameters, quality settings
- **PowerShell Integration**: Command templates or scripts that might be used for file operations

### 5. DRY (Don't Repeat Yourself) Violations
Identify code duplication and repetitive patterns:

- **Duplicate Logic**: Similar functions or methods that could be generalized (especially audio file processing)
- **Repeated UI Patterns**: Widget compositions that appear multiple times (audiobook cards, navigation elements)
- **Copy-Paste Code**: Nearly identical code blocks with minor variations
- **Similar Data Structures**: Types that could be unified or parameterized (audiobook metadata, user preferences)
- **Redundant Error Handling**: Repeated error handling patterns that could be abstracted into helper functions or macros
- **Common File Operations**: Repeated patterns for reading audiobook files, metadata extraction, or file system operations
- **Windows Path Handling**: Duplicated path manipulation logic that should be centralized
- **Iced Widget Patterns**: Repeated widget styling or layout patterns that could be extracted into custom components

### 6. Code Quality Improvement Opportunities
Look for areas to enhance overall code quality:

- **Function Length**: Methods that are too long and should be broken down
- **Cognitive Complexity**: Complex nested logic that could be simplified
- **Single Responsibility**: Functions or types doing too many things
- **Abstraction Levels**: Mixing high-level and low-level operations
- **Code Comments**: Misleading, outdated, or unnecessary comments
- **Test Coverage**: Missing unit tests, integration tests, or test quality issues
- **Modularity**: Poor module organization or circular dependencies
- **API Design**: Inconsistent or confusing public interfaces

### 7. Code Centralization Opportunities
Identify redundant code that should be centralized:

- **Common Utilities**: Helper functions scattered across modules (file system operations, path handling)
- **Shared Constants**: Values defined in multiple places (supported audio formats, default settings)
- **Similar Widgets**: Custom Iced widgets with overlapping functionality
- **Audio Processing**: Similar transformation, validation, or metadata extraction logic
- **Configuration Handling**: Settings management spread across files (consider `config` or `serde` patterns)
- **Error Types**: Custom error types that could be unified using `thiserror` derive macro
- **Logging Patterns**: Inconsistent logging approaches (consider centralizing with `tracing` crate)
- **File System Operations**: Windows-specific file operations that should be abstracted
- **Database/Storage**: If using any persistence, centralize data access patterns
- **Theme/Styling**: Centralize Material Design 3 color schemes and component styles

## Windows/PowerShell Specific Considerations
- **Path Handling**: Verify proper use of `std::path::Path` and `PathBuf` for Windows paths
- **File System**: Check for proper handling of Windows file attributes, permissions, and long path support
- **Process Spawning**: If spawning any processes or PowerShell commands, check for proper error handling and security
- **Unicode Support**: Ensure proper Unicode handling for international audiobook titles and file names
- **Performance**: Consider Windows-specific performance optimizations for file I/O operations
NO GREP, WC -L ETC

## Output Format
For each category, provide:

1. **Issues Found**: List specific problems with file/line references
2. **Severity**: High/Medium/Low priority
3. **Recommendation**: Concrete steps to address each issue
4. **Code Examples**: Show before/after snippets where helpful
5. **Impact**: Explain the benefits of making the suggested changes

## Additional Instructions
- Prioritize issues that affect functionality, security, or maintainability
- Consider the audiobook app context when making recommendations
- Suggest modern Rust patterns and recent Iced framework updates
- Provide rationale for each recommendation
- Group related issues together for easier implementation
- Consider performance implications of suggested changes

## Example Usage
```
Please review [specific module/entire codebase] using this comprehensive analysis framework. Focus especially on [any particular category if needed].
```