# Roadmap: ABOP Frontend Feature Completion

**Created:** 2025-01-24
**Phases:** 2

---

## Progress: 0/2 phases complete

| Phase | Status | Progress |
|-------|--------|----------|
| 1     | ✗ Not started | 0% |
| 2     | ○ Pending | 0% |

---

## Phase 1: Fix Test Infrastructure

**Duration:** 4-6 hours
**Goal:** Fix all 34 failing tests to enable feature verification

**Requirements:** TEST-01, TEST-02, TEST-03

**Success criteria:**
1. All audiobook repository tests pass (9/9)
2. All progress repository tests pass (27/27)
3. All minor tests pass (2/2)
4. Tests use proper `TestDatabase::new()` initialization
5. Database migrations run before test execution

**Plans:**
- [ ] 01-01-PLAN.md — Migrate audiobook repository tests
- [ ] 01-02-PLAN.md — Migrate progress repository tests
- [ ] 01-03-PLAN.md — Fix minor failing tests

**Blocked by:** None

**Description:**
Currently, 34 tests are failing due to broken test infrastructure. The tests don't initialize the database properly or run migrations. This phase migrates the tests to use centralized test utilities (`TestDatabase::new()`, `TestDataFactory`, `TestAssertions`) from `abop-core/src/test_utils/db.rs`.

**Files to modify:**
- `abop-core/src/db/repositories/audiobook/tests.rs` (684 lines)
- `abop-core/src/db/repositories/progress/tests.rs` (504 lines)
- `abop-core/src/scanner/file_discovery/tests.rs` (minor)
- `abop-gui/src/utils/path_utils/tests.rs` (minor)

---

## Phase 2: Verify End-to-End Features

**Duration:** 2-3 hours
**Goal:** Verify all frontend features work end-to-end with passing tests

**Requirements:** LIB-01 through LIB-05, PLAY-01 through PLAY-05, PROC-01 through PROC-03, SETT-01 through SETT-03

**Success criteria:**
1. Library scanning displays audiobooks in GUI
2. Search/filter functionality works
3. Audio playback starts and plays correctly
4. Player state updates in real-time
5. Progress tracking works during playback
6. Audio processing (mono conversion) creates output files
7. Processing progress displays during conversion
8. All settings toggles work (theme, auto-save, scan subdirs)

**Plans:**
- [ ] TBD (created by /gsd-plan-phase 2)

**Blocked by:** Phase 1

**Description:**
After all tests pass, verify each feature works end-to-end by manually testing the application workflows. This confirms the frontend promises are backed by working backend code.

**Workflows to test:**
1. Scan a directory → audiobooks appear in database and GUI
2. Search/filter audiobooks → results display correctly
3. Select audiobook → click play → audio plays
4. During playback → player state updates → progress tracks
5. Select audiobooks → convert to mono → output files created
6. Toggle settings → UI responds correctly

---

## Traceability Summary

**Total v1 requirements:** 18
**Requirements mapped to phases:** 18 ✓
**Unmapped:** 0

| Phase | Requirements |
|-------|--------------|
| Phase 1 | TEST-01, TEST-02, TEST-03 (3 requirements) |
| Phase 2 | LIB-01 through LIB-05, PLAY-01 through PLAY-05, PROC-01 through PROC-03, SETT-01 through SETT-03 (15 requirements) |

---

## Notes

- **Phase 1 is blocking:** Cannot verify features reliably while 34 tests fail
- **Backend appears complete:** Library management, playback, processing, and settings code exists
- **Focus:** Test infrastructure migration, not new feature development
- **Risk:** Test migration requires careful refactoring to preserve test behavior

---
*Roadmap created: 2025-01-24*