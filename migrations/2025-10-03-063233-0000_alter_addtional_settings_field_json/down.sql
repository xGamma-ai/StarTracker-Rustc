-- This file should undo anything in `up.sql`
ALTER TABLE user_settings
DROP COLUMN IF EXISTS addtional_settings;