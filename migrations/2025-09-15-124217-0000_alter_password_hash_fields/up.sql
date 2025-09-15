-- Your SQL goes here
-- Convert existing password_hash (stored as text) into bytea using UTF-8 bytes
ALTER TABLE password_manager
ALTER COLUMN password_hash TYPE BYTEA USING convert_to(password_hash, 'UTF8');

-- Reintroduce salt column. Make it nullable to avoid failures on existing rows; application can backfill and enforce NOT NULL later.
ALTER TABLE password_manager
ADD COLUMN IF NOT EXISTS salt TEXT NOT NULL;