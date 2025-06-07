-- filepath: c:\Users\pshap\coding\abop\abop-core\src\db\migrations\001_initial_schema_down.sql
-- Rollback initial database schema

DROP TRIGGER IF EXISTS update_audiobooks_timestamp;
DROP INDEX IF EXISTS idx_progress_audiobook_id;
DROP INDEX IF EXISTS idx_audiobooks_path;
DROP INDEX IF EXISTS idx_audiobooks_library_id;
DROP TABLE IF EXISTS progress;
DROP TABLE IF EXISTS audiobooks;
DROP TABLE IF EXISTS libraries;
