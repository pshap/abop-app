# Copilot Instructions for ABOP

## Project Overview
- **ABOP** is a modular Rust workspace for audiobook management and processing, with three main crates:
  - `abop-core`: Core audio processing, business logic, and database
  - `abop-gui`: Iced-based GUI with Material Design 3 theming
  - `abop-cli`: Command-line interface (basic)

## Architecture & Patterns
- **Strict modularity**: Each crate has a clear responsibility; cross-crate logic is via public APIs only.
- **Component-based GUI**: All UI in `abop-gui` uses reusable components and design tokens (see `src/components/`, `src/styling/`).
- **Safe casting**: Never use direct `as` casts. Use safe conversion utilities in `abop-core/src/audio/processing/casting_utils.rs`, `abop-gui/src/utils/safe_conversions.rs`, or Rust's `TryFrom`/`TryInto`.
- **Error handling**: Use custom error types (`abop-core/src/error.rs`) and propagate errors with `Result<T, AppError>`.
- **Material Design 3**: All theming and layout in the GUI must use design tokens and follow MD3 specs.
- **Database**: SQLite via rusqlite, with connection pooling and migrations in `abop-core/src/db/migrations/`.
- **Testing**: Use `cargo nextest run --all --no-capture` for tests. Property-based tests (`proptest`) for conversions and audio processing.
- **Accessibility**: Honor `ABOP_REDUCE_MOTION`/`PREFER_REDUCED_MOTION` env vars. All UI must degrade gracefully.

## Developer Workflows
- **Build all**: `cargo build` (workspace), or `cargo build -p abop-gui`/`abop-core`/`abop-cli`.
- **Run GUI**: `cargo run -p abop-gui` (set `$env:RUST_LOG` for log level).
- **Run CLI**: `cargo run -p abop-cli`.
- **Test**: `cargo nextest run --all --no-capture` (preferred), or `cargo test --workspace`.
- **Lint/Format**: `cargo clippy --workspace --all-targets`, `cargo fmt --all`.
- **Docs**: `cargo doc --workspace --open`.

## Project-Specific Conventions
- **File size**: Keep modules under 300 lines; split if larger.
- **Design tokens**: Never hardcode spacing/colors in GUI; always use tokens from `styling/` or `design_tokens.rs`.
- **Selection components**: Use the `CommonSelectionBuilder` trait and `CommonBuilderState` for new selection UIs.
- **Logging**: Use `tracing` throughout; control with `RUST_LOG`.
- **Experimental Rust**: Nightly toolchain is used for SIMD and other features (see `rust-toolchain.toml`).
- **No direct cross-crate imports**: Use public APIs only; never reach into another crate's internals.

## Examples
- **Audio processing**: See `abop-core/src/audio/processing/` and usage in `README.md`.
- **GUI component**: New components go in `abop-gui/src/components/`, use design tokens, and follow DRY patterns.
- **Database migration**: Add SQL to `abop-core/src/db/migrations/`, update models and repositories.

## Key References
- `README.md`, `CLAUDE.md` (project philosophy, architecture, and workflow)
- `abop-gui/README.md` (GUI-specific patterns)
- `abop-core/src/audio/processing/casting_utils.rs` (safe casting)
- `abop-gui/src/styling/`, `abop-gui/src/design_tokens.rs` (design system)
- `abop-core/src/error.rs` (error handling)

---
**Always follow established patterns and update documentation for new conventions.**
