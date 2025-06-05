-- Add bookmarks support

CREATE TABLE IF NOT EXISTS bookmarks (
    id TEXT PRIMARY KEY,
    audiobook_id TEXT NOT NULL,
    name TEXT NOT NULL,
    position_seconds INTEGER NOT NULL,
    note TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (audiobook_id) REFERENCES audiobooks(id) ON DELETE CASCADE
);

-- Create indexes for bookmarks
CREATE INDEX IF NOT EXISTS idx_bookmarks_audiobook_id ON bookmarks(audiobook_id);

-- Trigger to update the updated_at timestamp on bookmarks update
CREATE TRIGGER IF NOT EXISTS update_bookmarks_timestamp
AFTER UPDATE ON bookmarks
BEGIN
    UPDATE bookmarks SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
