# Database Module Refactoring Plan

**Target File:** `abop-core/src/db/mod.rs`  
**Current Size:** 621 lines (exceeds 300-line guideline by 321 lines)  
**Status:** Research Complete, Ready for Implementation  
**Date:** October 14, 2025

---

## Executive Summary

The `Database` struct in `mod.rs` serves as a **facade** over connection pooling and repository operations. While the module already has extensive submodule organization (11 submodules), the main file is bloated with:

1. **Legacy CRUD methods** (path-based, pre-repository pattern)
2. **Convenience wrappers** around repository methods
3. **Internal helper methods** (cache management, schema init)
4. **Mixed concerns** (app database paths, connection pooling, library ID resolution)

**Goal:** Reduce `mod.rs` to a thin facade (<150 lines) by extracting logical groupings into focused modules while preserving the public API.

---

## Current Architecture Analysis

### File Structure
```
abop-core/src/db/
├── mod.rs (621 lines) ← TARGET
├── connection.rs (Enhanced connection wrapper)
├── operations.rs (Generic execute/query operations)
├── error.rs (Database error types)
├── helpers.rs (SQL query builders, utilities)
├── mappers.rs (Row → Model conversions)
├── migrations.rs (Schema migrations)
├── retry.rs (Retry logic)
├── health.rs (Connection health checks)
├── statistics.rs (Query statistics)
├── datetime_serde.rs (DateTime serialization)
├── migrations/ (SQL migration files)
├── queries/ (SQL query definitions)
└── repositories/ (Repository pattern implementations)
    ├── audiobook.rs
    ├── library.rs
    └── progress.rs
```

### Database Struct Components

**Core Fields:**
```rust
pub struct Database {
    pool: Arc<Pool<SqliteConnectionManager>>,
    operations: EnhancedConnection,
    state: Arc<Mutex<DatabaseState>>, // Cache for library_id lookups
}

struct DatabaseState {
    library_cache: HashMap<PathBuf, String>,
}
```

**Method Categories (26 public methods):**

1. **Constructors** (3 methods)
   - `new(config)` - Main constructor with pooling
   - `in_memory()` - Test database
   - `open(path)` / `open_app_database()` - File-based databases

2. **Legacy CRUD - Library** (2 methods)
   - `add_library(library)` - Direct SQL insert
   - `get_libraries()` - Delegates to repository

3. **Legacy CRUD - Audiobook** (6 methods)
   - `add_audiobook(audiobook)` - Path-based, direct SQL
   - `add_audiobooks_bulk(audiobooks)` - Bulk insert
   - `get_audiobooks(library_path)` - Path-based query
   - `get_audiobook(path)` - Single fetch by path
   - `delete_audiobook(path)` - Path-based delete
   - `delete_library_audiobooks(library_path)` - Bulk delete

4. **Repository-Based CRUD** (6 methods)
   - `get_audiobooks_in_library(library_id)` - Delegates to repo
   - `get_audiobooks_in_library_paginated(...)` - Pagination
   - `count_audiobooks_in_library(library_id)` - Count query
   - `get_all_audiobooks()` - All audiobooks
   - `find_library_by_path(path)` - Library lookup
   - `add_library_with_path(name, path)` - Create if not exists

5. **Repository Factories** (4 methods)
   - `audiobook_repository()` - Returns `AudiobookRepository`
   - `library_repository()` - Returns `LibraryRepository`
   - `progress_repository()` - Returns `ProgressRepository`
   - `libraries()` - Alias for `library_repository()`

6. **Infrastructure** (5 methods)
   - `connect()` - Get pooled connection
   - `get_library_id(path)` - Internal cache lookup
   - `get_app_database_path()` - Static path helper
   - `init_schema(conn)` - Schema initialization
   - Internal: Cache management in `DatabaseState`

---

## Dependency Analysis

### External Consumers (Low Coupling ✅)
**Only 1 direct consumer:** `abop-gui/src/commands/library.rs`

**Methods Used in GUI:**
```rust
// From library.rs command handlers
Database::open_app_database()
Database::library_repository()
Database::add_library_with_path(name, path)
Database::audiobook_repository()
```

**Implications:**
- Minimal breaking change risk
- Can safely refactor internals
- Must preserve these 4 method signatures

### Internal Usage
- `scanner/mod.rs`: Re-exports `Database` but doesn't use methods directly
- No other core crate usage found

---

## Refactoring Strategy

### Proposed Module Split

#### 1. **`db/facade.rs`** (New) - Core Database Facade
**Purpose:** Main `Database` struct with minimal coordination logic  
**Contents:**
- Struct definition (`Database`, `PoolConfig`, `DatabaseState`)
- Constructors (`new`, `in_memory`)
- Repository factory methods
- Connection pooling (`connect()`)
- Cache management (`get_library_id`, state operations)
- Lines: ~150

**Rationale:** This becomes the new "thin" mod.rs, keeping only essential facade logic.

#### 2. **`db/legacy_crud.rs`** (New) - Legacy Path-Based Operations
**Purpose:** Backward-compatible CRUD methods (deprecation candidates)  
**Contents:**
- `add_audiobook(audiobook)` - Path-based insert
- `add_audiobooks_bulk(audiobooks)` - Bulk insert
- `get_audiobooks(library_path)` - Path-based query
- `get_audiobook(path)` - Single fetch
- `delete_audiobook(path)` - Path-based delete
- `delete_library_audiobooks(library_path)` - Bulk delete
- `add_library(library)` - Direct insert
- Lines: ~200

**Rationale:** These methods are inconsistent with repository pattern. Grouping them signals technical debt and makes future deprecation easier.

#### 3. **`db/convenience.rs`** (New) - Repository Wrapper Methods
**Purpose:** High-level convenience methods delegating to repositories  
**Contents:**
- `get_audiobooks_in_library(library_id)`
- `get_audiobooks_in_library_paginated(...)`
- `count_audiobooks_in_library(library_id)`
- `get_all_audiobooks()`
- `get_libraries()`
- `find_library_by_path(path)`
- `add_library_with_path(name, path)` - Used by GUI
- Lines: ~120

**Rationale:** These are thin wrappers for ergonomics. Could be deprecated in favor of direct repository usage, but kept for backward compatibility.

#### 4. **`db/app_database.rs`** (New) - Application Database Utilities
**Purpose:** Centralized app database path management  
**Contents:**
- `open_app_database()` - Used by GUI
- `get_app_database_path()` - Path resolution
- Related path/directory helpers
- Lines: ~50

**Rationale:** Application-level database concerns separate from core database logic.

#### 5. **`db/schema.rs`** (New) - Schema Initialization
**Purpose:** Database schema definitions and setup  
**Contents:**
- `init_schema(conn)` - Currently unused but important
- Schema SQL strings
- Migration triggers (if needed)
- Lines: ~100

**Rationale:** Schema is architectural concern, separate from runtime operations.

### Updated `mod.rs` Structure
```rust
// db/mod.rs (New: ~50 lines)

mod connection;
mod operations;
mod error;
mod helpers;
mod mappers;
mod migrations;
mod retry;
mod health;
mod statistics;
mod datetime_serde;

// New modules
mod facade;           // Core Database struct
mod legacy_crud;      // Path-based operations
mod convenience;      // Repository wrappers
mod app_database;     // App DB utilities
mod schema;           // Schema initialization

pub mod repositories;
pub mod queries;

// Re-exports
pub use facade::{Database, PoolConfig};
pub use error::DatabaseError;
pub use connection::EnhancedConnection;
// ... other re-exports
```

---

## Implementation Plan

### Phase 1: Extract Schema (Low Risk)
1. Create `db/schema.rs`
2. Move `init_schema()` method
3. Update references in `Database::new()`
4. Run tests: `cargo nextest run --all --no-capture`

### Phase 2: Extract App Database Utilities (GUI-Critical)
1. Create `db/app_database.rs`
2. Move `open_app_database()`, `get_app_database_path()`
3. Add as methods to `Database` impl via import
4. **Verify GUI still compiles:** `cargo check -p abop-gui`
5. Run tests

### Phase 3: Extract Legacy CRUD (High Line Count)
1. Create `db/legacy_crud.rs`
2. Move path-based methods (7 methods)
3. Add deprecation warnings: `#[deprecated(since = "0.2.0", note = "Use repository methods instead")]`
4. Update internal references
5. Run tests

### Phase 4: Extract Convenience Wrappers
1. Create `db/convenience.rs`
2. Move repository delegation methods (7 methods)
3. Ensure `add_library_with_path()` preserved (GUI dependency)
4. Run tests

### Phase 5: Create Facade & Refactor mod.rs
1. Create `db/facade.rs` with core struct + constructors + factories
2. Update `mod.rs` to import from `facade`
3. Re-export all public APIs through `mod.rs`
4. Run tests
5. Verify metrics: `pwsh scripts/metrics_long_rust_files.ps1`

### Phase 6: Final Validation
1. Full test suite: `cargo nextest run --all --no-capture`
2. Build GUI: `cargo build -p abop-gui`
3. Run GUI: `cargo run -p abop-gui`
4. Check for warnings: `cargo clippy --workspace`
5. Format: `cargo fmt --all`

---

## API Preservation Checklist

Must preserve these exact signatures for GUI compatibility:

```rust
impl Database {
    // Used by GUI
    pub fn open_app_database() -> Result<Self>
    pub fn library_repository(&self) -> LibraryRepository
    pub fn audiobook_repository(&self) -> AudiobookRepository
    pub fn add_library_with_path(&self, name: &str, path: PathBuf) -> Result<String>
    
    // Repository factories
    pub fn progress_repository(&self) -> ProgressRepository
    pub fn libraries(&self) -> LibraryRepository
    
    // Infrastructure
    pub fn connect(&self) -> Result<PooledConnection<...>>
}
```

---

## Success Criteria

- [ ] `db/mod.rs` reduced to <150 lines (from 621)
- [ ] All tests pass (`cargo nextest run --all --no-capture`)
- [ ] GUI compiles and runs without errors
- [ ] No clippy warnings introduced
- [ ] Public API fully preserved (no breaking changes)
- [ ] Code formatted (`cargo fmt --all`)
- [ ] Deprecation warnings added to legacy methods
- [ ] Updated metrics show improvement

---

## Rollback Plan

If issues arise during refactoring:
1. Git stash changes: `git stash`
2. Run tests on clean state
3. Review diff: `git diff stash@{0}`
4. Apply changes incrementally per phase

---

## Post-Refactoring Tasks

1. **Update Documentation:**
   - Add module-level docs to new files
   - Update `docs/architecture.md` with new structure
   
2. **Deprecation Timeline:**
   - Add issue: "Migrate scanner to repository pattern"
   - Add issue: "Remove legacy path-based CRUD methods (v0.3.0)"

3. **Next Targets:**
   - `abop-gui/src/styling/tokens/mod.rs` (616 lines)
   - `abop-gui/src/styling/theme_utils.rs` (573 lines)
   - `abop-core/src/audio/processing/mod.rs` (542 lines)

---

## Usage for New Chat Session

**Copy-paste this prompt:**

> I need to refactor `abop-core/src/db/mod.rs` (621 lines → <150 lines target). 
> 
> **Context:** See `docs/refactoring/db-mod-refactor-plan.md` for complete analysis.
> 
> **Task:** Implement the 6-phase extraction plan:
> 1. Extract schema initialization → `schema.rs`
> 2. Extract app database utilities → `app_database.rs` (GUI critical)
> 3. Extract legacy CRUD → `legacy_crud.rs` (add deprecation warnings)
> 4. Extract convenience wrappers → `convenience.rs`
> 5. Create facade → `facade.rs` with core struct
> 6. Update `mod.rs` to thin re-export layer
> 
> **Critical:** Preserve these GUI-used methods:
> - `Database::open_app_database()`
> - `Database::library_repository()`
> - `Database::audiobook_repository()`
> - `Database::add_library_with_path(name, path)`
> 
> **Validation:** Run `cargo nextest run --all --no-capture` after each phase. Check final metrics with `pwsh scripts/metrics_long_rust_files.ps1`.
> 
> Please proceed with Phase 1 (schema extraction) and wait for confirmation before continuing.

---

## Notes

- All existing submodules (connection, operations, repositories, etc.) remain unchanged
- This refactoring is purely organizational - no logic changes
- Low risk due to minimal external coupling (1 consumer)
- Follows established pattern from `error/macros.rs` refactoring (653 → 9 lines)
