-- Rollback for initial schema migration
-- This removes all tables and structures created in 001_initial_schema.sql

-- Drop triggers first
DROP TRIGGER IF EXISTS update_audiobooks_timestamp;
DROP TRIGGER IF EXISTS update_progress_timestamp;

-- Drop indexes
DROP INDEX IF EXISTS idx_audiobooks_library_id;
DROP INDEX IF EXISTS idx_audiobooks_path;
DROP INDEX IF EXISTS idx_progress_audiobook_id;

-- Drop tables in reverse dependency order
DROP TABLE IF EXISTS progress;
DROP TABLE IF EXISTS audiobooks;
DROP TABLE IF EXISTS libraries;
