-- This file should undo anything in `up.sql`
ALTER TABLE password_manager
DROP COLUMN IF EXISTS salt;

ALTER TABLE password_manager
ALTER COLUMN password_hash TYPE VARCHAR(255) USING convert_from(password_hash, 'UTF8');