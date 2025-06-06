-- Rollback for bookmarks table migration
-- Drops bookmarks table and related indexes

DROP INDEX IF EXISTS idx_bookmarks_position;
DROP INDEX IF EXISTS idx_bookmarks_audiobook_id;
DROP TABLE IF EXISTS bookmarks;
