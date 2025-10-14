<!-- 3a304fdd-a72c-4d89-952e-93408cac12ee 9f1413dd-6ecd-40e9-a307-92b8e67c9097 -->
# ABOP Completion Plan

## Executive Summary

**Current State**: The project is ~80% complete with a solid foundation. Core audio processing, database layer, GUI framework, and basic functionality work well. The application compiles successfully with no errors (Rust 2024, Iced 0.13.1).

**What's Missing**: Cover art display, chapter navigation, search functionality, batch processing automation, and CLI completion.

**Timeline**: 6 weeks across 3 phases

## Current Implementation Status

### ✅ What Works (80% Complete)

- **Audio Processing Pipeline**: Resampling, channel mixing, normalization, silence detection
- **Library Scanner**: Multi-threaded directory traversal with metadata extraction
- **Database Layer**: SQLite with connection pooling, progress tracking schema complete
- **GUI Foundation**: Iced-based interface with Material Design 3 theming
- **Audio Playback**: Basic player with play/pause/stop/next/previous
- **Configuration System**: Type-safe settings with validation
- **Format Support**: MP3, M4A, M4B, FLAC, OGG, WAV, AAC

### ❌ What's Missing (20% Remaining)

- **Cover Art Display**: Database field exists (`audiobooks.cover_art BLOB`), no UI implementation
- **Chapter Navigation**: `Chapter` model exists, no extraction or UI
- **Search Functionality**: `SearchQuery` and `SearchResult` models exist, no search engine
- **Batch Processing GUI**: Core `BatchProcessor` exists, no GUI automation
- **CLI Completion**: Basic structure exists, missing key commands
- **Playback Position UI**: Progress tracking DB schema exists, no UI integration

## Implementation Plan

### Phase 1: Critical Path Items (Weeks 1-2)

*Enable basic daily use of the application*

#### Task 1.1: Cover Art Display System

**Priority**: High | **Complexity**: Medium | **Time**: 3 days

**What to Build**:

- Cover art component with thumbnail display
- Integration with library table view
- Placeholder for missing cover art
- Image caching system

**Files to Create**:

- `abop-gui/src/components/cover_art.rs`
- `abop-gui/src/utils/image_cache.rs`

**Files to Modify**:

- `abop-gui/src/components/table_row.rs` - Add cover art column
- `abop-gui/src/views/library.rs` - Integrate cover art display
- `abop-gui/Cargo.toml` - Add `image = "0.24"` dependency

**Technical Approach**:

- Use `iced::widget::image` with `Handle::from_memory`
- Generate 64x64 thumbnails for table view
- Cache decoded images in memory (LRU cache, max 100 items)
- Use Material Design placeholder icon for missing art

#### Task 1.2: Search Functionality

**Priority**: High | **Complexity**: Medium | **Time**: 4 days

**What to Build**:

- Simple in-memory search engine with fuzzy matching
- Search bar component in library view
- Filter by title, author, narrator
- Real-time search results

**Files to Create**:

- `abop-core/src/search/engine.rs`
- `abop-core/src/search/mod.rs`
- `abop-gui/src/components/search_bar.rs`

**Files to Modify**:

- `abop-gui/src/messages.rs` - Add `SearchQuery(String)` message
- `abop-gui/src/views/library.rs` - Add search bar and filter results
- `abop-gui/src/state_refactored/library_state.rs` - Add search state
- `Cargo.toml` - Add `fuzzy-matcher = "0.3"`

**Technical Approach**:

- Use `fuzzy-matcher` for fuzzy string matching
- Search across title, author, narrator fields
- Score results by relevance
- Update filtered audiobooks list reactively

#### Task 1.3: Playback Position Tracking UI

**Priority**: High | **Complexity**: Simple | **Time**: 2 days

**What to Build**:

- Progress bar showing current position
- Time display (current / total)
- Seek functionality via progress bar click
- Auto-save position to database

**Files to Modify**:

- `abop-gui/src/components/audio_controls.rs` - Add progress bar
- `abop-core/src/audio/player.rs` - Add `get_position()` and `seek()`
- `abop-gui/src/audio/player.rs` - Expose position tracking
- `abop-gui/src/handlers/ui_state.rs` - Handle seek message

**Technical Approach**:

- Poll player position every 500ms via subscription
- Use `iced::widget::slider` for seekable progress bar
- Persist position to `progress` table on stop/pause
- Resume from saved position on play

### Phase 2: Core Features (Weeks 3-4)

*Complete essential audiobook management features*

#### Task 2.1: Chapter Navigation System

**Priority**: Medium | **Complexity**: Complex | **Time**: 5 days

**What to Build**:

- Chapter extraction from MP3 ID3v2, M4B, and FLAC
- Chapter list UI component
- Chapter-based seeking
- Chapter storage in database

**Files to Create**:

- `abop-core/src/audio/chapter_extractor.rs`
- `abop-core/src/db/migrations/002_chapters.sql`
- `abop-core/src/db/repositories/chapter.rs`
- `abop-gui/src/components/chapter_list.rs`
- `abop-gui/src/views/player.rs`

**Files to Modify**:

- `abop-core/src/audio/metadata.rs` - Extract chapter metadata
- `abop-gui/src/router.rs` - Add `Player` route
- `abop-gui/src/messages.rs` - Add chapter navigation messages

**Technical Approach**:

- Extract chapters from ID3v2 CHAP frames (MP3)
- Extract chapters from MP4 `chpl` atom (M4B)
- Extract chapters from FLAC `CUESHEET` blocks
- Store in new `chapters` table with foreign key to audiobook
- Display chapter list in player view with click-to-seek

**Database Migration**:

```sql
CREATE TABLE chapters (
    id TEXT PRIMARY KEY,
    audiobook_id TEXT NOT NULL,
    title TEXT NOT NULL,
    start_time INTEGER NOT NULL,
    end_time INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (audiobook_id) REFERENCES audiobooks(id) ON DELETE CASCADE
);
CREATE INDEX idx_chapters_audiobook_id ON chapters(audiobook_id);
```

#### Task 2.2: Batch Processing Automation

**Priority**: Medium | **Complexity**: Medium | **Time**: 4 days

**What to Build**:

- Batch operation dialog with operation selection
- Progress tracking for batch operations
- Queue management for multiple operations
- Operation templates (convert to mono, normalize, etc.)

**Files to Create**:

- `abop-gui/src/components/batch_dialog.rs`
- `abop-gui/src/commands/batch_processing.rs`

**Files to Modify**:

- `abop-gui/src/views/library.rs` - Add batch processing button
- `abop-gui/src/messages.rs` - Add batch operation messages
- `abop-core/src/audio/processing/batch_processor.rs` - Expose to GUI

**Technical Approach**:

- Leverage existing `BatchProcessor` from abop-core
- Create dialog for operation selection and settings
- Show progress with file count and percentage
- Allow cancellation of running operations

#### Task 2.3: Enhanced Library Management

**Priority**: Medium | **Complexity**: Medium | **Time**: 3 days

**What to Build**:

- Library statistics dashboard
- Bulk operations (delete, tag editing)
- Library export/import (JSON format)
- Duplicate detection

**Files to Create**:

- `abop-gui/src/components/library_stats.rs`
- `abop-gui/src/commands/library_export.rs`

**Files to Modify**:

- `abop-gui/src/views/library.rs` - Add stats panel
- `abop-gui/src/messages.rs` - Add bulk operation messages
- `abop-core/src/db/repositories/audiobook.rs` - Add bulk operations

**Technical Approach**:

- Display total audiobooks, total duration, total size
- Multi-select for bulk delete with confirmation
- Export library as JSON for backup
- Hash-based duplicate detection by file content

### Phase 3: Polish and Refinement (Weeks 5-6)

*Production-ready features and optimization*

#### Task 3.1: CLI Completion

**Priority**: Low | **Complexity**: Medium | **Time**: 4 days

**What to Build**:

- Complete scan command with progress
- Add process command for batch operations
- Add export command for library data
- Add config commands for settings management

**Files to Modify**:

- `abop-cli/src/commands/scan.rs` - Complete implementation
- `abop-cli/src/commands/process.rs` - Create new
- `abop-cli/src/commands/export.rs` - Create new
- `abop-cli/src/commands/config.rs` - Create new

**Technical Approach**:

- Use `indicatif` for progress bars
- Leverage abop-core functionality
- JSON output mode for scripting
- Support piping and standard Unix patterns

#### Task 3.2: Advanced Player Features

**Priority**: Low | **Complexity**: Medium | **Time**: 3 days

**What to Build**:

- Playback queue/playlist
- Shuffle and repeat modes
- Speed control (0.5x - 2.0x)
- Sleep timer

**Files to Create**:

- `abop-core/src/models/playlist.rs`
- `abop-gui/src/components/queue.rs`

**Files to Modify**:

- `abop-core/src/audio/player.rs` - Add speed control
- `abop-gui/src/state_refactored/player_state.rs` - Add queue state
- `abop-gui/src/components/audio_controls.rs` - Add speed/repeat controls

**Technical Approach**:

- Queue as `Vec<String>` of audiobook IDs
- Speed control via rodio sink
- Repeat: none, one, all
- Sleep timer via tokio delayed task

#### Task 3.3: Performance Optimization

**Priority**: Medium | **Complexity**: Medium | **Time**: 3 days

**What to Build**:

- Progress text caching (already stubbed)
- Virtual scrolling for large libraries
- Image caching improvements
- Database query optimization

**Files to Modify**:

- `abop-gui/src/state_refactored/progress_cache.rs` - Implement caching
- `abop-gui/src/components/table_core.rs` - Add virtual scrolling
- `abop-gui/src/utils/image_cache.rs` - Optimize cache
- `abop-core/src/db/repositories/audiobook.rs` - Add indexes and query optimization

**Technical Approach**:

- Cache formatted progress strings (LRU, 1000 entries)
- Virtual scrolling: render only visible rows + 20 buffer
- Image cache: 100 max, evict LRU
- Add database indexes for common queries

## Dependencies to Add

```toml
[workspace.dependencies]
# Phase 1
image = "0.24"
fuzzy-matcher = "0.3"

# Phase 3
indicatif = "0.17"
```

## Success Criteria

### Phase 1 Complete When:

- Cover art displays in library table view
- Search bar filters audiobooks in real-time
- Playback shows current position with seek capability
- All features tested and working

### Phase 2 Complete When:

- Chapters extracted and navigable for MP3/M4B/FLAC
- Batch processing dialog can process multiple files
- Library statistics display correctly
- Export/import functionality works

### Phase 3 Complete When:

- CLI provides scan, process, export commands
- Playback queue and speed control work
- Application handles 1000+ audiobooks smoothly
- All TODO comments resolved

## Risk Mitigation

### High Risk: Chapter Extraction Complexity

- **Risk**: Different formats have varying chapter support
- **Mitigation**: Start with MP3 ID3v2, add others incrementally
- **Fallback**: Manual chapter markers if extraction fails

### Medium Risk: Search Performance

- **Risk**: Fuzzy search may be slow for large libraries
- **Mitigation**: Implement incremental search with debouncing
- **Fallback**: Exact match search as alternative

### Low Risk: Image Memory Usage

- **Risk**: Cover art may consume excessive memory
- **Mitigation**: Strict LRU cache limits, thumbnail generation
- **Fallback**: On-demand loading without caching

## Testing Strategy

- **Unit Tests**: Add for new search engine, chapter extractor
- **Integration Tests**: Cover art loading, batch processing
- **Manual Testing**: Test with real audiobook library (100+ books)
- **Performance Testing**: Verify 1000+ audiobook library performance

## Notes

- Maintain Rust 2024 and Iced 0.13.1 compatibility (per user rules)
- Follow existing Material Design 3 patterns
- Keep module size under 300 lines where possible
- Use existing error handling patterns
- Maintain comprehensive logging with tracing

### To-dos

- [ ] Implement cover art display system with thumbnails and caching
- [ ] Build search functionality with fuzzy matching and real-time filtering
- [ ] Add playback position tracking UI with progress bar and seek
- [ ] Implement chapter navigation system with extraction and UI
- [ ] Create batch processing automation with GUI dialog
- [ ] Enhance library management with stats and bulk operations
- [ ] Complete CLI implementation with all core co
- [ ] Add advanced player features (queue, speed, sleep timer)
- [ ] Optimize performance for large libraries with caching and virtual scrolling