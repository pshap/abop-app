# Tech-debt repayment roadmap (abop-gui)

Last updated: 2025-08-18
Owner: abop maintainers
Scope: abop-gui crate and light workspace hygiene touching `abop-core` when versions must align.

## Goals and success criteria

- Reproducible builds: no wildcard deps; shared pins moved to `[workspace.dependencies]`.
- Consistent logging with `RUST_LOG` control and uniform output across binaries.
- Theme wiring uses `theme::ThemeMode` end-to-end; UI reflects chosen mode.
- Router history is bounded/deduped; has `replace(route)`.
- Lower risk of regressions through basic unit tests (router, theming) and CI (fmt, clippy, nextest).
- Clear path for larger refactors (messages, async, fonts, error handling).

## Evaluation summary (from initial audit)

- Dependency hygiene: Pin wildcards (e.g., `material-color-utilities-rs`, `approx`), prune unused deps in `abop-gui`, and align `iced` versions across the workspace.
- Logging: Standardize on `tracing` + `tracing-subscriber` (+ `tracing-log` temporarily to capture `log::*`), or swap fully to `log` + `env_logger`. Recommendation: use `tracing`.
- Duplicate/unused modules: Remove or wire `update.rs`; remove duplicate `Theme` enum in `app.rs` in favor of `theme::ThemeMode`. Consolidate message variants like `Stop`, `StopPlayback`, `PlaybackStopped`.
- Theme wiring: Have `App::theme()` return the theme derived from `ThemeMode` and ensure `main.rs` honors it. Avoid direct token access in views; prefer a small theme facade or consistent `IcedTheme` use.
- build.rs: Current script writes invalid Rust and isn’t used; remove or replace with a gated `include_bytes!` embedding path in `assets.rs`.
- Router: Cap history length, dedupe consecutive entries, support `replace(route)`.
- Tests/lints: Enable clippy in CI, fix obvious lints, add crate-level `#![deny(unsafe_code)]` (and optionally warn on missing docs).
- Medium+ items: Split messages into domain enums, typed IDs and sort enums, prefer `iced::Task` for UI async; add font strategy (`embed-fonts` feature); consolidate GUI errors with `thiserror`.

---

## Four-week plan

### Week 1 — Baseline hygiene and visibility

Deliverables
- Dependencies
  - Pin wildcard versions and move common pins to `[workspace.dependencies]`.
  - Remove unused deps from `abop-gui` after a quick references check.
- Logging
  - Standardize on `tracing` with `tracing-subscriber` and `RUST_LOG`; add `tracing-log` to capture `log` macros during migration.
  - Document common `RUST_LOG` presets in README.
- build.rs / assets
  - Remove or disable the broken `build.rs`. If kept, add minimal `cargo:rerun-if-changed` only for used assets.
- Theme wiring
  - Make `App::theme()` return the selected `theme::ThemeMode` theme and ensure `main.rs` uses it.
- CI/tooling
  - Add GitHub Actions: fmt + clippy + nextest.
  - Add crate-level lints in `abop-gui/lib.rs`: `#![deny(unsafe_code)]` and a conservative clippy config.

Acceptance checks
- Clean checkout builds and `cargo nextest run --all` passes.
- `RUST_LOG` controls log verbosity consistently.
- UI visibly switches between light/dark/system as expected.

PR slices
- PR#1 deps: pin + prune.
- PR#2 logging: tracing init + README note.
- PR#3 theme wiring minimal.
- PR#4 remove/disable build.rs.

Risks
- Version pin conflicts; resolve by aligning within workspace or feature flags.

---

### Week 2 — Surface cleanup and low-risk refactors

Deliverables
- Remove duplicates/dead code
  - Remove or wire `update.rs` (confirm unused by search); remove duplicate `Theme` in `app.rs` and use `theme::ThemeMode` everywhere.
  - Consolidate redundant message variants.
- Router improvements
  - Cap history (e.g., 64), dedupe consecutive identical routes, add `replace(route)`.
- Tests
  - Router unit tests for push/pop/replace, cap, dedupe.
  - Theming test verifying `System/Light/Dark` mapping.

Acceptance checks
- No references to removed types; unit tests green.
- Router tests cover cap and dedupe behavior.

PR slices
- PR#5 remove duplicate theme + propagate.
- PR#6 message variant consolidation.
- PR#7 router improvements + tests.

Risks
- Message consolidation may require small pattern-match updates across screens.

---

### Week 3 — Message/command design and async lifecycles

Deliverables
- Message structure
  - Introduce domain sub-enums: `UiMessage`, `LibraryMessage`, `PlaybackMessage`, `SystemMessage`, wrapped by top-level `Message`.
  - Introduce typed IDs (newtype/`Uuid`) for audiobook IDs; add `SortKey` and `Order` enums.
  - Migrate high-traffic flows first; keep adapters for legacy variants during transition.
- Async/task management
  - Prefer `iced::Task::perform` for UI-triggered async work; keep `tokio::spawn` for background services with explicit cancellation tokens.
  - Ensure iced tokio feature is configured if `tokio` remains; document runtime expectations.

Acceptance checks
- Core flows compile and run with new message types; adapters deprecated but present.
- Example UI action uses `Task::perform` and cancels appropriately on navigation.

PR slices
- PR#8 introduce new message enums + adapters.
- PR#9 typed IDs and sort enums + targeted migrations.
- PR#10 async migration for 1–2 flows + example.

Risks
- Touches many files; mitigate via small PRs and staged migration.

---

### Week 4 — Fonts/assets, errors, and quality gates

Deliverables
- Fonts strategy
  - Add feature flag `embed-fonts`: embed Roboto via `include_bytes!` and register in `assets::register_fonts`.
  - Default to system fonts; document Windows setup; prune unused scripts if not needed.
- Error handling
  - Introduce `GuiError` with `thiserror` and central mapping to user-facing messages.
- CI/tooling
  - Add `cargo-deny` (licenses/duplicates) and coverage (`cargo-llvm-cov` or similar).
  - Expand tests for message routing, theming edge cases.
- Public API
  - Reduce `pub` surface in `abop-gui`; use `pub(crate)` where possible; optional `prelude` module for intended re-exports.

Acceptance checks
- Build passes with and without `embed-fonts` feature; fonts render properly.
- `cargo-deny` passes in CI; coverage artifact produced for GUI crate.
- Error paths render consistent user notifications.

PR slices
- PR#11 fonts feature + docs.
- PR#12 GUI error type + mapping.
- PR#13 CI extras: deny + coverage.
- PR#14 public API tightening.

Risks
- Cross-platform font behavior; gate with feature and provide fallbacks.

---

## Backlog (post-roadmap)

- Theme/token facade for views (hide `material_tokens`); optional OS theme detection for `ThemeMode::System`.
- Component organization: presentational vs. container components; keep props simple and equatable.
- Performance: avoid sending large collections across messages/commands; consider `smallvec` for tiny hot-path collections; audit `Arc` usage.
- Packaging: Windows icon/metadata, CHANGELOG, semantic versioning for the crate.

## Operating model

- Branching: continue on `tech-debt-repayment` with small, focused PRs; merge early/often.
- Labels: chore/tech-debt, refactor, test, ci, docs.
- PR size: aim < 400 LOC; each PR must pass build, clippy (no new warns), nextest; minimal UI smoke (theme switch).

## Requirements coverage

- Capture the evaluated tech-debt plan in-repo: Done (this file).
- Provide milestones, PR slices, acceptance checks, and risks: Done.
