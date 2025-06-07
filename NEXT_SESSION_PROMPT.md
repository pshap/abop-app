# ABOP Database Migration Completion - Session Continuation

## CURRENT STATE
I'm converting a Rust audiobook management system (ABOP) from deadpool-sqlite to r2d2_sqlite for database connection pooling. The project has **47 compilation errors** that need to be fixed systematically.

## COMPLETED TASKS
✅ **Dependency Updates**: Removed deadpool-sqlite, confirmed r2d2_sqlite dependencies
✅ **Database Module**: Complete r2d2-based Database implementation (430 lines)
✅ **Progress Reporter**: Fixed trait compatibility for dynamic dispatch
✅ **Basic Import Fixes**: Fixed AudioMetadata and ScanError imports
✅ **Error Enum**: Added missing TaskJoin variant to AppError
✅ **DateTime Helper Module**: Created `datetime_serde.rs` with custom serialization helpers

## CRITICAL REMAINING ISSUES (47 errors)

### 1. **DateTime Serialization (24+ errors)**
- `DateTime<Utc>` doesn't implement `ToSql`/`FromSql` traits
- Affects all repository operations in:
  - `abop-core/src/db/repositories/audiobook.rs`
  - `abop-core/src/db/repositories/progress.rs`
- **Solution Started**: Created `datetime_serde.rs` helper module with conversion functions
- **Next**: Update all repository functions to use `datetime_to_sql()` and `datetime_from_sql()`

### 2. **Database Transaction Issues (2 errors)**
- Mutable borrow errors in bulk insert operations
- `cannot borrow conn as mutable` in `abop-core/src/db/mod.rs:267`
- `cannot move out of tx` in transaction commit

### 3. **Missing Trait Imports (2 errors)**
- `FileDiscoverer` and `FileProcessor` traits not in scope in `library_scanner.rs`
- Need to add: `use crate::scanner::file_discovery::FileDiscoverer;`
- Need to add: `use crate::scanner::file_processor::FileProcessor;`

### 4. **Async/Await Errors (1 error)**
- `AudioMetadata::from_file()` is not async but being awaited in `file_processor.rs:142`
- Remove `.await` from the call

### 5. **Type Mismatches (4 errors)**
- `ScanError` vs `AppError` inconsistencies in scanner modules
- `CancellationToken` vs `Arc<CancellationToken>` mismatches in task_manager.rs

### 6. **Missing Struct Fields (2 errors)**
- `cancel_token` field missing from LibraryScanner struct
- Referenced in library_scanner.rs but not defined

### 7. **Lifetime Issues (1 error)**
- Borrowed data escaping method scope in library_scanner.rs:257
- Related to Task::perform with async move closure

### 8. **Generic Trait Bounds (1 error)**
- Missing `ProgressReporter` bound on generic parameter `P` in file_processor.rs:261

## IMMEDIATE NEXT STEPS

1. **Fix DateTime Issues (Priority 1)**:
   ```rust
   // Update all repository functions to use:
   &datetime_to_sql(&audiobook.created_at)  // for inserts
   datetime_from_sql(&row.get::<_, String>(10)?)  // for queries
   ```

2. **Fix Transaction Mutability**:
   ```rust
   let mut conn = self.pool.get().map_err(...)?;  // Add 'mut'
   ```

3. **Add Missing Imports**:
   ```rust
   use crate::scanner::{
       file_discovery::FileDiscoverer,
       file_processor::FileProcessor,
   };
   ```

4. **Fix Async Call**:
   ```rust
   let metadata = AudioMetadata::from_file(path)?;  // Remove .await
   ```

5. **Add Missing Field**:
   ```rust
   pub struct LibraryScanner {
       // ...existing fields...
       cancel_token: CancellationToken,
   }
   ```

## PROJECT CONTEXT
- **Language**: Rust with tokio async runtime
- **Database**: SQLite with r2d2 connection pooling
- **Architecture**: Modular with scanner, database, and UI layers
- **Dependencies**: rusqlite, r2d2, chrono, tokio, serde

## COMPILATION COMMAND
```powershell
cd c:\Users\pshap\coding\abop && cargo check
```

## FILES TO FOCUS ON
1. `abop-core/src/db/repositories/audiobook.rs` (DateTime serialization)
2. `abop-core/src/db/repositories/progress.rs` (DateTime serialization)  
3. `abop-core/src/db/mod.rs` (transaction mutability)
4. `abop-core/src/scanner/library_scanner.rs` (imports, fields, lifetimes)
5. `abop-core/src/scanner/file_processor.rs` (async, trait bounds)
6. `abop-core/src/scanner/task_manager.rs` (type mismatches)

## SUCCESS CRITERIA
- All 47 compilation errors resolved
- Clean `cargo check` output with only warnings
- Maintain existing functionality and API compatibility
- Follow Rust best practices for error handling and async code

Please start by systematically fixing the DateTime serialization issues across all repository functions, then move to the other critical errors in priority order.
