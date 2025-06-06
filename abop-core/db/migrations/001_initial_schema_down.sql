-- Rollback for initial schema migration
-- Drops all tables and indexes created in 001_initial_schema.sql

-- Drop indexes first
DROP INDEX IF EXISTS idx_libraries_name;
DROP INDEX IF EXISTS idx_progress_audiobook_id;
DROP INDEX IF EXISTS idx_audiobooks_title;
DROP INDEX IF EXISTS idx_audiobooks_author;
DROP INDEX IF EXISTS idx_audiobooks_library_id;

-- Drop tables in reverse dependency order
DROP TABLE IF EXISTS progress;
DROP TABLE IF EXISTS audiobooks;
DROP TABLE IF EXISTS libraries;
