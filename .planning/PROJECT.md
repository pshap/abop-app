# ABOP Frontend Feature Completion Plan

## Executive Summary

**Goal**: Implement and verify as many frontend features as reasonably possible.

**Current State**: Frontend promises are backed by partially implemented backend code. The main blocker is **34 failing tests** due to broken test infrastructure, not missing features.

**Success Criteria**: All features work end-to-end with passing tests.

---

## Frontend Feature Promises

### Library Management (Priority 1)

**GUI Features:**
- ✅ Browse audiobook library with table display
- ✅ Search/filter audiobooks by title, author, narrator
- ✅ Select multiple audiobooks
- ✅ Sort audiobooks by various columns
- ✅ Quick scan and full library scan
- ✅ Recent directories dialog
- ✅ Status display during scanning

**Backend Status:**
- ✅ `LibraryScanner` in `abop-core` - multi-threaded directory scanning
- ✅ Scan progress reporting via `ScanProgress` messages
- ✅ Database operations via `LibraryRepository` and `AudiobookRepository`
- ✅ Command handler: `handle_library_command()` in `abop-gui/src/commands/library.rs`
- ✅ View: `library_view()` with table, search bar, status display

**Test Coverage:**
- ❌ **9 audiobook repository tests FAILING** - broken test infrastructure (disk I/O errors)
- ✅ Core library scanner testsPASSING
- ✅ Database testsPASSING

**Blocking Issues:**
1. Audiobook repository tests don't initialize database properly
2. Tests using legacy infrastructure without migrations

### Audio Playback (Priority 2)

**GUI Features:**
- ✅ Play/pause, stop, next, previous controls
- ✅ Audio toolbar for selected audiobooks
- ✅ Status display showing player state and current file
- ✅ Progress tracking via progress caching

**Backend Status:**
- ✅ `ThreadSafeAudioPlayer` global instance in `abop-gui/src/audio/player.rs`
- ✅ Symphonia/Rodio audio decoding and playback
- ✅ Player state management (Playing, Paused, Stopped)
- ✅ Command handler: `handle_audio_command()` in `abop-gui/src/commands/audio.rs`
- ✅ Async playback: `play_selected_audio()`

**Test Coverage:**
- ✅ Audio player testsPASSING
- ✅ Audio decoder testsPASSING

**Blocking Issues:**
None - playback appears fully implemented!

### Audio Processing (Priority 3)

**GUI Features:**
- ✅ Convert selected audiobooks to mono
- ✅ Processing progress display
- ✅ Processing status messages

**Backend Status:**
- ✅ `AudioProcessingPipeline` with channel mixing
- ✅ `convert_selected_to_mono()` in `abop-gui/src/audio/processing.rs`
- ✅ Symphonia audio decoding and Rodio-compatible output
- ✅ File I/O with `AudioFileProcessor`

**Test Coverage:**
- ✅ Audio processing testsPASSING
- ✅ Channel mixer testsPASSING

**Blocking Issues:**
None - processing appears fully implemented!

### Settings (Priority 4)

**GUI Features:**
- ✅ Toggle light/dark theme
- ✅ Toggle auto-save library
- ✅ Toggle scan subdirectories

**Backend Status:**
- ✅ `ThemeMode` enum and state management
- ✅ Settings UI with MD3 switch components
- ✅ Message handlers for toggles

**Test Coverage:**
- ✅ GUI component testsPASSING

**Blocking Issues:**
None - settings are UI-only feature, no persistence needed yet

---

## Critical Test Failures Blocking Verification

### Audiobook Repository Tests (9 FAILING)
**Location**: `abop-core/src/db/repositories/audiobook/tests.rs`

**Failure Pattern**: "disk I/O error" or "no such table: libraries"

**Root Cause**: Tests use legacy infrastructure that doesn't:
- Call `TestDatabase::new()` to create proper test DB
- Run database migrations
- Initialize schema

**Impact**: Can't verify audiobook CRUD operations work

### Progress Repository Tests (27 FAILING)
**Location**: `abop-core/src/db/repositories/progress/tests.rs`

**Failure Pattern**: "disk I/O error"

**Root Cause**: Same as audiobook tests - broken test infrastructure

**Impact**: Can't verify progress tracking works

### Scanner Permission Test (1 FAILING)
**Location**: `abop-core/src/scanner/file_discovery/tests.rs`

**Failure**: Permission denied handling test

**Impact**: Minor - edge case handling

### GUI Path Utils Test (1 FAILING)
**Location**: `abop-gui/src/utils/path_utils/tests.rs`

**Failure**: Trait implementation test

**Impact**: Minor - utility function

---

## Implementation Plan

### Phase 1: Fix Test Infrastructure (Required for Verification)

**Goal**: Fix all 34 failing tests to enable feature verification

**Tasks**:

1. **Migrate Audiobook Repository Tests** (CRITICAL - 9 tests)
   - File: `abop-core/src/db/repositories/audiobook/tests.rs`
   - Current: 684 lines, broken infrastructure
   - Action:
     - Replace `TestContext::setup()` with `TestDatabase::new()`
     - Use `TestDataFactory::create_test_library()`, `create_test_audiobook()`
     - Use `TestAssertions::assert_audiobook_equals()` for assertions
     - Ensure database migrations run
   - Expected result: All 9 tests passing

2. **Migrate Progress Repository Tests** (CRITICAL - 27 tests)
   - File: `abop-core/src/db/repositories/progress/tests.rs`
   - Current: 504 lines, broken infrastructure
   - Action: Same migration pattern as audiobook tests
   - Expected result: All 27 tests passing

3. **Fix Scanner Permission Test** (MINOR - 1 test)
   - File: `abop-core/src/scanner/file_discovery/tests.rs`
   - Action: Requires actual permission denied scenario
   - Expected result: Test passing or skip if scenario can't be created

4. **Fix GUI Path Utils Test** (MINOR - 1 test)
   - File: `abop-gui/src/utils/path_utils/tests.rs`
   - Action: Fix trait implementation issue
   - Expected result: Test passing

**Success Metrics**:
- ✅ 0/34 tests failing
- ✅ All core tests passing
- ✅ Repository tests pass with proper migrations

### Phase 2: Verify End-to-End Features

After tests pass, verify each feature works end-to-end:

1. **Library Management Flow**
   - Test: Scan a directory
   - Verify: Audiobooks appear in database
   - Verify: GUI displays scanned audiobooks
   - Verify: Search/filter works

2. **Audio Playback Flow**
   - Test: Select audiobook and click play
   - Verify: Audio starts playing
   - Verify: Player state updates
   - Verify: Progress tracking works

3. **Audio Processing Flow**
   - Test: Select audiobooks and convert to mono
   - Verify: Output files created
   - Verify: Processing progress displays

**Success Metrics**:
- ✅ All features work manually
- ✅ No runtime errors
- ✅ User can complete workflows

---

## Implementation Details

### Test Migration Pattern

**Old Pattern (Broken)**
```rust
let db = Database::open_in_memory().unwrap();
// Try to use DB without migrations
db.add_library(...).unwrap();
```

**New Pattern (Working)**
```rust
let test_db = TestDatabase::new().await;
// Centralized test utilities handle DB setup:
// - Create test database with migrations
// - Provide factory methods for test data
// - Provide assertion helpers
```

### Critical Files to Update

1. `abop-core/src/db/repositories/audiobook/tests.rs` (684 lines)
2. `abop-core/src/db/repositories/progress/tests.rs` (504 lines)

Both files will be rewritten to use:
- `TestDatabase::new()` for database setup
- `TestDataFactory` methods for creating test data
- `TestAssertions` for clean assertions
- Centralized test utilities in `abop-core/src/test_utils/db.rs`

---

## Timeline and Effort

### Phase 1: Fix Test Infrastructure
- **Effort**: 4-6 hours
- **Files**: 2 test files (audiobook, progress)
- **Tests**: 36 tests (broken → passing)
- **Risk**: Medium - requires careful test migration

### Phase 2: Verify Features
- **Effort**: 2-3 hours
- **Activities**: Manual testing, smoke tests
- **Features**: 4 major features verified

**Total**: 6-9 hours to complete

---

## Success Definition

**Definition of Success**:
1. ✅ All 34 failing tests pass (0 failures)
2. ✅ Library scanning and display works end-to-end
3. ✅ Audio playback works end-to-end
4. ✅ Audio processing works end-to-end
5. ✅ Settings UI works (no persistence needed)

**What's NOT in Scope**:
- Advanced audio processing effects (beyond mono conversion)
- Progress persistence to disk
- Import/export functionality
- Advanced metadata editing
- Cover art editing

---

## Next Steps

1. **Fix audiobook repository tests** (9 tests)
2. **Fix progress repository tests** (27 tests)
3. **Fix minor permission and utils tests** (2 tests)
4. **Verify all features work end-to-end**
5. **Document any remaining limitations**

All backend code appears complete - we're primarily fixing test infrastructure to verify functionality!