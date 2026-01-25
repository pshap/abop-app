# Roadmap: ABOP Codebase Improvements

**Created:** 2026-01-25
**Phases:** 5

---

## Progress: 1/5 phases in progress

| Phase | Status | Requirements |
|-------|--------|--------------|
| 1     | 🔄 Gap Closure | QUAL-01 through QUAL-04 (8/8 plans, 4 complete + 4 gap closure) |
| 2     | ○ Pending | DEBT-01 through DEBT-04 |
| 3     | ○ Pending | PERF-01 through PERF-04 |
| 4     | ○ Pending | DOCS-01 through DOCS-04 |
| 5     | ○ Pending | TEST-01 through TEST-04 |

---

## Phase 1: Code Quality Improvements 🔄

**Goal:** Reduce duplication, improve modularity, ensure consistent style

**Requirements:** QUAL-01, QUAL-02, QUAL-03, QUAL-04

**Status:** Gap closure in progress - 8 total plans (4 complete + 4 gap closure)

**Initial completion:** 2026-01-25 (4/24 must-haves verified)
**Gap closure:** 2026-01-25 (addressing critical quality issues)

**Results from initial plans:**
- Large files split into focused modules (test files, styling, audio processing)
- Clippy fixes applied (zero warnings)
- Code consistently formatted
- **Critical gaps discovered:** 361 unwrap() calls, 21 TODO/FIXME, 15+ files >300 lines

**Verification gaps identified:**
- QUAL-01: 21 TODO/FIXME comments remain, large files still >300 lines
- QUAL-02: 361 unwrap() calls remain across codebase (critical safety issue)
- QUAL-03: 15+ files still exceed 300-line target despite improvements
- QUAL-04: Clippy and test verification incomplete due to timeouts

**Description:**
Foundational code quality improvements achieved modularity gains but failed to address core quality issues. Gap closure plans target the most critical blockers: unsafe unwrap() calls, remaining large files, and incomplete modularization.

**Key areas completed:**
- ✅ Split test files into focused modules (audiobook, library, progress)
- ✅ Split styling files into focused modules (components, tokens, containers)
- ✅ Split audio processing files into focused modules (resampler, normalizer, etc.)
- ✅ Applied cargo fmt and initial clippy fixes

**Gap closure in progress:**
- 🔄 Split remaining large files (>500 lines) into focused modules
- 🔄 Replace critical unwrap() calls with proper error handling
- 🔄 Complete progress test modularization (missing mod.rs)
- 🔄 Address all TODO/FIXME comments
- 🔄 Verify clippy compliance and test suite health

**Plans:**
- [x] 01-01-PLAN.md — Split test files into focused modules
- [x] 01-02-PLAN.md — Split styling files into focused modules  
- [x] 01-03-PLAN.md — Split audio processing files into focused modules
- [x] 01-04-PLAN.md — Apply clippy fixes and formatting
- [ ] 01-05-PLAN.md — Split large files (>500 lines) into focused modules
- [ ] 01-06-PLAN.md — Replace critical unwrap() calls with error handling
- [ ] 01-07-PLAN.md — Complete modularization and address TODO/FIXME
- [ ] 01-08-PLAN.md — Verify clippy compliance and test suite health

---

## Phase 2: Technical Debt Reduction

**Goal:** Safer code with better error handling and cleanup

**Requirements:** DEBT-01, DEBT-02, DEBT-03, DEBT-04

**Success criteria:**
1. 0 instances of unsafe unwrap() outside test code
2. All error messages provide actionable context
3. No dead code or unused imports
4. Consistent error types across modules

**Description:**
Replace unsafe unwrap() calls with proper Result/Option handling. Improve error messages to actually help users understand and fix issues. Remove deprecated code and standardize error propagation patterns.

**Key areas:**
- Use `clippy::unwrap_used` to find all unwrap() calls
- Replace with proper Result propagation and user-friendly messages
- Clean up unused code identified by compiler
- Consolidate similar error types into unified types

---

## Phase 3: Performance Optimizations

**Goal:** Improve performance and reduce resource usage

**Requirements:** PERF-01, PERF-02, PERF-03, PERF-04

**Success criteria:**
1. Hot paths 20% faster (benchmark验证)
2. Binary size reduced by measurable amount
3. No performance regressions introduced
4. Identified bottlenecks resolved

**Description:**
Profile the application to find hot paths and bottlenecks. Focus on realistic workloads: scanning large directories, processing audio files, rendering GUI. Use criterion benchmarks to measure improvements.

**Key areas:**
- Use flamegraph/perf to profile real usage
- Optimize audio processing pipeline
- Reduce allocations in hot paths
- Benchmark before and after each optimization
- Profile GUI rendering for improvements

---

## Phase 4: Documentation Improvements

**Goal:** Comprehensive documentation for easy onboarding

**Requirements:** DOCS-01, DOCS-02, DOCS-03, DOCS-04

**Success criteria:**
1. All public APIs documented with examples
2. README accurately describes current features
3. Complex algorithms have explanation comments
4. New contributors understand codebase structure

**Description:**
Add comprehensive documentation to public APIs, update project documentation, and explain complex algorithms. Make the codebase approachable for new contributors.

**Key areas:**
- Run `cargo doc --open` and document undocumented items
- Update README with current capabilities
- Add comments explaining non-obvious logic
- Document architecture patterns and module responsibilities

---

## Phase 5: Test Quality Improvements

**Goal:** Test coverage that matters, fast and maintainable

**Requirements:** TEST-01, TEST-02, TEST-03, TEST-04

**Success criteria:**
1. Unnecessary tests removed
2. Critical paths well-tested
3. Test names clearly describe what's tested
4. Test suite runs in < 5s

**Description:**
Review the 894 passing tests to identify redundancies, test what matters, and ensure tests are fast and maintainable. Focus on preventing real bugs, not just increasing coverage metrics.

**Key areas:**
- Identify redundant or over-specific tests
- Ensure core functionality (audio playback, file scanning) has good coverage
- Rename vague tests to be descriptive
- Slow tests move to integration test suite
- Remove edge case tests that don't add value

---

## Traceability Summary

**Total v1 requirements:** 20
**Requirements mapped to phases:** 20 ✓
**Unmapped:** 0

| Phase | Requirements |
|-------|--------------|
| Phase 1 | QUAL-01, QUAL-02, QUAL-03, QUAL-04 |
| Phase 2 | DEBT-01, DEBT-02, DEBT-03, DEBT-04 |
| Phase 3 | PERF-01, PERF-02, PERF-03, PERF-04 |
| Phase 4 | DOCS-01, DOCS-02, DOCS-03, DOCS-04 |
| Phase 5 | TEST-01, TEST-02, TEST-03, TEST-04 |

---

## Notes

- **Sequential execution needed**: Phases build on each other (e.g., fix unwrap() before performance work)
- **Test-driven**: Keep tests passing throughout all improvements
- **Incremental commits**: Commit after each improvement, revert if it breaks something

---
*Roadmap created: 2026-01-25*