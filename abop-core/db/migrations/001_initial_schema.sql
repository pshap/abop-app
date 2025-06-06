-- Initial database schema for ABOP
-- Creates tables for libraries, audiobooks, and progress tracking

-- Libraries table
CREATE TABLE IF NOT EXISTS libraries (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    path TEXT NOT NULL
);

-- Audiobooks table
CREATE TABLE IF NOT EXISTS audiobooks (
    id TEXT PRIMARY KEY,
    library_id TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE,
    title TEXT,
    author TEXT,
    narrator TEXT,
    description TEXT,
    duration_seconds INTEGER,
    size_bytes INTEGER,
    cover_art BLOB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (library_id) REFERENCES libraries(id) ON DELETE CASCADE
);

-- Progress tracking table
CREATE TABLE IF NOT EXISTS progress (
    id TEXT PRIMARY KEY,
    audiobook_id TEXT NOT NULL,
    position_seconds REAL NOT NULL DEFAULT 0.0,
    completed INTEGER NOT NULL DEFAULT 0,
    last_played TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (audiobook_id) REFERENCES audiobooks(id) ON DELETE CASCADE
);

-- Indexes for better performance
CREATE INDEX IF NOT EXISTS idx_audiobooks_library_id ON audiobooks(library_id);
CREATE INDEX IF NOT EXISTS idx_audiobooks_author ON audiobooks(author);
CREATE INDEX IF NOT EXISTS idx_audiobooks_title ON audiobooks(title);
CREATE INDEX IF NOT EXISTS idx_progress_audiobook_id ON progress(audiobook_id);
CREATE INDEX IF NOT EXISTS idx_libraries_name ON libraries(name);
