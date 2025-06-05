-- Rollback for bookmarks migration
-- This removes the bookmarks table and related structures

-- Drop trigger
DROP TRIGGER IF EXISTS update_bookmarks_timestamp;

-- Drop index
DROP INDEX IF EXISTS idx_bookmarks_audiobook_id;

-- Drop table
DROP TABLE IF EXISTS bookmarks;
