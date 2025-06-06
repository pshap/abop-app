-- Up
CREATE TABLE IF NOT EXISTS audiobooks (
    path TEXT PRIMARY KEY,
    title TEXT,
    author TEXT,
    duration INTEGER,
    file_size INTEGER,
    last_modified INTEGER,
    metadata TEXT
);

CREATE INDEX IF NOT EXISTS idx_audiobooks_author ON audiobooks(author);
CREATE INDEX IF NOT EXISTS idx_audiobooks_title ON audiobooks(title);
CREATE INDEX IF NOT EXISTS idx_audiobooks_last_modified ON audiobooks(last_modified);

-- Down
DROP INDEX IF EXISTS idx_audiobooks_author;
DROP INDEX IF EXISTS idx_audiobooks_title;
DROP INDEX IF EXISTS idx_audiobooks_last_modified;
DROP TABLE IF EXISTS audiobooks; 