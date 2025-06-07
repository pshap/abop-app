# ABOP Database Migration - Session Continuation

## PROJECT OVERVIEW
ABOP (Audiobook Player) is a Rust application for managing and playing audiobooks. We're in the process of migrating the database layer from deadpool-sqlite to r2d2_sqlite for connection pooling. The core functionality is working, but there are several compilation errors to resolve.

## CURRENT STATUS
- **Core Database**: Migrated to r2d2_sqlite with connection pooling
- **DateTime Handling**: Partially implemented with `datetime_serde.rs`
- **Remaining Issues**: 47 compilation errors across multiple files

## COMPLETED TASKS
✅ **Dependency Updates**: Removed deadpool-sqlite, added r2d2_sqlite
✅ **Database Module**: Implemented r2d2-based Database struct (430 lines)
✅ **Progress Reporter**: Fixed trait compatibility for dynamic dispatch
✅ **Basic Import Fixes**: Fixed AudioMetadata and ScanError imports
✅ **Error Enum**: Added missing TaskJoin variant to AppError
✅ **DateTime Helpers**: Created `datetime_serde.rs` with serialization utilities

## CRITICAL REMAINING ISSUES (47 errors)

### 1. DateTime Serialization (24+ errors)
- **Files Affected**:
  - `abop-core/src/db/repositories/audiobook.rs`
  - `abop-core/src/db/repositories/progress.rs`
- **Current Implementation**:
  - `datetime_serde.rs` provides `SqliteDateTime` wrapper and helper functions
  - Includes `datetime_to_sql()`, `datetime_from_sql()`, and related utilities
- **Required Changes**:
  - Update repository methods to use the new serialization helpers
  - Ensure consistent error handling with `DateTimeError`

### 2. Database Transaction Issues (2 errors)
- **Location**: `abop-core/src/db/mod.rs` (lines ~267)
- **Issues**:
  - Mutable borrow errors in bulk insert operations
  - `cannot borrow conn as mutable`
  - `cannot move out of tx` in transaction commit
- **Solution**:
  - Ensure proper transaction handling with `?` operator
  - Use `transaction::<_, _, AppError>()` pattern consistently

### 3. Missing Trait Imports (2 errors)
- **Files**: `abop-core/src/scanner/library_scanner.rs`
- **Missing Imports**:
  ```rust
  use crate::scanner::file_discovery::FileDiscoverer;
  use crate::scanner::file_processor::FileProcessor;
  ```

### 4. Async/Await Errors (1 error)
- **File**: `file_processor.rs` (line 142)
- **Issue**: `AudioMetadata::from_file()` is not async but being awaited
- **Fix**: Remove `.await` from the call

### 5. Type Mismatches (4 errors)
- **Issues**:
  - `ScanError` vs `AppError` inconsistencies
  - `CancellationToken` vs `Arc<CancellationToken>` mismatches in `task_manager.rs`
- **Fix**: Ensure consistent error types and proper Arc wrapping

### 6. Missing Struct Fields (2 errors)
- **File**: `library_scanner.rs`
- **Issue**: `cancel_token` field referenced but not defined
- **Fix**: Add field to struct:
  ```rust
  pub struct LibraryScanner {
      // ...
      cancel_token: CancellationToken,
  }
  ```

### 7. Lifetime Issues (1 error)
- **File**: `library_scanner.rs` (line 257)
- **Issue**: Borrowed data escaping method scope
- **Context**: Related to `Task::perform` with async move closure

### 8. Generic Trait Bounds (1 error)
- **File**: `file_processor.rs` (line 261)
- **Issue**: Missing `ProgressReporter` bound on generic parameter `P`

## IMPLEMENTATION DETAILS

### DateTime Serialization Helpers
```rust
// Conversion functions available in datetime_serde.rs
pub fn datetime_to_sql(dt: &DateTime<Utc>) -> String;
pub fn datetime_from_sql(s: &str) -> Result<DateTime<Utc>, DateTimeError>;
pub fn datetime_to_sql_output(dt: &DateTime<Utc>) -> ToSqlOutput<'_>;
pub fn optional_datetime_to_sql_output(dt: &Option<DateTime<Utc>>) -> ToSqlOutput<'_>;
```

### Example Repository Update
```rust
// Before
row.get::<_, DateTime<Utc>>(10)?;

// After
row.get::<_, String>(10)
    .and_then(|s| datetime_from_sql(&s).map_err(rusqlite::Error::from))?;
```

## VALIDATION COMMANDS
```powershell
# Run checks
cd c:\Users\pshap\coding\abop
cargo check

# Run tests
cargo test

# Check for dead code
cargo clippy -- -D warnings
```

## FILES TO MODIFY (Priority Order)
1. `abop-core/src/db/repositories/audiobook.rs` - DateTime serialization
2. `abop-core/src/db/repositories/progress.rs` - DateTime serialization  
3. `abop-core/src/db/mod.rs` - Transaction handling
4. `abop-core/src/scanner/library_scanner.rs` - Imports, fields, lifetimes
5. `abop-core/src/scanner/file_processor.rs` - Async, trait bounds
6. `abop-core/src/scanner/task_manager.rs` - Type mismatches

## SUCCESS CRITERIA
- All 47 compilation errors resolved
- Clean `cargo check` output with only warnings
- Maintain existing functionality and API compatibility
- Follow Rust best practices for error handling and async code

## NEXT STEPS
1. Fix DateTime serialization in repository methods
2. Resolve transaction mutability issues
3. Add missing imports and fix struct definitions
4. Address async/await and type mismatch issues
5. Fix lifetime and trait bound issues
6. Run full test suite to verify functionality
