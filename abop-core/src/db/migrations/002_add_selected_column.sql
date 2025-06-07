-- Add selected column to audiobooks table
ALTER TABLE audiobooks ADD COLUMN selected BOOLEAN NOT NULL DEFAULT 0; 