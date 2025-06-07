-- filepath: c:\Users\pshap\coding\abop\abop-core\src\db\migrations\001_initial_schema.sql
-- Initial database schema for ABOP

-- Libraries table
CREATE TABLE libraries (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Audiobooks table
CREATE TABLE audiobooks (
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
CREATE TABLE progress (
    id TEXT PRIMARY KEY,
    audiobook_id TEXT NOT NULL UNIQUE,
    position_seconds INTEGER NOT NULL DEFAULT 0,
    completed BOOLEAN NOT NULL DEFAULT 0,
    last_played TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (audiobook_id) REFERENCES audiobooks(id) ON DELETE CASCADE
);

-- Create indexes for better query performance
CREATE INDEX idx_audiobooks_library_id ON audiobooks(library_id);
CREATE INDEX idx_audiobooks_path ON audiobooks(path);
CREATE INDEX idx_progress_audiobook_id ON progress(audiobook_id);

-- Trigger to update the updated_at timestamp on audiobooks update
CREATE TRIGGER update_audiobooks_timestamp
AFTER UPDATE ON audiobooks
BEGIN
    UPDATE audiobooks SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- Seed a default library for CLI operations
INSERT INTO libraries (id, name, path) VALUES ('1', 'Default Library', '/default');
