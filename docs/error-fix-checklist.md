# Error Fix Checklist

## 1. Import and Module Issues
- [ ] Fix `iced_font_awesome` imports in:
  - [ ] `abop-gui/src/components/icon_support.rs`
  - [ ] `abop-gui/src/components/icons.rs`
  - [ ] `abop-gui/src/styling/material/components/button_style/functions.rs`
  - Replace with `font_awesome` as suggested by compiler

- [ ] Add missing imports:
  - [ ] Add `use abop_core::models::Library` in `abop-gui/src/messages.rs`
  - [ ] Add `use crate::state::TaskType` in `abop-gui/src/messages.rs`
  - [ ] Add `use crate::components::task_manager::TaskManager` in `abop-gui/src/handlers/ui_state.rs`

## 2. Theme and Style Issues
- [ ] Fix theme-related imports and usage:
  - [ ] Add proper imports for `Button` and `Text` in `abop-gui/src/components/status.rs`
  - [ ] Update style calls to use correct theme types
  - [ ] Fix `container_style` calls in `task_manager.rs` to use correct theme mode

## 3. Struct and Enum Field Updates
- [ ] Update `ScanSummary` field access:
  - [ ] Replace `audiobooks` with `new_files` and `updated_files`
  - [ ] Update `processed_files` to `processed`
  - [ ] Update `processed_count` to `processed`
  - [ ] Update `error_count` to `errors`

- [ ] Update `ScanProgress` field access:
  - [ ] Replace `current_file` with appropriate field
  - [ ] Update `processed` and `total` field access
  - [ ] Update `throughput` field access
  - [ ] Update `eta` field access
  - [ ] Update `progress_percentage` field access

## 4. Message Enum Updates
- [ ] Add missing variants to `Message` enum:
  - [ ] Add `ScanProgressEnhanced`
  - [ ] Add `CancelScan`
  - [ ] Add `Player` to `ViewType`

## 5. Task Manager Updates
- [ ] Fix `TaskManager::view` function:
  - [ ] Update function signature to match usage
  - [ ] Fix field access for `TaskInfo`
  - [ ] Update style closure to take correct number of arguments

## 6. Library Model Updates
- [ ] Update `Library` struct usage:
  - [ ] Remove `created_at` and `updated_at` fields
  - [ ] Update field access in `commands/library.rs`

## 7. Type Mismatch Fixes
- [ ] Fix `Result` type usage:
  - [ ] Update `Task<Result<ScanResult, AppError>>` to use correct type alias
  - [ ] Fix `ScanError` vs `AppError` conversions

## 8. Unstable Feature Fixes
- [ ] Replace unstable `let` expressions in conditions with stable alternatives:
  - [ ] `abop-gui/src/commands/library.rs`
  - [ ] `abop-gui/src/handlers/ui_state.rs`
  - [ ] `abop-gui/src/styling/material/components/feedback/notification.rs`
  - [ ] `abop-gui/src/styling/material/components/menu_item_style.rs`
  - [ ] `abop-gui/src/styling/material/components/menus/autocomplete.rs`
  - [ ] `abop-gui/src/styling/material/components/widgets/material_button.rs`
  - [ ] `abop-gui/src/styling/material/elevation/context.rs`
  - [ ] `abop-gui/src/styling/plugins.rs`

## 9. Clean Up Warnings
- [ ] Remove unused imports:
  - [ ] `error::AppError` and `scanner::progress::ScanProgress` in `library.rs`
  - [ ] `ScanProgress` in `data_updates.rs`
  - [ ] `Audiobook` in `ui_state.rs`
  - [ ] `Style` in `views/mod.rs`
  - [ ] `DatabaseConfig` and `Arc` in `scanner.rs`
  - [ ] Various unused imports in `task_manager.rs`

## 10. Documentation
- [ ] Add missing documentation:
  - [ ] `to_string` method in `error.rs`
  - [ ] Fields in `library_scanner.rs`
  - [ ] Methods in `progress.rs`
  - [ ] Methods in `result.rs`

## Priority Order
1. Import and Module Issues (Blocking)
2. Struct and Enum Field Updates (Core functionality)
3. Message Enum Updates (UI functionality)
4. Type Mismatch Fixes (Type safety)
5. Task Manager Updates (UI functionality)
6. Theme and Style Issues (UI appearance)
7. Library Model Updates (Data model)
8. Unstable Feature Fixes (Stability)
9. Clean Up Warnings (Code quality)
10. Documentation (Code quality)

## Notes
- All changes should maintain compatibility with `iced 0.13.1`
- Keep `web-sys` version at `0.3.69` or higher
- Ensure all async/await patterns are properly implemented
- Maintain backward compatibility where possible 