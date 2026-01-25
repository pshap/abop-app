# ABOP Codebase Improvements

## What This Is

A Rust audiobook management application built with Iced GUI. We're shifting from feature verification to improving code quality, performance, reducing technical debt, and documentation.

## Core Value

A maintainable, well-documented, performant Rust codebase with excellent engineering practices.

## Requirements

### Validated
- ✓ Comprehensive audio playback, library management, and processing features exist
- ✓ Test infrastructure passing (894 tests)
- ✓ Material Design 3 theming system working

### Active
- [ ] Reduce code duplication (DRY violations)
- [ ] Fix unwrap() calls and improve error handling
- [ ] Optimize performance bottlenecks
- [ ] Add/improve documentation
- [ ] Remove unnecessary tests
- [ ] Improve code structure and modularity

### Out of Scope
- New feature development (already implemented)
- Manual GUI verification
- Advanced features beyond what exists

## Context

The ABOP audiobook manager was built with comprehensive features:
- Library scanning, browsing, searching, filtering
- Audio playback with player state management
- Audio processing (mono conversion)
- Settings management
- Material Design 3 theming

All features are implemented and tests pass (894/894), but there's room for improvement in:
- Code quality: Some duplication, large files (>300 lines)
- Technical debt: unwrap() calls, error handling could improve
- Performance: Potential optimizations not yet explored
- Documentation: Public APIs need better docs
- Tests: May have unnecessary tests, could improve coverage

## Constraints
- **Tech stack**: Rust 2024, Iced 0.13.1, Symphonia, Rodio, SQLite
- **Codebase**: Workspace with 3 crates (core, gui, cli)
- **Testing**: Use existing test infrastructure, remove what's not needed
- **Philosophy**: Existing patterns guide refactoring, don't reinvent

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Focus on improvements | Features complete, tests passing | — Pending |
| Remove unnecessary tests | Maintain valuable test coverage, reduce noise | — Pending |
| Fix unwrap() calls | Safer code, better error handling | — Pending |
| Improve structure | Better maintainability, easier onboarding | — Pending |

---
*Last updated: 2026-01-25 after scope change*