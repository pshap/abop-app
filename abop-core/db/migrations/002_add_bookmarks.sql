-- Add bookmarks table for user bookmarks
-- Allows users to save specific positions in audiobooks with notes

CREATE TABLE IF NOT EXISTS bookmarks (
    id TEXT PRIMARY KEY,
    audiobook_id TEXT NOT NULL,
    position_seconds REAL NOT NULL,
    title TEXT,
    note TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (audiobook_id) REFERENCES audiobooks(id) ON DELETE CASCADE
);

-- Indexes for bookmarks
CREATE INDEX IF NOT EXISTS idx_bookmarks_audiobook_id ON bookmarks(audiobook_id);
CREATE INDEX IF NOT EXISTS idx_bookmarks_position ON bookmarks(position_seconds);
